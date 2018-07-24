/target/release/urbandict: src/main.rs Cargo.toml
	cargo build --release

clean:
	rm -vf ./target/release/urbandict*

.PHONY: clean
