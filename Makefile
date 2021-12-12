CARGO = cargo
RUN_DAY ?= $(shell date +"%e")
RUN_DAYS=$(shell seq 1 $(RUN_DAY))
BM_DAYS=$(shell seq -s ',' 0 $(RUN_DAY) | sed 's/,*$$//g')
SOURCE_DIR = $(PWD)
STATIC_TMP_DIR = ~/tmp/aoc_build
BM_RUNS ?= 50

PROFILE ?= debug

ifeq ($(OS),Windows_NT)
	AOC_DEBUG=target\debug\advent-of-code-2021.exe
	AOC_RELEASE=target\release\advent-of-code-2021.exe
	AOC_RUN=target\$(PROFILE)\advent-of-code-2021.exe
	AOC_CMD=tmp\aoc.win.exe
	LS =
else
	AOC_DEBUG=target/debug/advent-of-code-2021
	AOC_RELEASE=target/release/advent-of-code-2021
	AOC_RUN=target/$(PROFILE)/advent-of-code-2021
	AOC_CMD=tmp/aoc.linux
	RUST_FLAGS = RUSTFLAGS='-C link-arg=-s'
	LS = ls -ahlF $(AOC_RELEASE) $(AOC_DEBUG)
	TIME = time
endif

default: run

build: build.dbg build.release
	@$(LS)

build.dbg:
	$(CARGO) build

build.release:
	$(RUST_FLAGS) $(CARGO) build --release

build.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) build

run: build run_

run_:
	$(AOC_RUN) $(RUN_DAY) 1 -t
	$(AOC_RUN) $(RUN_DAY) 1
	$(AOC_RUN) $(RUN_DAY) 2 -t
	$(AOC_RUN) $(RUN_DAY) 2

run.all: build
	@for d in $(RUN_DAYS); do $(MAKE) run_ RUN_DAY=$$d; done

run.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) run PROFILE=$(PROFILE)

run.dbg.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) build.dbg run_

run.all.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) run.all

test:
	$(CARGO) test -- --nocapture

test.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) test


# benchmarking

benchmark:
	RUSTFLAGS='-C link-arg=-s' $(CARGO) build --release
	cp $(AOC_RELEASE) $(AOC_CMD)
	hyperfine \
		--ignore-failure\
		--export-markdown tmp/benchmark.md \
		--warmup 10 --runs $(BM_RUNS) \
		-L parts "1,2" -L days "$(BM_DAYS)" \
		"$(AOC_CMD) {days} {parts}"
	cat tmp/benchmark.md

# we copy into a WSL-only folder ($HOME/tmp),
# so the bad WSL2 filesystem sync doesn't block us (almost 40ms extra time)
benchmark.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && $(MAKE) benchmark BM_RUNS=100

# ENABLE in Cargo.toml: debug = true
prof.wsl: wsl.sync
	cd $(STATIC_TMP_DIR) && \
	$(CARGO) build --release && \
	(cargo flamegraph --verbose -c "record -F max --call-graph dwarf -g" --bin advent-of-code-2021 -o day-5-2.svg -- 5 2 || true) && \
	ls -ahlF .
	cp $(STATIC_TMP_DIR)/day-5-2.svg tmp/

wsl.sync:
	mkdir -p $(STATIC_TMP_DIR)
	rsync -av $(SOURCE_DIR)/ $(STATIC_TMP_DIR)/ --exclude .git --exclude target

# even better performance (esp for compilation)
wsl.ramdisk:
	mkdir -p $(STATIC_TMP_DIR)
	sudo mount -t ramfs ramfs $(STATIC_TMP_DIR)
	sudo chown $(USER):$(USER) $(STATIC_TMP_DIR)

wsl.ramdisk.umount:
	sudo umount $(STATIC_TMP_DIR)

# utilities

days:
	for i in $$(seq 1 25); do \
		rm -f src/days/day$$i.rs; \
		cp src/days/day0.rs src/days/day$$i.rs; \
	done

inputs:
	for i in $$(seq 1 25); do \
		cp inputs/0.challenge.txt inputs/$$i.challenge.txt || true; \
		cp inputs/0.test.txt inputs/$$i.test.txt || true; \
	done
.PHONY: inputs
