use nothing::Probably;
use wasmtime::{Config, Engine, Error, OptLevel, Strategy, WasmBacktraceDetails};

fn main() -> Result<(), Error> {
    /*
    let config = Config {
        debug_info: false,
        parse_wasm_debuginfo: false,
        wasm_threads: false,
        wasm_reference_types: false,
        wasm_bulk_memory: false,
        wasm_simd: false,
        wasm_relaxed_simd: false,
        wasm_multi_value: false,
        static_memory_maximum_size: 4294967296,
        static_memory_guard_size: 2147483648,
        dynamic_memory_guard_size: 65536,
        guard_before_linear_memory: true,
        compiler_config: CompilerConfig {
            strategy: CompilerStrategy::Auto,
            target: None,
            settings: {
                "use_egraphs": "false",
                "opt_level": "speed_and_size",
                "enable_verifier": "false",
                "enable_nan_canonicalization": "false",
            },
            flags: {},
            cache_store: None,
        },
    }
     */
    let mut config = Config::new();

    #[allow(deprecated)]
    config
        .debug_info(false)
        .wasm_backtrace_details(WasmBacktraceDetails::Disable)
        .wasm_threads(false)
        .wasm_reference_types(false)
        .wasm_bulk_memory(false)
        .wasm_simd(false)
        .wasm_relaxed_simd(false)
        .wasm_multi_value(false)
        .static_memory_maximum_size(4294967296)
        .static_memory_guard_size(2147483648)
        .dynamic_memory_guard_size(65536)
        .guard_before_linear_memory(true)
        .cranelift_nan_canonicalization(false)
        .cranelift_use_egraphs(false)
        .strategy(Strategy::Auto)
        .cranelift_opt_level(OptLevel::SpeedAndSize);

    /* unsafe { config.cranelift_flag_set("use_egraphs", "false") } */

    // .target("None")?;
    // .cranelift_flag_set()
    /*

    .compiler_config(CompilerConfig {
        strategy: CompilerStrategy::Auto,
        target: None,
        settings: {
            "use_egraphs": "false",
            "opt_level": "speed_and_size",
            "enable_verifier": "false",
            "enable_nan_canonicalization": "false",
        },
        flags: {},
        cache_store: None,
    });
     */

    dbg!(&config);
    let engine = Engine::new(&config)?;
    // dbg!(&engine);
    Ok(())
    // println!("Hello, world!").into()
}
