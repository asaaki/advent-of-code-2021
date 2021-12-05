# -msvc not working due to some dependencies
CARGO = cargo +stable-gnu
RUN_DAY ?= $(shell date +"%e")
BM_DAYS=$(shell seq -s ',' 0 $(RUN_DAY))
SOURCE_DIR = $(PWD)
STATIC_TMP_DIR = ~/tmp/aoc_build

ifeq ($(OS),Windows_NT)
	AOC_DEBUG=target\debug\advent-of-code-2021.exe
	AOC_RELEASE=target\release\advent-of-code-2021.exe
	AOC_CMD=tmp\aoc.win.exe
else
	AOC_DEBUG=target/debug/advent-of-code-2021
	AOC_RELEASE=target/release/advent-of-code-2021
	AOC_CMD=tmp/aoc.linux
endif

default: run

run:
	$(CARGO) build
	$(AOC_DEBUG) $(RUN_DAY) 1 -t
	$(AOC_DEBUG) $(RUN_DAY) 1
	$(AOC_DEBUG) $(RUN_DAY) 2 -t
	$(AOC_DEBUG) $(RUN_DAY) 2

test:
	$(CARGO) test -- --nocapture

# benchmarking

benchmark:
	$(CARGO) build --release
	cp $(AOC_RELEASE) $(AOC_CMD)
	hyperfine \
		--export-markdown tmp/benchmark.md \
		--warmup 10 --runs 50 \
		-L parts 1,2 -L days $(BM_DAYS) \
		"$(AOC_CMD) {days} {parts}"
	cat tmp/benchmark.md

# we copy into a WSL-only folder ($HOME/tmp),
# so the bad WSL2 filesystem sync doesn't block us (almost 40ms extra time)
benchmark.wsl:
	mkdir -p $(STATIC_TMP_DIR)
	rsync -av $(SOURCE_DIR)/ $(STATIC_TMP_DIR)/ --exclude .git --exclude target
	cd $(STATIC_TMP_DIR) && $(MAKE) benchmark

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
