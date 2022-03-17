ARCH = x86_64-unknown-linux-musl
ROOT_DIR = $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

build:
	rustup target add $(ARCH)
	cargo build --target $(ARCH) --release --target-dir ../target
	sam build

build-BrainfuckFunction:
	cp $(ROOT_DIR)/../target/$(ARCH)/release/brainfuck_aws $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap
