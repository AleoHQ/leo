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

use crate::load_asg;

#[test]
fn test_num_returns_fail() {
    let program_string = include_str!("num_returns_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_let_mut_declaration_fail() {
    let program_string = include_str!("let_mut_declaration_fail.leo");
    load_asg(program_string).err().unwrap();
}
