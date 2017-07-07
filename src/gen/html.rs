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

use node::Node;
use node::Node::*;
use node::Text;
use node::Item;

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
pub fn gen<G: HtmlGen>(gen: &mut G, node: &Node) -> String {
    gen.node(node)
}

pub struct Generator {
}

pub trait HtmlGen {
    fn node(&mut self, node: &Node) -> String {
        match *node {
            HorizontalRule => self.horizontal_rule(),
            Mark(ref text) => self.mark(&text),
            PageBreak => self.page_break(),
            Paragraph(ref text) => self.paragraph(&text),
        }
    }

    fn horizontal_rule(&mut self) -> String {
        hr()
    }

    fn item(&mut self, item: &Item) -> String {
        match *item {
            Item::Space => " ".to_string(),
            Item::Word(ref text) => text.clone(),
        }
    }

    fn mark(&mut self, text: &Text) -> String {
        let text = self.text(text);
        mark(text)
    }

    fn page_break(&mut self) -> String {
        div_a(
            attr! { style = "page-break-after: always;" },
            empty()
        )
    }

    fn paragraph(&mut self, text: &Text) -> String {
        let text = self.text(text);
        div_a(
            attr! { class = "paragraph" },
            p(text),
        )
    }

    fn text(&mut self, text: &Text) -> String {
        let mut string = String::new();
        for item in &text.items {
            string.push_str(&self.item(item));
        }
        string
    }
}

impl HtmlGen for Generator {}

fn div_a(attributes: String, children: String) -> String {
    tag_a("div", &attributes, &children)
}

fn empty() -> String {
    String::new()
}

fn hr() -> String {
    tag("hr", "")
}

fn mark(children: String) -> String {
    tag("mark", &children)
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
