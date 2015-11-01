extern crate xml;

mod document;

pub use self::document::Document;

use std::rc::Rc;
use std::iter::{ empty, once };
use std::collections::HashMap;

pub struct Element {
    tag_name: String,
    children: Option<Vec<Rc<Element>>>,
    attr_map: HashMap<String, String>,
    text: String,
}

impl Element {
    pub fn select_all<'a>(&'a self, selector: &'a str) -> Result<Box<Iterator<Item=&'a Element> + 'a>, ()> {
        let initial_iterator: Box<Iterator<Item=&'a Element>> = Box::new(once(self));

        let iterator = selector.split_whitespace()
            .fold(initial_iterator, |iter, selector_part| {
                Box::new(iter
                    .flat_map(|child| child.children_deep_iter())
                    .filter_map(move |child| {
                        if child.tag_name() == selector_part {
                            Some(child)
                        } else {
                            None
                        }
                    }))
            });

        return Ok(Box::new(iterator));
    }

    pub fn children_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Element> + 'a> {
        if let Some(ref children) = self.children {
            Box::new(children.iter().map(|node| -> &'a Element { node }))
        } else {
            Box::new(empty::<&'a Element>())
        }
    }

    pub fn children_deep_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Element> + 'a> {
        let iterator = self.children_iter()
            .flat_map(|child| once(child).chain(child.children_deep_iter()));

        Box::new(iterator)
    }

    pub fn subtree_size(&self) -> usize {
        if let Some(ref children) = self.children {
            children.iter().fold(1, |subtotal, child| child.subtree_size() + subtotal)
        } else {
            1
        }
    }

    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    pub fn attr(&self, attr_name: &str) -> Option<&String> {
        self.attr_map.get(attr_name)
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}
