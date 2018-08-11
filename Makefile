release: src/main.rs Cargo.toml
	cargo build --release

crate:
	cargo package --allow-dirty

clean:
	rm -rfv target/ Cargo.lock

.PHONY: clean crate release
