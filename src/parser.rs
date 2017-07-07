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
use error::{Error, Result};
use lexer::Lexer;
use node::{Item, Node, Text};
use node::Node::*;
use token::Token;
use token::Token::*;

/// Type of node to parse.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Type {
    HorizontalRule,
    NewLine,
    NumberSign,
    PageBreak,
    Space,
    TokErr,
    Underscore,
    Word,
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

    /// Eat the expected token or return an error if a different token is found.
    fn eat(&mut self, expected: Token) -> Result<()> {
        if let Some(token) = self.tokens.next() {
            let token = token?;
            if token != expected {
                bail!("expected to eat token {:?}, but found {:?}", expected, token);
            }
        }
        else {
            bail!("expected to eat token {:?}, but found none", expected);
        }
        Ok(())
    }

    /// Parse an horizontal rule.
    fn horizontal_rule(&mut self) -> Result<Node> {
        self.eat(TripleApos)?;
        Ok(HorizontalRule)
    }

    /// Parse an italic text.
    fn italic(&mut self) -> Result<Item> {
        // TODO: refactor to be able to reuse this part and others like mark, bold…
        self.eat(Underscore)?;
        let text = self.text_while(|token| token != Type::Underscore)?;
        self.eat(Underscore)?;
        Ok(Item::Italic(text))
    }

    /// Parse a mark.
    fn mark(&mut self) -> Result<Item> {
        self.eat(NumberSign)?;
        let text = self.text_while(|token| token != Type::NumberSign)?;
        self.eat(NumberSign)?;
        Ok(Item::Mark(text))
    }

    /// An iterator over the nodes of the document.
    pub fn node(&mut self) -> Result<Node> {
        let ty = self.node_type()?;
        match ty {
            Type::HorizontalRule => self.horizontal_rule(),
            Type::PageBreak => self.page_break(),
            Type::NewLine | Type::Space => {
                self.tokens.next();
                self.node()
            },
            Type::NumberSign | Type::Underscore | Type::Word => self.paragraph(),
            Type::TokErr => {
                if let Some(Err(err)) = self.tokens.next() {
                    bail!(err);
                }
                else {
                    bail!("cannot reach non-error branch");
                }
            },
        }
    }

    /// Get the node type.
    // TODO: find a way to satisfy the borrow checker and remove this node type.
    fn node_type(&mut self) -> Result<Type> {
        let ty =
            match self.tokens.peek() {
                Some(&Ok(ref token)) =>
                    match *token {
                        NewLine => Type::NewLine,
                        NumberSign => Type::NumberSign,
                        Space => Type::Space,
                        TripleApos => Type::HorizontalRule,
                        TripleLt => Type::PageBreak,
                        Underscore => Type::Underscore,
                        Word(_) => Type::Word,
                    },
                Some(&Err(Error(Eof, _))) => bail!(Eof),
                Some(&Err(_)) => Type::TokErr,
                None => bail!(Eof),
            };
        Ok(ty)
    }

    /// Parse a page break
    fn page_break(&mut self) -> Result<Node> {
        self.eat(TripleLt)?;
        Ok(PageBreak)
    }

    /// Parse a paragraph.
    fn paragraph(&mut self) -> Result<Node> {
        let mut items = vec![];
        loop {
            let mut line = self.text_while(|node_type| node_type != Type::NewLine)?;
            // End of paragraph on an empty line.
            if line.items.is_empty() {
                break;
            }
            items.append(&mut line.items);
        }
        Ok(Paragraph(Text::new(items)))
    }

    /// Parse a space.
    fn space(&mut self) -> Result<Item> {
        self.eat(Space)?;
        Ok(Item::Space)
    }

    /// Parse text while the predicate returns true.
    fn text_while<F: Fn(Type) -> bool>(&mut self, predicate: F) -> Result<Text> {
        let mut items = vec![];
        loop {
            let node_type = self.node_type()?;
            if !predicate(node_type) {
                break;
            }
            let item =
                match node_type {
                    Type::NewLine => {
                        self.eat(NewLine)?;
                        continue;
                    },
                    Type::NumberSign => self.mark()?,
                    Type::Space => self.space()?,
                    Type::Underscore => self.italic()?,
                    Type::Word => self.word()?,
                    _ => bail!("Should have got text token, but got {:?}", node_type),
                };
            items.push(item);
        }
        Ok(Text::new(items))
    }

    /// Parse a single word.
    fn word(&mut self) -> Result<Item> {
        if let Some(Ok(Word(bytes))) = self.tokens.next() {
            Ok(Item::Word(String::from_utf8(bytes)?))
        }
        else {
            bail!("Should have got word token");
        }
    }
}
