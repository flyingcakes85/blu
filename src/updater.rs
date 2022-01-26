use std::error::Error;
use toml::Value;

pub fn process_sources(_source_opts: &Value) -> Result<(), Box<dyn Error>> {
    Ok(())
}
