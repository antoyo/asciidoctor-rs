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

//! Generate HTML from the asciidoctor nodes.

use std::io::Write;

use error::Result;
use node::Node;
use node::Node::*;

macro_rules! attr {
    ($( $name:ident = $value:expr ),*) => {{
        let mut attributes = String::new();
        $(
            attributes.push_str(stringify!($name));
            attributes.push_str("=\"");
            attributes.push_str(&$value.to_string());
            attributes.push_str("\"");
        )*
        attributes
    }};
}

/// Write the resulting HTML code for the specified `node` in the `writer`.
pub fn gen<G: HtmlGen<W>, W: Write>(gen: &mut G, node: &Node) -> Result<()> {
    gen.node(node)
}

pub struct Generator<'a, W: Write + 'a> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> Generator<'a, W> {
    /// Create a new HTML generator.
    pub fn new(writer: &'a mut W) -> Self {
        Generator {
            writer,
        }
    }
}

pub trait HtmlGen<W: Write> {
    fn node(&mut self, node: &Node) -> Result<()> {
        match *node {
            PageBreak => self.page_break(),
            Paragraph(ref text) => self.paragraph(text),
        }
    }

    fn page_break(&mut self) -> Result<()> {
        self.write(div_a(
            attr! { style = "page-break-after: always;" },
            empty()
        ))
    }

    fn paragraph(&mut self, text: &str) -> Result<()> {
        self.write(div_a(
            attr! { class = "paragraph" },
            p(text_node(text)),
        ))
    }

    fn write(&mut self, string: String) -> Result<()>;
}

impl<'a, W: Write + 'a> HtmlGen<W> for Generator<'a, W> {
    fn write(&mut self, string: String) -> Result<()> {
        write!(self.writer, "{}", string)?;
        Ok(())
    }
}

fn div_a(attributes: String, children: String) -> String {
    tag_a("div", &attributes, &children)
}

fn empty() -> String {
    String::new()
}

fn p(children: String) -> String {
    tag("p", &children)
}

fn p_a(attributes: String, children: String) -> String {
    tag_a("p", &attributes, &children)
}

fn tag(tag: &str, children: &str) -> String {
    format!("<{}>{}</{}>", tag, children, tag)
}

fn tag_a(tag: &str, attributes: &str, children: &str) -> String {
    format!("<{} {}>{}</{}>", tag, attributes, children, tag)
}

fn text_node(text: &str) -> String {
    text.to_string()
}
