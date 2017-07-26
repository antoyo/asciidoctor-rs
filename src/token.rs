/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Tokens from an asciidoctor document.

use self::Token::*;

/// Different types of token.
#[derive(Debug, PartialEq)]
pub enum Token {
    Backquote,
    Caret,
    CloseSquareBracket,
    NewLine,
    NumberSign,
    OpenSquareBracket,
    Space,
    Star,
    Tilde,
    TripleApos,
    TripleLt,
    Underscore,
    Word(Vec<u8>),
}

impl Token {
    /// Convert the token to a user-readable string.
    /// Useful for error reporting.
    pub fn to_string(&self) -> String {
        match *self {
            Backquote => "`".to_string(),
            Caret => "^".to_string(),
            CloseSquareBracket => "]".to_string(),
            NewLine => "(newline)".to_string(),
            NumberSign => "#".to_string(),
            OpenSquareBracket => "[".to_string(),
            Space => "(space)".to_string(),
            Star => "*".to_string(),
            Tilde => "~".to_string(),
            TripleApos => "'''".to_string(),
            TripleLt => "<<<".to_string(),
            Underscore => "_".to_string(),
            Word(ref word) => String::from_utf8_lossy(word).to_string(),
        }
    }
}
