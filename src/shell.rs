use core::fmt;
use std::{path::Path, str::FromStr};

use serde::Deserialize;

use crate::config::{Alias, ShellList};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    #[serde(alias = "csh")]
    Tcsh,
    Ksh,
    Dash,
    Ash,
}

pub enum ShellParsingError {
    UnknownShell(String),
    BorkedOsStr,
    NoFileName,
}

impl fmt::Display for ShellParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellParsingError::UnknownShell(unknown_shell) => {
                write!(f, "Unknown shell: {}", unknown_shell)
            }
            ShellParsingError::NoFileName => write!(f, "Could not get file name from shell path"),
            ShellParsingError::BorkedOsStr => write!(f, "Could not convert shell OsStr to &str"),
        }
    }
}

impl FromStr for Shell {
    type Err = ShellParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_name: &str = Path::new(s)
            .file_name()
            .ok_or(ShellParsingError::NoFileName)?
            .to_str()
            .ok_or(ShellParsingError::BorkedOsStr)?;

        match file_name.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "fish" => Ok(Shell::Fish),
            "zsh" => Ok(Shell::Zsh),
            "tcsh" | "csh" => Ok(Shell::Tcsh),
            "ksh" => Ok(Shell::Ksh),
            "dash" => Ok(Shell::Dash),
            "ash" => Ok(Shell::Ash),
            unknown => Err(ShellParsingError::UnknownShell(unknown.to_string())),
        }
    }
}

impl Shell {
    pub fn create_alias(&self, alias: &Alias) -> Option<String> {
        if let Some(shell_list) = &alias.shells {
            match shell_list {
                ShellList::Blacklist(ls) => {
                    if ls.contains(&self) {
                        return None;
                    }
                }
                ShellList::Whitelist(ls) => {
                    if !ls.contains(&self) {
                        return None;
                    }
                }
            }
        }

        match self {
            Shell::Fish => Some(Shell::fish_alias(alias)),
            Shell::Bash => Some(Shell::bash_alias(alias)),
            Shell::Zsh => Some(Shell::zsh_alias(alias)),
            Shell::Tcsh => Some(Shell::tcsh_alias(alias)),
            Shell::Ksh => Some(Shell::ksh_alias(alias)),
            Shell::Dash => Some(Shell::dash_alias(alias)),
            Shell::Ash => Some(Shell::ash_alias(alias)),
        }
    }

    #[inline(always)]
    fn fish_alias(alias: &Alias) -> String {
        format!("alias {} \"{}\";", alias.alias, alias.equals)
    }

    #[inline(always)]
    fn bash_alias(alias: &Alias) -> String {
        format!("alias {}=\"{}\";", alias.alias, alias.equals)
    }

    #[inline(always)]
    fn zsh_alias(alias: &Alias) -> String {
        Shell::bash_alias(alias)
    }

    #[inline(always)]
    fn tcsh_alias(alias: &Alias) -> String {
        Shell::fish_alias(alias)
    }

    #[inline(always)]
    fn ksh_alias(alias: &Alias) -> String {
        Shell::bash_alias(alias)
    }

    #[inline(always)]
    fn dash_alias(alias: &Alias) -> String {
        Shell::bash_alias(alias)
    }

    #[inline(always)]
    fn ash_alias(alias: &Alias) -> String {
        Shell::bash_alias(alias)
    }
}
