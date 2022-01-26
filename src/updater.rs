use serde_json::Value as JsonValue;
use std::error::Error;
use std::fs;
use toml::Value as TomlValue;

pub fn process_sources(source_opts: &TomlValue) -> Result<(), Box<dyn Error>> {
    let updates = fs::read_to_string(source_opts["updates"].as_str().unwrap())
        .expect("Updates file does not exist. Check config file.")
        .parse::<JsonValue>()
        .expect("Could not parse updates file.");
    let updates = updates["updates"].as_array().unwrap();

    for update in updates {
        // update empty hashes
        println!("{:#?}", update);
    }

    Ok(())
}
