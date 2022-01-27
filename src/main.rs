use clap::Parser;
use updater::update_sources;

mod parse_config;
mod updater;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(about = "Program to update Flatpak manifests", long_about = None)]
struct Args {
    /// Config file path
    #[clap(short, long, parse(from_os_str), default_value = ".blu.conf.toml")]
    config: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let runtime_opts = match parse_config::parse_config(args.config) {
        Ok(value) => value,
        Err(error) => panic!("{:#?}", error),
    };

    let (package_list, updates) = updater::process_sources(&runtime_opts["source"]).unwrap();

    update_sources(&runtime_opts["source"], &package_list, &updates).unwrap();
}
