default:
	@echo no default target

days:
	for i in $$(seq 1 25); do \
		rm -f src/days/day$$i.rs; \
		cp src/days/day0.rs src/days/day$$i.rs; \
	done
