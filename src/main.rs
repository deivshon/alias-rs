pub mod config;
pub mod shell;

use std::{env, process::exit, str::FromStr};

use config::Config;

use argparse::{ArgumentParser, Collect, StoreOption, StoreTrue};

use crate::shell::Shell;

const DEFAULT_CONFIG_PATH: &str = ".config/alias-rs/config.json";

fn failure(msg: String) -> ! {
    eprintln!("Failure: {}", msg);
    exit(1)
}

fn main() {
    let mut configs_paths: Vec<String> = Vec::new();
    let mut shell: Option<Shell> = None;
    let mut always_include: bool = false;

    let config_option_help = format!(
        "Path to the configuration file (default: $HOME/{})",
        DEFAULT_CONFIG_PATH
    );

    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut configs_paths)
            .add_option(&["-c", "--config"], Collect, &config_option_help);

        ap.refer(&mut shell).add_option(
            &["-s", "--shell"],
            StoreOption,
            "Shell to create aliases for (default: $SHELL)",
        );

        ap.refer(&mut always_include).add_option(&["-a", "--always-include"], StoreTrue, "Include the default configuration file when others are specified through command line arguments");

        ap.parse_args_or_exit();
    }

    if configs_paths.is_empty() || always_include {
        match env::var("HOME") {
            Ok(h) => configs_paths.push(format!("{}/{}", h, DEFAULT_CONFIG_PATH)),
            Err(_) => failure("Could not obtain default configuration path".to_string()),
        }
    }

    let shell = match shell {
        Some(s) => s,
        None => match env::var("SHELL") {
            Ok(s) => match Shell::from_str(s.as_str()) {
                Ok(s) => s,
                Err(e) => failure(format!("Could not obtain default shell: {}", e)),
            },
            Err(e) => failure(format!("Could not obtain default shell: {}", e)),
        },
    };

    let mut alias_strings: Vec<String> = vec![];

    for path in configs_paths {
        let config = match Config::from_file(path.as_str()) {
            Ok(c) => c,
            Err(e) => failure(format!("Could not obtain config for path: {}: {}", path, e)),
        };

        for alias in config.aliases {
            if let Some(alias_string) = shell.create_alias(&alias) {
                alias_strings.push(alias_string);
            }
        }
    }

    println!("{}", alias_strings.join(""));
}
