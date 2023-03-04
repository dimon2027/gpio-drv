.PHONY: default all

release_raspi: build_raspi deploy_raspi

deploy_raspi:
	scp ./target/aarch64-unknown-linux-gnu/release/gpiodrv dima@10.42.0.1:/opt/gpiodrv

build_raspi:
	cargo build --release --target=aarch64-unknown-linux-gnu

build_linux:
	cargo build --release

