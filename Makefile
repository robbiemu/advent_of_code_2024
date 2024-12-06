AOC_PATH := $(shell which aoc)
AOC_SESSION_FILE := .adventofcode.session

check-aoc:
    ifndef AOC_PATH
        $(error Couldn't find 'aoc'. Please install (ie cargo install aoc-cli) it from https://crates.io/crates/advent-of-code-cli)
    endif
    ifeq ($(wildcard $(AOC_SESSION_FILE)),)
        $(error AOC_SESSION_FILE '$(AOC_SESSION_FILE)' does not exist. Please create the file.)
    endif

define copy_and_replace
    cp template.$1 $2/$1
    sed -i '' "s/__DAY__/day_$3/g" $2/$1
endef
		
day-%: check-aoc
	cargo new $@
	rm $@/src/main.rs
	mkdir -p $@/benches
	$(call copy_and_replace,main.rs,$@/src,$*)
	$(call copy_and_replace,lib.rs,$@/src,$*)
	$(call copy_and_replace,bench.rs,$@/benches,$*)
	echo "\n[dev-dependencies]\nmry = \"^0.10\"\ndivan = \"^0.1\"\n\n[features]\nsample = []\npart2 = []" >> $@/Cargo.toml
	touch $@/sample.txt
	aoc --session-file $(AOC_SESSION_FILE) download --day $* --input-only --input-file $@/input.txt