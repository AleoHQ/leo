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

use leo_span::{sym, Symbol};

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreFunction {
    BHP256Commit,
    BHP256CommitToGroup,
    BHP256HashToField,
    BHP256HashToGroup,

    BHP512Commit,
    BHP512CommitToGroup,
    BHP512HashToField,
    BHP512HashToGroup,

    BHP768Commit,
    BHP768CommitToGroup,
    BHP768HashToField,
    BHP768HashToGroup,

    BHP1024Commit,
    BHP1024CommitToGroup,
    BHP1024HashToField,
    BHP1024HashToGroup,

    Pedersen64Commit,
    Pedersen64CommitToGroup,
    Pedersen64HashToField,
    Pedersen64HashToGroup,

    Pedersen128Commit,
    Pedersen128CommitToGroup,
    Pedersen128HashToField,
    Pedersen128HashToGroup,

    Poseidon2HashToField,
    Poseidon2HashToGroup,
    Poseidon2HashToScalar,

    Poseidon4HashToField,
    Poseidon4HashToGroup,
    Poseidon4HashToScalar,

    Poseidon8HashToField,
    Poseidon8HashToGroup,
    Poseidon8HashToScalar,

    MappingGet,
    MappingGetOrUse,
    MappingSet,
}

impl CoreFunction {
    /// Returns a `CoreFunction` from the given module and method symbols.
    pub fn from_symbols(module: Symbol, function: Symbol) -> Option<Self> {
        Some(match (module, function) {
            (sym::BHP256, sym::commit) => Self::BHP256Commit,
            (sym::BHP256, sym::commit_to_group) => Self::BHP256CommitToGroup,
            (sym::BHP256, sym::hash_to_field) => Self::BHP256HashToField,
            (sym::BHP256, sym::hash_to_group) => Self::BHP256HashToGroup,

            (sym::BHP512, sym::commit) => Self::BHP512Commit,
            (sym::BHP512, sym::commit_to_group) => Self::BHP512CommitToGroup,
            (sym::BHP512, sym::hash_to_field) => Self::BHP512HashToField,
            (sym::BHP512, sym::hash_to_group) => Self::BHP512HashToGroup,

            (sym::BHP768, sym::commit) => Self::BHP768Commit,
            (sym::BHP768, sym::commit_to_group) => Self::BHP768CommitToGroup,
            (sym::BHP768, sym::hash_to_field) => Self::BHP768HashToField,
            (sym::BHP768, sym::hash_to_group) => Self::BHP768HashToGroup,

            (sym::BHP1024, sym::commit) => Self::BHP1024Commit,
            (sym::BHP1024, sym::commit_to_group) => Self::BHP1024CommitToGroup,
            (sym::BHP1024, sym::hash_to_field) => Self::BHP1024HashToField,
            (sym::BHP1024, sym::hash_to_group) => Self::BHP1024HashToGroup,

            (sym::Pedersen64, sym::commit) => Self::Pedersen64Commit,
            (sym::Pedersen64, sym::commit_to_group) => Self::Pedersen64CommitToGroup,
            (sym::Pedersen64, sym::hash_to_field) => Self::Pedersen64HashToField,
            (sym::Pedersen64, sym::hash_to_group) => Self::Pedersen64HashToGroup,

            (sym::Pedersen128, sym::commit) => Self::Pedersen128Commit,
            (sym::Pedersen128, sym::commit_to_group) => Self::Pedersen128CommitToGroup,
            (sym::Pedersen128, sym::hash_to_field) => Self::Pedersen128HashToField,
            (sym::Pedersen128, sym::hash_to_group) => Self::Pedersen128HashToGroup,

            (sym::Poseidon2, sym::hash_to_field) => Self::Poseidon2HashToField,
            (sym::Poseidon2, sym::hash_to_group) => Self::Poseidon2HashToGroup,
            (sym::Poseidon2, sym::hash_to_scalar) => Self::Poseidon2HashToScalar,

            (sym::Poseidon4, sym::hash_to_field) => Self::Poseidon4HashToField,
            (sym::Poseidon4, sym::hash_to_group) => Self::Poseidon4HashToGroup,
            (sym::Poseidon4, sym::hash_to_scalar) => Self::Poseidon4HashToScalar,

            (sym::Poseidon8, sym::hash_to_field) => Self::Poseidon8HashToField,
            (sym::Poseidon8, sym::hash_to_group) => Self::Poseidon8HashToGroup,
            (sym::Poseidon8, sym::hash_to_scalar) => Self::Poseidon8HashToScalar,

            (sym::Mapping, sym::get) => Self::MappingGet,
            (sym::Mapping, sym::get_or_use) => Self::MappingGetOrUse,
            (sym::Mapping, sym::set) => Self::MappingSet,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            Self::BHP256Commit => 2,
            Self::BHP256CommitToGroup => 2,
            Self::BHP256HashToField => 1,
            Self::BHP256HashToGroup => 1,

            Self::BHP512Commit => 2,
            Self::BHP512CommitToGroup => 2,
            Self::BHP512HashToField => 1,
            Self::BHP512HashToGroup => 1,

            Self::BHP768Commit => 2,
            Self::BHP768CommitToGroup => 2,
            Self::BHP768HashToField => 1,
            Self::BHP768HashToGroup => 1,

            Self::BHP1024Commit => 2,
            Self::BHP1024CommitToGroup => 2,
            Self::BHP1024HashToField => 1,
            Self::BHP1024HashToGroup => 1,

            Self::Pedersen64Commit => 2,
            Self::Pedersen64CommitToGroup => 2,
            Self::Pedersen64HashToField => 1,
            Self::Pedersen64HashToGroup => 1,

            Self::Pedersen128Commit => 2,
            Self::Pedersen128CommitToGroup => 2,
            Self::Pedersen128HashToField => 1,
            Self::Pedersen128HashToGroup => 1,

            Self::Poseidon2HashToField => 1,
            Self::Poseidon2HashToGroup => 1,
            Self::Poseidon2HashToScalar => 1,

            Self::Poseidon4HashToField => 1,
            Self::Poseidon4HashToGroup => 1,
            Self::Poseidon4HashToScalar => 1,

            Self::Poseidon8HashToField => 1,
            Self::Poseidon8HashToGroup => 1,
            Self::Poseidon8HashToScalar => 1,

            Self::MappingGet => 2,
            Self::MappingGetOrUse => 3,
            Self::MappingSet => 3,
        }
    }
}
