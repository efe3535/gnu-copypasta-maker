prog :=gnu-copypasta-maker

release :=--release
target :=release


build:
	@echo "Building for release."
	cargo build $(release)

install:
	@echo "Installing, if it fails try make && sudo make install"
	cp target/$(target)/$(prog) /usr/bin/$(prog)

uninstall:
	@echo "Uninstalling, sad to say bye..."
	rm /usr/bin/$(prog)

help:
	@echo "usage: make $(prog) [debug=1]"
