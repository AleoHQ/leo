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

use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum DeprecatedError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl DeprecatedError {
    fn new_from_span(message: String, span: &Span) -> Self {
        DeprecatedError::Error(FormattedError::new_from_span(message, span))
    }
}

impl LeoError for DeprecatedError {}

impl DeprecatedError {
    pub fn mut_function_input(mut span: Span) -> Self {
        let message =
            "function func(mut a: u32) { ... } is deprecated. Passed variables are mutable by default.".to_string();
        span.col_start -= 1;
        span.col_stop -= 1;
        Self::new_from_span(message, &span)
    }

    pub fn let_mut_statement(mut span: Span) -> Self {
        let message = "let mut = ... is deprecated. `let` keyword implies mutabality by default.".to_string();
        span.col_start -= 1;
        span.col_stop -= 1;
        Self::new_from_span(message, &span)
    }

    pub fn test_function(span: &Span) -> Self {
        let message = "\"test function...\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, span)
    }

    pub fn context_annotation(span: &Span) -> Self {
        let message = "\"@context(...)\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, span)
    }
}
