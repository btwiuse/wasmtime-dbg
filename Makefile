zigbuild:
	cargo zigbuild --release --target x86_64-unknown-linux-musl
	cargo zigbuild --release --target aarch64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/wasmtime-dbg wasmtime-linux-x86_64-dbg
	cp target/aarch64-unknown-linux-musl/release/wasmtime-dbg wasmtime-linux-aarch64-dbg
	cp target/x86_64-unknown-linux-musl/release/mmap-rs mmap-rs.x86_64
	cp target/aarch64-unknown-linux-musl/release/mmap-rs mmap-rs.aarch64

zigcc:
	zig cc -target aarch64-linux-musl --static -o mmap.aarch64 mmap.c
	zig cc -target x86_64-linux-musl --static -o mmap.x86_64 mmap.c

strace:
	strace ./wasmtime-linux-x86_64-dbg &> wasmtime-linux-x86_64-dbg.strace.txt
