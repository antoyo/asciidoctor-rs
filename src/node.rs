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

/// An attribute like a role or an ID.
#[derive(Debug)]
pub enum Attribute {
    //Id(String),
    Role(String),
}

/// This is a recursive node structure that represents part of a asciidoctor document.
#[derive(Debug)]
pub enum Node {
    HorizontalRule,
    PageBreak,
    Paragraph(Text),
}

/// A text contains words, links, bold text, …
#[derive(Debug)]
pub struct Text {
    pub items: Vec<Item>,
}

impl Text {
    pub fn new(items: Vec<Item>) -> Self {
        Text {
            items,
        }
    }
}

/// A text item, like a word, link, bold text, …
#[derive(Debug)]
pub enum Item {
    //Bold(Box<Text>),
    Italic(Text, Vec<Attribute>),
    Mark(Text),
    Space,
    Word(String),
}
