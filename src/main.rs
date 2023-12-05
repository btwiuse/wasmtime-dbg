use nothing::Probably;
use wasmtime::{
    Config, Engine, Error, InstanceAllocationStrategy, OptLevel, PoolingAllocationConfig, Strategy,
    WasmBacktraceDetails,
};

fn main() -> Result<(), Error> {
    let mut config = Config::new();

    dbg!(&config);

    /*
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
        .strategy(Strategy::Auto)
        .cranelift_opt_level(OptLevel::SpeedAndSize);

    dbg!(&config);
    */

    let mut pooling_config = PoolingAllocationConfig::default();

    dbg!(&pooling_config);

    /*
    const MAX_WASM_PAGES: u64 = 0x10000;
    const WASM_PAGE_SIZE: u64 = 65536;

    let memory_pages = MAX_WASM_PAGES;

    pooling_config
      .max_unused_warm_slots(4)
      .total_core_instances(64)
      .total_memories(64)
      .max_memories_per_module(1)
      .total_tables(64)
      .max_tables_per_module(1)
      .max_component_instance_size(128*1024)
      .table_elements(8192)
      .memory_pages(memory_pages);
      // .max_memories_per_component(65536)
      // .max_core_instances_per_component(65536)
      // .max_tables_per_component(65536);

    dbg!(&pooling_config);
    */

    config.allocation_strategy(InstanceAllocationStrategy::Pooling(pooling_config));

    dbg!(&config);

    let engine = Engine::new(&config)?;

    Ok(())
}
