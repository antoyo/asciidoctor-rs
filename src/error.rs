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

use std::fmt::{self, Display, Formatter};
use std::io;
use std::result;
use std::string::FromUtf8Error;

use position::Pos;
use self::Error::{Eof, Msg, UnexpectedChar, UnexpectedToken};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Eof,
    Msg(String),
    UnexpectedChar {
        actual: u8,
        expected: Vec<u8>,
        pos: Pos,
    },
    UnexpectedToken {
        actual: String,
        expected: String,
        pos: Pos,
    },
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Eof => write!(fmt, "end of file"),
            Msg(ref message) => write!(fmt, "{}", message),
            UnexpectedChar { ref actual, ref expected, ref pos } =>
                write!(fmt, "{}:{}: expected {}, but found `{}` on line {}, column {}", pos.line, pos.column,
                       expected_chars(expected), actual, pos.line, pos.column),
            UnexpectedToken { ref actual, ref expected, ref pos } =>
                write!(fmt, "{}:{}: expected {}, but found `{}` on line {}, column {}", pos.line, pos.column, expected,
                       actual, pos.line, pos.column),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(string: &str) -> Self {
        Msg(string.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Msg(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Msg(error.to_string())
    }
}

fn expected_chars(expected: &[u8]) -> String {
    if expected.len() == 1 {
        format!("`{}`", expected[0])
    }
    else {
        let chars = expected.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("`, `");
        format!("one of `{}`", chars)
    }
}
