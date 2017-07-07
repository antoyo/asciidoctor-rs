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

use std::fs::File;
use std::io::Read;

use html_diff::get_differences;

use asciidoctor::{Error, Lexer, Parser};
use asciidoctor::ErrorKind::Eof;
use asciidoctor::html::{self, Generator};

#[test]
fn test_parse_gen() {
    //generate_html_and_cmp("block_page_break");
    //generate_html_and_cmp("block_thematic_break");
    generate_html_and_cmp("inline_quoted");
    //generate_html_and_cmp("block_admonition");
}

fn generate_html_and_cmp(name: &str) {
    let file = read_file(&format!("input/{}.adoc", name));
    let lexer = Lexer::new(file.as_bytes());
    let mut parser = Parser::new(lexer);
    let mut html = String::new();
    {
        let mut generator = Generator {};
        loop {
            let node = parser.node();
            match node {
                Ok(node) => html.push_str(&html::gen(&mut generator, &node)),
                Err(Error(Eof, _)) => break,
                Err(err) => panic!("cannot parse asciidoctor: {}", err),
            }
        }
    }

    let result_file = read_file(&format!("output/{}.html", name));
    let differences = get_differences(&result_file, &html);
    if !differences.is_empty() {
        let mut diffs = "\n".to_string();
        for diff in differences {
            diffs += &diff.to_string();
            diffs += "\n";
        }
        println!("{}", diffs);
        assert!(false);
        //assert_eq!(result_file, html);
    }
}

fn read_file(filename: &str) -> String {
    let mut string = String::new();
    let mut file = File::open(format!("tests/{}", filename)).unwrap();
    file.read_to_string(&mut string).unwrap();
    string
}
