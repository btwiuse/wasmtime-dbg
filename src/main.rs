use nothing::Probably;
use wasmtime::{Config, Engine, Error};

fn main() -> Result<(), Error> {
    let config = Config::default();
    dbg!(&config);
    let engine = Engine::new(&config)?;
    // dbg!(&engine);
    Ok(())
    // println!("Hello, world!").into()
}
