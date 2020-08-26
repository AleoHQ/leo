// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::errors::CLIError;

use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all, File},
    io,
    io::prelude::*,
    path::{Path, PathBuf},
};

pub const PACKAGE_MANAGER_URL: &str = "https://apm-backend-prod.herokuapp.com/";

pub const LEO_CREDENTIALS_FILE: &str = "credentials";
pub const LEO_CONFIG_FILE: &str = "config.toml";

lazy_static! {
    pub static ref LEO_CONFIG_DIRECTORY: PathBuf = {
        let mut path = home_dir().expect("Invalid home directory");
        path.push(".leo");
        path
    };
    pub static ref LEO_CREDENTIALS_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CREDENTIALS_FILE);
        path
    };
    pub static ref LEO_CONFIG_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CONFIG_FILE);
        path
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub auto_update: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { auto_update: true }
    }
}

impl Config {
    /// Read the config from the `config.toml` file
    pub fn read_config() -> Result<Self, CLIError> {
        let config_dir = LEO_CONFIG_DIRECTORY.clone();
        let config_path = LEO_CONFIG_PATH.clone();

        if !Path::exists(&config_path) {
            // Create a new default `config.toml` file if it doesn't already exist
            create_dir_all(&config_dir)?;

            let default_config_string = toml::to_string(&Config::default())?;

            fs::write(&config_path, default_config_string)?;
        }

        let toml_string = match fs::read_to_string(&config_path) {
            Ok(toml) => toml,
            Err(_) => {
                create_dir_all(&config_dir)?;
                String::new()
            }
        };

        // Parse the contents into the `Config` struct
        let config: Config = toml::from_str(&toml_string)?;

        Ok(config)
    }
}

pub fn write_token(token: &str) -> Result<(), io::Error> {
    let config_dir = LEO_CONFIG_DIRECTORY.clone();

    // Create Leo config directory if it not exists
    if !Path::new(&config_dir.to_path_buf()).exists() {
        create_dir_all(&config_dir)?;
    }

    let mut credentials = File::create(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    credentials.write_all(&token.as_bytes())?;
    Ok(())
}

pub fn read_token() -> Result<String, io::Error> {
    let mut credentials = File::open(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    let mut buf = String::new();
    credentials.read_to_string(&mut buf)?;
    Ok(buf)
}
