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

///
/// Toggles compiler optimizations on the program.
///
#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub constant_folding_enabled: bool,
    pub dead_code_elimination_enabled: bool,
}

impl Default for CompilerOptions {
    ///
    /// All compiler optimizations are enabled by default.
    ///
    fn default() -> Self {
        CompilerOptions {
            constant_folding_enabled: true,
            dead_code_elimination_enabled: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct OutputOptions {
    pub ast_initial: bool,
    pub ast_imports_resolved: bool,
    pub ast_canonicalized: bool,
    pub ast_type_inferenced: bool,
    pub emit_ir: bool,
}
