prog :=gnu-copypasta-maker

release :=--release
target :=release


build:
	cargo build $(release)

install: build
	cp target/$(target)/$(prog) /usr/bin/$(prog)

uninstall:
	rm /usr/bin/$(prog)

help:
	@echo "usage: make $(prog) [debug=1]"
