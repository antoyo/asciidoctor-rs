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

extern crate asciidoctor;
extern crate html_diff;
#[macro_use]
extern crate pretty_assertions;

use html_diff::get_differences;

use asciidoctor::{Lexer, Parser};
use asciidoctor::html::{self, Generator};

#[test]
fn test_parse_gen() {
    let file = include_str!("input/block_page_break.adoc");
    let lexer = Lexer::new(file.as_bytes());
    let mut parser = Parser::new(lexer);
    let mut html = vec![];
    {
        let mut generator = Generator::new(&mut html);
        loop {
            let node = parser.nodes();
            match node {
                Ok(node) => html::gen(&mut generator, &node).unwrap(),
                Err(Eof) => break,
                Err(err) => panic!("cannot parse asciidoctor: {}", err),
            }
        }
    }
    let html = String::from_utf8(html).unwrap();

    let result_file = include_str!("output/block_page_break.html");
    let differences = get_differences(result_file, &html);
    if !differences.is_empty() {
        let mut diffs = "\n".to_string();
        for diff in differences {
            diffs += &diff.to_string();
            diffs += "\n";
        }
        assert!(false, diffs);
    }
}
