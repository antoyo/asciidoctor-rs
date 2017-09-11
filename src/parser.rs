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
use lexer::Lexer;
use node::{Attribute, Item, Node, Text};
use node::Attribute::{Id, Role};
use node::Node::*;
use node::Tag::*;
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
        let text = $_self.text_while(|token| token != &$token)?;
        $_self.eat($token)?;
        text
    }};
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
                NumberSign => {
                    if let Word(word) = self.tokens.token()? {
                        Id(String::from_utf8(word)?)
                    } else {
                        return Err(self.unexpected_token("ident")) // FIXME: does not show the right actual token because it was consumed by the call to token().
                    }
                },
                Word(word) => Role(String::from_utf8(word)?),
                _ => return Err(self.unexpected_token("ident")), // FIXME: does not show the right actual token because it was consumed by the call to token().
            };
        Ok(attribute)
    }

    /// Parse attributes and the node following it.
    fn attributes(&mut self) -> Result<Vec<Attribute>> {
        let mut attributes = vec![];
        if *self.tokens.peek()? == OpenSquareBracket {
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
            return Err(self.unexpected_token(&expected.to_string())); // FIXME: does not show the right actual token because it was consumed by the call to token().
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
    parse_text_between!(subscript, Tilde, SubScript);
    parse_text_between!(superscript, Caret, SuperScript);
    parse_text_between!(unconstrained_bold, DoubleStar, Bold);
    parse_text_between!(unconstrained_inline_code, DoubleBackquote, InlineCode);
    parse_text_between!(unconstrained_italic, DoubleUnderscore, Italic);

    /// Parse a mark.
    fn mark(&mut self, attributes: Vec<Attribute>) -> Result<Item> {
        let text = text_between!(self, NumberSign);
        Ok(Item::Mark(text, attributes))
    }

    /// An iterator over the nodes of the document.
    pub fn node(&mut self) -> Result<Node> {
        let func =
            match *self.tokens.peek()? {
                TripleApos => Self::horizontal_rule,
                TripleLt => Self::page_break,
                NewLine | Space => {
                    self.tokens.token()?;
                    Self::node
                },
                Backquote | Caret | CloseSquareBracket | DoubleBackquote | DoubleStar |
                    DoubleUnderscore | NumberSign | OpenSquareBracket | Star | Tilde |
                    Underscore | Word(_) =>
                    Self::paragraph,
            };
        func(self)
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
            let mut line = self.text_while(|node| node != &NewLine)?;
            // End of paragraph on an empty line.
            if line.items.is_empty() {
                break;
            }
            items.append(&mut line.items);
        }
        Ok(Paragraph(Text::new(items)))
    }

    /// Parse a space.
    fn space(&mut self, _attributes: Vec<Attribute>) -> Result<Item> {
        self.eat(Space)?;
        Ok(Item::Space)
    }

    /// Parse a text item.
    fn text_item(&mut self, mut attributes: Vec<Attribute>) -> Result<Item> {
        if *self.tokens.peek()? == OpenSquareBracket {
            if !attributes.is_empty() {
                return Err(self.unexpected_token("["));
            }
            attributes = self.attributes()?;
        }
        let func =
            match *self.tokens.peek()? {
                Backquote => Self::inline_code,
                Caret => Self::superscript,
                DoubleBackquote => Self::unconstrained_inline_code,
                DoubleStar => Self::unconstrained_bold,
                DoubleUnderscore => Self::unconstrained_italic,
                NumberSign => Self::mark,
                OpenSquareBracket => Self::text_item,
                Space => Self::space,
                Star => Self::bold,
                Tilde => Self::subscript,
                Underscore => Self::italic,
                Word(_) => Self::word,
                ref node => return Err(Error::Msg(format!("Should have got text token, but got {:?}", node))), // TODO: better error.
            };
        let item = func(self, attributes)?;
        Ok(item)
    }

    /// Parse text while the predicate returns true.
    fn text_while<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Result<Text> {
        let mut items = vec![];
        loop {
            let is_newline = {
                let token = self.tokens.peek()?;
                if !predicate(token) {
                    break;
                }
                *token == NewLine
            };
            if is_newline {
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
        Error::UnexpectedToken {
            actual,
            expected: expected.to_string(),
            pos: self.tokens.pos(),
        }
    }

    /// Parse a single word.
    fn word(&mut self, _attributes: Vec<Attribute>) -> Result<Item> {
        if let Ok(Word(bytes)) = self.tokens.token() {
            Ok(Item::Word(String::from_utf8(bytes)?))
        }
        else {
            return Err(Error::Msg("Should have got word token".to_string())); // TODO: better error.
        }
    }
}
