pub mod config;
pub mod shell;

use std::{env, process::exit, str::FromStr};

use config::Config;

use argparse::{ArgumentParser, StoreOption};

use crate::shell::Shell;

const DEFAULT_CONFIG_PATH: &str = ".config/alias-rs/config.json";

fn failure(msg: String) -> ! {
    eprintln!("Failure: {}", msg);
    exit(1)
}

fn main() {
    let mut config_path: Option<String> = None;
    let config_option_help = format!(
        "Path to the configuration file (default: $HOME/{})",
        DEFAULT_CONFIG_PATH
    );
    let mut shell: Option<Shell> = None;
    let shell_option_help = format!(
        "Shell to create aliases for (default: $SHELL/{})",
        DEFAULT_CONFIG_PATH
    );

    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut config_path).add_option(
            &["-c", "--config"],
            StoreOption,
            &config_option_help,
        );

        ap.refer(&mut shell)
            .add_option(&["-s", "--shell"], StoreOption, &shell_option_help);

        ap.parse_args_or_exit();
    }

    let config_path = match config_path {
        Some(c) => c,
        None => match env::var("HOME") {
            Ok(h) => format!("{}/{}", h, DEFAULT_CONFIG_PATH),
            Err(_) => failure("Could not obtain default config path".to_string()),
        },
    };

    let config = match Config::from_file(config_path.as_str()) {
        Ok(c) => c,
        Err(e) => failure(format!("Could not obtain config: {}", e)),
    };

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
    for alias in config.aliases {
        if let Some(alias_string) = shell.create_alias(&alias) {
            alias_strings.push(alias_string);
        }
    }

    println!("{}", alias_strings.join(""));
}
