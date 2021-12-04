default:
	@echo no default target

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
