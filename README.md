You can use precompiled static binaries to reproduce the issue:

```
$ ./wasmtime-linux-aarch64-dbg 
[main.rs:6] &config = Config {
    debug_info: false,
    parse_wasm_debuginfo: false,
    wasm_threads: true,
    wasm_reference_types: true,
    wasm_function_references: false,
    wasm_bulk_memory: true,
    wasm_simd: true,
    wasm_relaxed_simd: true,
    wasm_multi_value: true,
    static_memory_maximum_size: 4294967296,
    static_memory_guard_size: 2147483648,
    dynamic_memory_guard_size: 65536,
    guard_before_linear_memory: true,
    parallel_compilation: true,
    compiler_config: CompilerConfig {
        strategy: Auto,
        target: None,
        settings: {
            "opt_level": "speed",
            "enable_verifier": "false",
        },
        flags: {},
        cache_store: None,
        clif_dir: None,
        wmemcheck: false,
    },
}
[main.rs:10] &config = Config {
    debug_info: false,
    parse_wasm_debuginfo: false,
    wasm_threads: true,
    wasm_reference_types: true,
    wasm_function_references: false,
    wasm_bulk_memory: true,
    wasm_simd: true,
    wasm_relaxed_simd: true,
    wasm_multi_value: true,
    static_memory_maximum_size: 4294967296,
    static_memory_guard_size: 2147483648,
    dynamic_memory_guard_size: 65536,
    guard_before_linear_memory: true,
    parallel_compilation: true,
    compiler_config: CompilerConfig {
        strategy: Auto,
        target: None,
        settings: {
            "opt_level": "speed",
            "enable_verifier": "false",
        },
        flags: {},
        cache_store: None,
        clif_dir: None,
        wmemcheck: false,
    },
}
Error: failed to create memory pool mapping

Caused by:
    0: mmap failed to reserve 0x5dc80000000 bytes
    1: Out of memory (os error 12)
```
