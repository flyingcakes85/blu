use std::error::Error;
use std::{fs, path};
use toml::Value;

pub fn check_source_opts(_source_opts: &Value) -> Result<(), Box<dyn Error>> {
    // do some actual check

    Ok(())
}

pub fn check_build_opts(_source_opts: &Value) -> Result<(), Box<dyn Error>> {
    // do some actual check

    Ok(())
}

pub fn parse_config(config_path: path::PathBuf) -> Result<toml::Value, Box<dyn Error>> {
    let config_text = fs::read_to_string(config_path)?;

    let value = config_text.parse::<Value>()?;

    check_source_opts(&value["source"])?;

    check_build_opts(&value["build"])?;

    Ok(value)
}
