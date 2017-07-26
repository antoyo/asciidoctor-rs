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

use error::{Error, Result};
use error::ErrorKind::UnexpectedToken;
use lexer::Lexer;
use node::{Attribute, Item, Node, Text};
use node::Attribute::Role;
use node::Node::*;
use node::Tag::{Bold, InlineCode, Italic, SuperScript};
use token::Token;
use token::Token::*;

macro_rules! parse_text_between {
    ($func_name:ident, $token:ident, $tag:ident) => {
        fn $func_name(&mut self, attributes: Vec<Attribute>) -> Result<Item> {
            let text = text_between!(self, $token);
            Ok(Item::Tag($tag, text, attributes))
        }
    };
}

macro_rules! text_between {
    ($_self:expr, $token:ident) => {{
        $_self.eat($token)?;
        let text = $_self.text_while(|token| token != Type::$token)?;
        $_self.eat($token)?;
        text
    }};
}

/// Type of node to parse.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Type {
    Backquote,
    Caret,
    CloseSquareBracket,
    HorizontalRule,
    NewLine,
    NumberSign,
    OpenSquareBracket,
    PageBreak,
    Space,
    Star,
    Underscore,
    Word,
}

/// Asciidoctor parser.
pub struct Parser<R: BufRead> {
    tokens: Lexer<R>,
}

impl<R: BufRead> Parser<R> {
    /// Create a new parser from an iterator of tokens.
    /// The resulting nodes can be fetched by calling `Parser::nodes()` which is an iterator over
    /// asciidoctor nodes.
    pub fn new(tokens: Lexer<R>) -> Self {
        Parser {
            tokens,
        }
    }

    /// Parse an attribute.
    fn attribute(&mut self) -> Result<Attribute> {
        let attribute =
            match self.tokens.token()? {
                Word(word) => Role(String::from_utf8(word)?),
                _ => bail!(self.unexpected_token("ident")),
            };
        Ok(attribute)
    }

    /// Parse attributes and the node following it.
    fn attributes(&mut self) -> Result<Vec<Attribute>> {
        let mut attributes = vec![];
        if self.node_type()? == Type::OpenSquareBracket {
            self.eat(OpenSquareBracket)?;
            attributes.push(self.attribute()?);
            // TODO: other attributes.
            self.eat(CloseSquareBracket)?;
        }
        Ok(attributes)
    }

    /// Eat the expected token or return an error if a different token is found.
    fn eat(&mut self, expected: Token) -> Result<()> {
        let token = self.tokens.token()?;
        if token != expected {
            bail!(self.unexpected_token(&expected.to_string()));
        }
        Ok(())
    }

    /// Parse an horizontal rule.
    fn horizontal_rule(&mut self) -> Result<Node> {
        self.eat(TripleApos)?;
        Ok(HorizontalRule)
    }

    parse_text_between!(bold, Star, Bold);
    parse_text_between!(inline_code, Backquote, InlineCode);
    parse_text_between!(italic, Underscore, Italic);
    parse_text_between!(superscript, Caret, SuperScript);

    /// Parse a mark.
    fn mark(&mut self, attributes: Vec<Attribute>) -> Result<Item> {
        let text = text_between!(self, NumberSign);
        Ok(Item::Mark(text, attributes))
    }

    /// An iterator over the nodes of the document.
    pub fn node(&mut self) -> Result<Node> {
        let ty = self.node_type()?;
        match ty {
            Type::HorizontalRule => self.horizontal_rule(),
            Type::PageBreak => self.page_break(),
            Type::NewLine | Type::Space => {
                self.tokens.token()?;
                self.node()
            },
            Type::Backquote | Type::Caret | Type::CloseSquareBracket | Type::NumberSign | Type::OpenSquareBracket |
                Type::Star | Type::Underscore | Type::Word =>
                self.paragraph(),
        }
    }

    /// Get the node type.
    // TODO: find a way to satisfy the borrow checker and remove this node type.
    fn node_type(&mut self) -> Result<Type> {
        let ty =
            match *self.tokens.peek()? {
                Backquote => Type::Backquote,
                Caret => Type::Caret,
                CloseSquareBracket => Type::CloseSquareBracket,
                NewLine => Type::NewLine,
                NumberSign => Type::NumberSign,
                OpenSquareBracket => Type::OpenSquareBracket,
                Space => Type::Space,
                Star => Type::Star,
                TripleApos => Type::HorizontalRule,
                TripleLt => Type::PageBreak,
                Underscore => Type::Underscore,
                Word(_) => Type::Word,
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

    /// Parse a text item.
    fn text_item(&mut self, attributes: Vec<Attribute>) -> Result<Item> {
        let node_type = self.node_type()?;
        let item =
            match node_type {
                Type::Backquote => self.inline_code(attributes)?,
                Type::Caret => self.superscript(attributes)?,
                Type::NumberSign => self.mark(attributes)?,
                Type::OpenSquareBracket => {
                    if !attributes.is_empty() {
                        bail!(self.unexpected_token("["));
                    }
                    let attributes = self.attributes()?;
                    self.text_item(attributes)?
                },
                Type::Space => self.space()?,
                Type::Star => self.bold(attributes)?,
                Type::Underscore => self.italic(attributes)?,
                Type::Word => self.word()?,
                _ => bail!("Should have got text token, but got {:?}", node_type), // TODO: better error.
            };
        Ok(item)
    }

    /// Parse text while the predicate returns true.
    fn text_while<F: Fn(Type) -> bool>(&mut self, predicate: F) -> Result<Text> {
        let mut items = vec![];
        loop {
            let node_type = self.node_type()?;
            if !predicate(node_type) {
                break;
            }
            if node_type == Type::NewLine {
                self.eat(NewLine)?;
                continue;
            }
            let item = self.text_item(vec![])?;
            items.push(item);
        }
        Ok(Text::new(items))
    }

    /// Return an UnexpectedToken error.
    fn unexpected_token(&mut self, expected: &str) -> Error {
        let actual = self.tokens.peek()
            .map(|token| token.to_string())
            .unwrap_or_else(|_| "(unknown token)".to_string());
        UnexpectedToken {
            actual,
            expected: expected.to_string(),
            pos: self.tokens.pos(),
        }.into()
    }

    /// Parse a single word.
    fn word(&mut self) -> Result<Item> {
        if let Ok(Word(bytes)) = self.tokens.token() {
            Ok(Item::Word(String::from_utf8(bytes)?))
        }
        else {
            bail!("Should have got word token"); // TODO: better error.
        }
    }
}
