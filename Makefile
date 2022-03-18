SHELL := /bin/bash

SOURCES = $(wildcard src/*.rs)

PHONY+=all
all: list-executables

list-executables: target/release/list-executables
	strip $< -o $@

target/release/list-executables: $(SOURCES)
	cargo build --release

PHONY+=clean
clean:
	cargo clean

PHONY+=run
run: all
	$(SHELL) -c $$(./list-executables | fzf --reverse --no-extended)

PHONY+=install
install: /usr/local/bin/list-executables

/usr/local/bin/list-executables: list-executables
	install --mode +x $< $@

.PHONY: $(PHONY)
