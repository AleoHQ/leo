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

//! This module contains the reducer which iterates through ast nodes - converting them into
//! asg nodes and saving relevant information.

mod monoid;
pub use monoid::*;

mod monoidal_director;
pub use monoidal_director::*;

mod monoidal_reducer;
pub use monoidal_reducer::*;

mod reconstructing_reducer;
pub use reconstructing_reducer::*;

mod reconstructing_director;
pub use reconstructing_director::*;

mod visitor;
pub use visitor::*;

mod visitor_director;
pub use visitor_director::*;
