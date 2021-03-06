// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use std::path::PathBuf;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum ZipFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Cannot remove the provided file - {:?}", _0)]
    FileRemovalError(PathBuf),

    #[error("{}", _0)]
    ZipError(#[from] ZipError),
}

impl From<std::io::Error> for ZipFileError {
    fn from(error: std::io::Error) -> Self {
        ZipFileError::Crate("std::io", error.to_string())
    }
}
