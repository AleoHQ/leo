// Copyright (C) 2019-2022 Aleo Systems Inc.
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

//! The tokenizer to convert Leo code text into tokens.
//!
//! This module contains the [`tokenize()`] method which breaks down string text into tokens,
//! separated by whitespace.

pub(crate) mod token;
use std::sync::Arc;

pub use self::token::KEYWORD_TOKENS;
pub(crate) use self::token::*;

pub(crate) mod lexer;
pub(crate) use self::lexer::*;

use leo_errors::{ParserError, Result};
use leo_span::Span;

/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(path: &str, input: &str) -> Result<Vec<SpannedToken>> {
    let path = Arc::new(path.to_string());
    let mut tokens = vec![];
    let mut index = 0usize;
    let mut line_no = 1usize;
    let mut line_start = 0usize;
    while input.len() > index {
        match Token::eat(&input[index..])? {
            (token_len, Token::WhiteSpace) => {
                let bytes = input.as_bytes();
                if bytes[index] == 0x000D && matches!(bytes.get(index + 1), Some(0x000A)) {
                    // Check carriage return followed by newline.
                    line_no += 1;
                    line_start = index + token_len;
                    index += token_len;
                } else if matches!(bytes[index], 0x000A | 0x000D) {
                    // Check new-line or carriage-return
                    line_no += 1;
                    line_start = index + token_len;
                }
                index += token_len;
            }
            (token_len, token) => {
                let mut span = Span::new(
                    line_no,
                    line_no,
                    index - line_start + 1,
                    index - line_start + token_len + 1,
                    path.clone(),
                    input[line_start
                        ..input[line_start..]
                            .find('\n')
                            .map(|i| i + line_start)
                            .unwrap_or(input.len())]
                        .to_string(),
                );
                match &token {
                    Token::CommentLine(_) => {
                        line_no += 1;
                        line_start = index + token_len;
                    }
                    Token::CommentBlock(block) => {
                        let line_ct = block.chars().filter(|x| *x == '\n').count();
                        line_no += line_ct;
                        if line_ct > 0 {
                            let last_line_index = block.rfind('\n').unwrap();
                            line_start = index + last_line_index + 1;
                            span.col_stop = index + token_len - line_start + 1;
                        }
                        span.line_stop = line_no;
                    }
                    Token::AddressLit(address) => {
                        if !check_address(address) {
                            return Err(ParserError::invalid_address_lit(address, &span).into());
                        }
                    }
                    _ => (),
                }
                tokens.push(SpannedToken { token, span });
                index += token_len;
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::symbol::create_session_if_not_set_then;

    #[test]
    fn test_tokenizer() {
        create_session_if_not_set_then(|_| {
            let tokens = tokenize(
                "test_path",
                r#"
        'a'
        '😭'
        "test"
        "test{}test"
        "test{}"
        "{}test"
        "test{"
        "test}"
        "test{test"
        "test}test"
        "te{{}}"
        aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8
        test_ident
        12345
        address
        bool
        const
        else
        false
        field
        for
        function
        group
        i128
        i64
        i32
        i16
        i8
        if
        in
        input
        let
        mut
        return
        string
        test
        true
        u128
        u64
        u32
        u16
        u8
        console
        !
        !=
        &&
        (
        )
        *
        **
        +
        ,
        -
        ->
        _
        .
        ..
        /
        :
        ;
        <
        <=
        =
        ==
        >
        >=
        [
        ]
        {{
        }}
        ||
        ?
        // test
        /* test */
        //"#,
            )
            .unwrap();
            let mut output = String::new();
            for SpannedToken { token, .. } in tokens.iter() {
                output += &format!("{} ", token);
            }

            assert_eq!(
                output,
                r#"'a' '😭' "test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8 test_ident 12345 address bool const else false field for function group i128 i64 i32 i16 i8 if in input let mut return string test true u128 u64 u32 u16 u8 console ! != && ( ) * ** + , - -> _ . .. / : ; < <= = == > >= [ ] { { } } || ? // test
 /* test */ // "#
            );
        });
    }

    #[test]
    fn test_spans() {
        create_session_if_not_set_then(|_| {
            let raw = r#"
ppp            test
            // test
            test
            /* test */
            test
            /* test
            test */
            test
            "#;
            let tokens = tokenize("test_path", raw).unwrap();
            let mut line_indicies = vec![0];
            for (i, c) in raw.chars().enumerate() {
                if c == '\n' {
                    line_indicies.push(i + 1);
                }
            }
            for token in tokens.iter() {
                let token_raw = token.token.to_string();
                let start = line_indicies.get(token.span.line_start - 1).unwrap();
                let stop = line_indicies.get(token.span.line_stop - 1).unwrap();
                let original = &raw[*start + token.span.col_start - 1..*stop + token.span.col_stop - 1];
                assert_eq!(original, &token_raw);
            }
        })
    }
}
