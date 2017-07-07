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

//! Parse asciidoctor.

use std::io::BufRead;
use std::iter::Peekable;

use error::ErrorKind::Eof;
use error::Result;
use lexer::Lexer;
use node::Node;
use node::Node::*;
use self::NodeType::*;
use token::Token::*;

/// Type of node to parse.
#[derive(Debug)]
enum NodeType {
    HorRule,
    PageBrk,
    Par,
    TokErr,
}

/// Asciidoctor parser.
pub struct Parser<R: BufRead> {
    tokens: Peekable<Lexer<R>>,
}

impl<R: BufRead> Parser<R> {
    /// Create a new parser from an iterator of tokens.
    /// The resulting nodes can be fetched by calling `Parser::nodes()` which is an iterator over
    /// asciidoctor nodes.
    pub fn new(tokens: Lexer<R>) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    /// Parse an horizontal rule.
    fn horizontal_rule(&mut self) -> Result<Node> {
        self.tokens.next(); // TODO: use a macro to eat.
        Ok(HorizontalRule)
    }

    /// An iterator over the nodes of the document.
    pub fn nodes(&mut self) -> Result<Node> {
        let ty = self.node_type()?;
        match ty {
            HorRule => self.horizontal_rule(),
            Par => self.paragraph(),
            PageBrk => self.page_break(),
            TokErr => {
                if let Some(Err(err)) = self.tokens.next() {
                    bail!(err);
                }
                else {
                    bail!("cannot reach non-error branch");
                }
            },
        }
    }

    fn node_type(&mut self) -> Result<NodeType> {
        let ty =
            match self.tokens.peek() {
                Some(&Ok(ref token)) =>
                    match *token {
                        Text(_) => Par,
                        TripleApos => HorRule,
                        TripleLt => PageBrk,
                    },
                Some(&Err(_)) => TokErr,
                None => bail!(Eof),
            };
        Ok(ty)
    }

    /// Parse a page break
    fn page_break(&mut self) -> Result<Node> {
        self.tokens.next(); // TODO: use a macro to eat.
        Ok(PageBreak)
    }

    /// Parse a paragraph.
    fn paragraph(&mut self) -> Result<Node> {
        let mut string = String::new();
        // TODO: use a macro to eat.
        while let Par = self.node_type()? {
            if let Some(Ok(Text(bytes))) = self.tokens.next() {
                string.push_str(&String::from_utf8(bytes)?);
            }
            else {
                bail!("Should have got text token");
            }
        }
        Ok(Paragraph(string))
    }
}
