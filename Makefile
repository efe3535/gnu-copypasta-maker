prog :=gnu-copypasta-maker

release :=--release
target :=release


build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) /usr/bin/$(prog)

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"