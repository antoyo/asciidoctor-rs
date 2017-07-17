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

use std::io;
use std::string::FromUtf8Error;

use position::Pos;

error_chain! {
    errors {
        Eof {
            description("end of file")
            display("end of file")
        }
        UnexpectedChar {
            actual: u8,
            expected: Vec<u8>,
            pos: Pos,
        } {
            description("expected a character, but found another character")
            display("{}:{}: expected {}, but found `{}` on line {}, column {}", pos.line, pos.column,
                    expected_chars(expected), actual, pos.line, pos.column)
        }
        UnexpectedToken {
            actual: String,
            expected: String,
            pos: Pos,
        } {
            description("unexpected token")
            display("{}:{}: expected {}, but found `{}` on line {}, column {}", pos.line, pos.column, expected, actual,
                    pos.line, pos.column)
        }
    }

    foreign_links {
        Utf8(FromUtf8Error);
        Io(io::Error);
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
