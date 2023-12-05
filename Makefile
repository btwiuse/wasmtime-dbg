zigbuild:
	cargo zigbuild --release --target x86_64-unknown-linux-musl
	cargo zigbuild --release --target aarch64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/wasmtime-dbg wasmtime-linux-x86_64-dbg
	cp target/aarch64-unknown-linux-musl/release/wasmtime-dbg wasmtime-linux-aarch64-dbg
