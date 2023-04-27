main: main.rs
	rustc main.rs

.PHONY: all
all: parsed.txt parsed_rev.txt

parsed.txt: main
	./main ./data/*.txt > $@

parsed_rev.txt: main
	./main -r ./data/*.txt > $@
