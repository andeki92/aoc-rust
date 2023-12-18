
ifeq (cli,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif

.PHONY: run
cli:
	cargo run --bin aoc-cli -- ${RUN_ARGS}

ifeq (run,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif

.PHONY: run
run:
	cargo run --bin aoc-solutions -- ${RUN_ARGS}

bench:
	cargo bench
	open ./target/criterion/report/index.html

clean:
	rm -rf ./target
