main: main.rs
	rustc main.rs

.PHONY: all
all: parsed.txt parsed_rev.txt

parsed.txt: main data/*.txt
	./main ./data/*.txt > $@

parsed_rev.txt: main data/*.txt
	./main -r ./data/*.txt > $@
