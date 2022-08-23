.phony: all test

all:
	cargo fmt
	cargo build

test:
	cargo test

clean:
	cargo clean

x86:
	cargo build --target x86_64-unknown-linux-gnu
