AOC_PATH := $(shell which aoc)
AOC_SESSION_FILE := .adventofcode.session

check-aoc:
    ifndef AOC_PATH
        $(error Couldn't find 'aoc'. Please install (ie cargo install aoc-cli) it from https://crates.io/crates/advent-of-code-cli)
    endif
    ifeq ($(wildcard $(AOC_SESSION_FILE)),)
        $(error AOC_SESSION_FILE '$(AOC_SESSION_FILE)' does not exist. Please create the file.)
    endif


		
day-%: check-aoc
	cargo new $@
	rm $@/src/main.rs
	cp template.rs $@/src/main.rs
	echo "\n[dev-dependencies]\nmry = \"^0.10\"\n\n[features]\nsample = []\npart2 = []" >> $@/Cargo.toml
	touch $@/sample.txt
	aoc --session-file $(AOC_SESSION_FILE) download --day $* --input-only --input-file $@/input.txt