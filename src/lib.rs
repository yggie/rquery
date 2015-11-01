//! This library offers the ability to represent XML documents as DOM trees,
//! allowing querying with CSS selectors.
//!
//! ```
//! extern crate rquery;
//!
//! use rquery::Document;
//!
//! fn main() {
//!   let document = Document::new_from_xml_file("tests/fixtures/sample.xml").unwrap();
//!
//!   let title = document.select("title").unwrap();
//!   assert_eq!(title.text(), "Sample Document");
//!   assert_eq!(title.attr("ref").unwrap(), "main-title");
//!
//!   let item_count = document.select_all("item").unwrap().count();
//!   assert_eq!(item_count, 2);
//!
//!   let item_titles = document.select_all("item > title").unwrap()
//!     .map(|element| element.text().clone())
//!     .collect::<Vec<String>>()
//!     .join(", ");
//!   assert_eq!(item_titles, "Another Sample, Other Sample");
//! }
//! ```

#![warn(missing_docs)]

extern crate xml;

mod selector;
mod document;

pub use self::document::Document;
pub use self::selector::{ CompoundSelector, Scope, Selector };

use std::rc::Rc;
use std::iter::{ empty, once };
use std::collections::HashMap;

/// Represents a single element in the DOM tree.
pub struct Element {
    tag_name: String,
    children: Option<Vec<Rc<Element>>>,
    attr_map: HashMap<String, String>,
    text: String,
}

impl Element {
    /// Searches the elements children for elements matching the given CSS
    /// selector.
    pub fn select_all<'a>(&'a self, selector: &'a str) -> Result<Box<Iterator<Item=&'a Element> + 'a>, ()> {
        CompoundSelector::parse(selector).and_then(|compound_selectors| {
            let initial_iterator: Box<Iterator<Item=&'a Element>> = Box::new(once(self));

            let iterator = compound_selectors.into_iter()
                .fold(initial_iterator, |iter, compound_selector| {
                    let scope = compound_selector.scope;

                    Box::new(iter
                         .flat_map(move |child| {
                             match scope {
                                 Scope::IndirectChild => child.children_deep_iter(),
                                 Scope::DirectChild => child.children_iter(),
                             }
                         })
                        .filter_map(move |child| {
                            if child.matches(&compound_selector) {
                                Some(child)
                            } else {
                                None
                            }
                        }))
                });

            return Ok(iterator);
        })
    }

    /// Just like `select_all` but only returns the first match.
    pub fn select<'a>(&'a self, selector: &'a str) -> Result<&'a Element, ()> {
        self.select_all(selector).and_then(|mut iterator| {
            if let Some(element) = iterator.next() {
                Ok(element)
            } else {
                Err(())
            }
        })
    }

    /// Returns an iterator over the element’s direct children.
    pub fn children_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Element> + 'a> {
        if let Some(ref children) = self.children {
            Box::new(children.iter().map(|node| -> &'a Element { node }))
        } else {
            Box::new(empty::<&'a Element>())
        }
    }

    /// Returns an iterator over all the element’s children, including indirect
    /// child elements.
    pub fn children_deep_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Element> + 'a> {
        let iterator = self.children_iter()
            .flat_map(|child| once(child).chain(child.children_deep_iter()));

        Box::new(iterator)
    }

    /// Returns the size of the DOM subtree, including the current element.
    pub fn subtree_size(&self) -> usize {
        if let Some(ref children) = self.children {
            children.iter().fold(1, |subtotal, child| child.subtree_size() + subtotal)
        } else {
            1
        }
    }

    /// Returns the name of the element’s tag.
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    /// Returns the value of the element attribute if found.
    pub fn attr(&self, attr_name: &str) -> Option<&String> {
        self.attr_map.get(attr_name)
    }

    /// Returns the text contained within the element.
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Returns true if the element matches the given selector.
    pub fn matches(&self, compound_selector: &CompoundSelector) -> bool {
        match compound_selector.parts.last() {
            Some(&Selector::TagName(ref name)) => self.tag_name() == name,
            _ => unimplemented!(),
        }
    }
}
