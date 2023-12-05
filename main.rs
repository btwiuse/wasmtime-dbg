use wasmtime::{Config, Engine, Error, InstanceAllocationStrategy, PoolingAllocationConfig};

fn main() -> Result<(), Error> {
    let mut config = Config::new();

    // dbg!(&config);

    let pooling_config = PoolingAllocationConfig::default();

    // dbg!(&pooling_config);

    config.allocation_strategy(InstanceAllocationStrategy::Pooling(pooling_config));

    let _engine = Engine::new(&config)?;

    Ok(())
}
