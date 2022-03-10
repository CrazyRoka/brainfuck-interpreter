ARCH = x86_64-unknown-linux-musl

build:
	rustup target add $(ARCH)
	cargo build --target $(ARCH) --release --target-dir ./target
	sam build

build-HelloWorldFunction:
	cp ./target/$(ARCH)/release/brainfuck_aws $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap
