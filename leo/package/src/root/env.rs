// Copyright (C) 2019-2023 Aleo Systems Inc.
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

//! The `.env` file.

use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static ENV_FILENAME: &str = ".env";

#[derive(Deserialize, Default)]
pub struct Env;

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(ENV_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(ENV_FILENAME);
        }

        let mut file = File::create(&path).map_err(PackageError::io_error_gitignore_file)?;
        file.write_all(self.template().as_bytes()).map_err(PackageError::io_error_gitignore_file)?;
        Ok(())
    }

    fn template(&self) -> String {
        "NETWORK=testnet3\nPRIVATE_KEY=APrivateKey1zkpBvXdKZKaXXcLUnwAVFCQNp41jrX6JqTuJo1JShfPoRfx\n".to_string()
    }
}
