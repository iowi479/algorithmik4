
build:
	cargo build --release

run:
	cargo run --release --bin problem1 ./dewiki-20220201-clean.txt 100000

clean:
	cargo clean
