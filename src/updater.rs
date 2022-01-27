use serde_json::Value as JsonValue;
use std::error::Error;
use std::{fs, vec};
use toml::Value as TomlValue;
use walkdir::{DirEntry, WalkDir};

pub fn process_sources(
    source_opts: &TomlValue,
) -> Result<(Vec<String>, Vec<JsonValue>), Box<dyn Error>> {
    let updates = fs::read_to_string(source_opts["update_file"].as_str().unwrap())
        .expect("Updates file does not exist. Check config file.")
        .parse::<JsonValue>()
        .expect("Could not parse updates file.");
    let updates = updates["updates"].as_array().unwrap();

    let mut package_list: Vec<String> = vec![];

    for update in updates {
        // TODO: update empty hashes

        package_list.push(update["name"].as_str().unwrap().to_string());
    }

    Ok((package_list, updates.to_vec()))
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}

fn is_json(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|ext| ext == "json")
        .unwrap_or(false)
}

// get update info for given package name
fn get_update_info(updates: &[JsonValue], name: &str) -> Result<JsonValue, Box<dyn Error>> {
    for update in updates {
        if update["name"].as_str().unwrap() == name {
            return Ok(update.clone());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not find update info for package",
    )))
}

// get '__exact' identifier
fn get_update_identifier(update_source: &JsonValue) -> Result<String, Box<dyn Error>> {
    for v in update_source.as_object().unwrap().keys() {
        if v.ends_with("__exact") {
            return Ok(v.to_string());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not find update identifier",
    )))
}

fn rewrite_source_attributes(
    orig_sources: &JsonValue,
    new_source: &JsonValue,
) -> Result<(), Box<dyn Error>> {
    let identifier = &get_update_identifier(new_source)?;

    let mut o = orig_sources.clone();

    for orig_source in o.as_array_mut().unwrap() {
        let orig_source = orig_source.as_object_mut().unwrap();
        if orig_source[String::from(identifier).strip_suffix("__exact").unwrap()]
            == new_source[identifier]
        {
            println!("found match");
            for (k, v) in new_source.as_object().unwrap() {
                if k != identifier {
                    orig_source[k] = v.clone();
                }
            }
        }
    }

    println!("rewritten sources:\n {:#?}", o);

    Ok(())
}

pub fn update_sources(
    source_opts: &TomlValue,
    update_package_list: &[String],
    updates: &[JsonValue],
) -> Result<(), Box<dyn Error>> {
    let walker = WalkDir::new(".").into_iter();

    // check if entry is not hidden and is json
    for entry in walker.filter_entry(is_not_hidden) {
        let entry = entry.unwrap();

        // check if entry is json and not equal to source_opts["update_file"]
        if is_json(&entry)
            && entry.path().to_str().unwrap() != source_opts["update_file"].as_str().unwrap()
        {
            // read entry
            let manifest = fs::read_to_string(entry.path())?;

            // parse manifest
            let manifest = manifest.parse::<JsonValue>()?;

            let modules = manifest.get("modules").unwrap().as_array().unwrap();

            for module in modules {
                let module_name = module["name"].as_str().unwrap();

                // check if module is in package list
                if update_package_list.contains(&module_name.to_string()) {
                    let update_info = get_update_info(updates, module["name"].as_str().unwrap())?;

                    for update_source in update_info["sources"].as_array().unwrap() {
                        rewrite_source_attributes(&module["sources"], update_source)?;
                    }
                }
            }
        }
    }

    Ok(())
}
