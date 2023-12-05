use wasmtime::{Config, Engine, Error, InstanceAllocationStrategy};

fn main() -> Result<(), Error> {
    let mut config = Config::new();

    let _engine_good = Engine::new(dbg!(&config))?;

    config.allocation_strategy(InstanceAllocationStrategy::pooling());

    let _engine_linux_aarch64_bad = Engine::new(dbg!(&config))?;

    Ok(())
}
