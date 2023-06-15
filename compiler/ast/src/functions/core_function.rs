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
    BHP256CommitToAddress,
    BHP256CommitToField,
    BHP256CommitToGroup,

    BHP256HashToAddress,
    BHP256HashToField,
    BHP256HashToGroup,
    BHP256HashToI8,
    BHP256HashToI16,
    BHP256HashToI32,
    BHP256HashToI64,
    BHP256HashToI128,
    BHP256HashToU8,
    BHP256HashToU16,
    BHP256HashToU32,
    BHP256HashToU64,
    BHP256HashToU128,
    BHP256HashToScalar,

    BHP512CommitToAddress,
    BHP512CommitToField,
    BHP512CommitToGroup,
    BHP512HashToAddress,
    BHP512HashToField,
    BHP512HashToGroup,
    BHP512HashToScalar,

    BHP768CommitToAddress,
    BHP768CommitToField,
    BHP768CommitToGroup,
    BHP768HashToAddress,
    BHP768HashToField,
    BHP768HashToGroup,
    BHP768HashToScalar,

    BHP1024CommitToAddress,
    BHP1024CommitToField,
    BHP1024CommitToGroup,
    BHP1024HashToAddress,
    BHP1024HashToField,
    BHP1024HashToGroup,
    BHP1024HashToScalar,

    Pedersen64CommitToAddress,
    Pedersen64CommitToField,
    Pedersen64CommitToGroup,
    Pedersen64HashToAddress,
    Pedersen64HashToField,
    Pedersen64HashToGroup,
    Pedersen64HashToScalar,

    Pedersen128CommitToAddress,
    Pedersen128CommitToField,
    Pedersen128CommitToGroup,
    Pedersen128HashToAddress,
    Pedersen128HashToField,
    Pedersen128HashToGroup,
    Pedersen128HashToScalar,

    Poseidon2HashToAddress,
    Poseidon2HashToField,
    Poseidon2HashToGroup,
    Poseidon2HashToScalar,

    Poseidon4HashToAddress,
    Poseidon4HashToField,
    Poseidon4HashToGroup,
    Poseidon4HashToScalar,

    Poseidon8HashToAddress,
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
            (sym::BHP256, sym::commit_to_address) => Self::BHP256CommitToAddress,
            (sym::BHP256, sym::commit_to_field) => Self::BHP256CommitToField,
            (sym::BHP256, sym::commit_to_group) => Self::BHP256CommitToGroup,

            (sym::BHP256, sym::hash_to_address) => Self::BHP256HashToAddress,
            (sym::BHP256, sym::hash_to_field) => Self::BHP256HashToField,
            (sym::BHP256, sym::hash_to_group) => Self::BHP256HashToGroup,
            (sym::BHP256, sym::hash_to_i8) => Self::BHP256HashToI8,
            (sym::BHP256, sym::hash_to_i16) => Self::BHP256HashToI16,
            (sym::BHP256, sym::hash_to_i32) => Self::BHP256HashToI32,
            (sym::BHP256, sym::hash_to_i64) => Self::BHP256HashToI64,
            (sym::BHP256, sym::hash_to_i128) => Self::BHP256HashToI128,
            (sym::BHP256, sym::hash_to_u8) => Self::BHP256HashToU8,
            (sym::BHP256, sym::hash_to_u16) => Self::BHP256HashToU16,
            (sym::BHP256, sym::hash_to_u32) => Self::BHP256HashToU32,
            (sym::BHP256, sym::hash_to_u64) => Self::BHP256HashToU64,
            (sym::BHP256, sym::hash_to_u128) => Self::BHP256HashToU128,
            (sym::BHP256, sym::hash_to_scalar) => Self::BHP256HashToScalar,

            (sym::BHP512, sym::commit_to_address) => Self::BHP512CommitToAddress,
            (sym::BHP512, sym::commit_to_field) => Self::BHP512CommitToField,
            (sym::BHP512, sym::commit_to_group) => Self::BHP512CommitToGroup,
            (sym::BHP512, sym::hash_to_address) => Self::BHP512HashToAddress,
            (sym::BHP512, sym::hash_to_field) => Self::BHP512HashToField,
            (sym::BHP512, sym::hash_to_group) => Self::BHP512HashToGroup,
            (sym::BHP512, sym::hash_to_scalar) => Self::BHP512HashToScalar,

            (sym::BHP768, sym::commit_to_address) => Self::BHP768CommitToAddress,
            (sym::BHP768, sym::commit_to_field) => Self::BHP768CommitToField,
            (sym::BHP768, sym::commit_to_group) => Self::BHP768CommitToGroup,
            (sym::BHP768, sym::hash_to_address) => Self::BHP768HashToAddress,
            (sym::BHP768, sym::hash_to_field) => Self::BHP768HashToField,
            (sym::BHP768, sym::hash_to_group) => Self::BHP768HashToGroup,
            (sym::BHP768, sym::hash_to_scalar) => Self::BHP768HashToScalar,

            (sym::BHP1024, sym::commit_to_address) => Self::BHP1024CommitToAddress,
            (sym::BHP1024, sym::commit_to_field) => Self::BHP1024CommitToField,
            (sym::BHP1024, sym::commit_to_group) => Self::BHP1024CommitToGroup,
            (sym::BHP1024, sym::hash_to_address) => Self::BHP1024HashToAddress,
            (sym::BHP1024, sym::hash_to_field) => Self::BHP1024HashToField,
            (sym::BHP1024, sym::hash_to_group) => Self::BHP1024HashToGroup,
            (sym::BHP1024, sym::hash_to_scalar) => Self::BHP1024HashToScalar,

            (sym::Pedersen64, sym::commit_to_address) => Self::Pedersen64CommitToAddress,
            (sym::Pedersen64, sym::commit_to_field) => Self::Pedersen64CommitToField,
            (sym::Pedersen64, sym::commit_to_group) => Self::Pedersen64CommitToGroup,
            (sym::Pedersen64, sym::hash_to_address) => Self::Pedersen64HashToAddress,
            (sym::Pedersen64, sym::hash_to_field) => Self::Pedersen64HashToField,
            (sym::Pedersen64, sym::hash_to_group) => Self::Pedersen64HashToGroup,
            (sym::Pedersen64, sym::hash_to_scalar) => Self::Pedersen64HashToScalar,

            (sym::Pedersen128, sym::commit_to_address) => Self::Pedersen128CommitToAddress,
            (sym::Pedersen128, sym::commit_to_field) => Self::Pedersen128CommitToField,
            (sym::Pedersen128, sym::commit_to_group) => Self::Pedersen128CommitToGroup,
            (sym::Pedersen128, sym::hash_to_address) => Self::Pedersen128HashToAddress,
            (sym::Pedersen128, sym::hash_to_field) => Self::Pedersen128HashToField,
            (sym::Pedersen128, sym::hash_to_group) => Self::Pedersen128HashToGroup,
            (sym::Pedersen128, sym::hash_to_scalar) => Self::Pedersen128HashToScalar,

            (sym::Poseidon2, sym::hash_to_address) => Self::Poseidon2HashToAddress,
            (sym::Poseidon2, sym::hash_to_field) => Self::Poseidon2HashToField,
            (sym::Poseidon2, sym::hash_to_group) => Self::Poseidon2HashToGroup,
            (sym::Poseidon2, sym::hash_to_scalar) => Self::Poseidon2HashToScalar,

            (sym::Poseidon4, sym::hash_to_address) => Self::Poseidon4HashToAddress,
            (sym::Poseidon4, sym::hash_to_field) => Self::Poseidon4HashToField,
            (sym::Poseidon4, sym::hash_to_group) => Self::Poseidon4HashToGroup,
            (sym::Poseidon4, sym::hash_to_scalar) => Self::Poseidon4HashToScalar,

            (sym::Poseidon8, sym::hash_to_address) => Self::Poseidon8HashToAddress,
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
            Self::BHP256CommitToAddress => 2,
            Self::BHP256CommitToField => 2,
            Self::BHP256CommitToGroup => 2,

            Self::BHP256HashToAddress => 1,
            Self::BHP256HashToField => 1,
            Self::BHP256HashToGroup => 1,
            Self::BHP256HashToI8 => 1,
            Self::BHP256HashToI16 => 1,
            Self::BHP256HashToI32 => 1,
            Self::BHP256HashToI64 => 1,
            Self::BHP256HashToI128 => 1,
            Self::BHP256HashToU8 => 1,
            Self::BHP256HashToU16 => 1,
            Self::BHP256HashToU32 => 1,
            Self::BHP256HashToU64 => 1,
            Self::BHP256HashToU128 => 1,
            Self::BHP256HashToScalar => 1,

            Self::BHP512CommitToAddress => 2,
            Self::BHP512CommitToField => 2,
            Self::BHP512CommitToGroup => 2,
            Self::BHP512HashToAddress => 1,
            Self::BHP512HashToField => 1,
            Self::BHP512HashToGroup => 1,
            Self::BHP512HashToScalar => 1,

            Self::BHP768CommitToAddress => 2,
            Self::BHP768CommitToField => 2,
            Self::BHP768CommitToGroup => 2,
            Self::BHP768HashToAddress => 1,
            Self::BHP768HashToField => 1,
            Self::BHP768HashToGroup => 1,
            Self::BHP768HashToScalar => 1,

            Self::BHP1024CommitToAddress => 2,
            Self::BHP1024CommitToField => 2,
            Self::BHP1024CommitToGroup => 2,
            Self::BHP1024HashToAddress => 1,
            Self::BHP1024HashToField => 1,
            Self::BHP1024HashToGroup => 1,
            Self::BHP1024HashToScalar => 1,

            Self::Pedersen64CommitToAddress => 2,
            Self::Pedersen64CommitToField => 2,
            Self::Pedersen64CommitToGroup => 2,
            Self::Pedersen64HashToAddress => 1,
            Self::Pedersen64HashToField => 1,
            Self::Pedersen64HashToGroup => 1,
            Self::Pedersen64HashToScalar => 1,

            Self::Pedersen128CommitToAddress => 2,
            Self::Pedersen128CommitToField => 2,
            Self::Pedersen128CommitToGroup => 2,
            Self::Pedersen128HashToAddress => 1,
            Self::Pedersen128HashToField => 1,
            Self::Pedersen128HashToGroup => 1,
            Self::Pedersen128HashToScalar => 1,

            Self::Poseidon2HashToAddress => 1,
            Self::Poseidon2HashToField => 1,
            Self::Poseidon2HashToGroup => 1,
            Self::Poseidon2HashToScalar => 1,

            Self::Poseidon4HashToAddress => 1,
            Self::Poseidon4HashToField => 1,
            Self::Poseidon4HashToGroup => 1,
            Self::Poseidon4HashToScalar => 1,

            Self::Poseidon8HashToAddress => 1,
            Self::Poseidon8HashToField => 1,
            Self::Poseidon8HashToGroup => 1,
            Self::Poseidon8HashToScalar => 1,

            Self::MappingGet => 2,
            Self::MappingGetOrUse => 3,
            Self::MappingSet => 3,
        }
    }
}
