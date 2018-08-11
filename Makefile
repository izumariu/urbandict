target/release/urbandict: src/main.rs Cargo.toml
	cargo build --release

clean:
	rm -rfv target/ Cargo.lock

.PHONY: clean
