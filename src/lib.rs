extern crate xml;

mod document;

pub use self::document::Document;

use std::rc::Rc;
use std::iter::{ empty, once };

pub struct Node {
    children: Option<Vec<Rc<Node>>>,
    element: Element,
}

impl Node {
    pub fn select_all<'a>(&'a self, selector: &'a str) -> Box<Iterator<Item=&'a Element> + 'a> {
        let initial_iterator: Box<Iterator<Item=&'a Node> + 'a> = Box::new(once(self));

        let iterator = selector.split_whitespace()
            .fold(initial_iterator, |iter, selector_part| {
                Box::new(iter
                    .flat_map(|child| child.children_deep_iter())
                    .filter_map(move |child| {
                        if child.element.tag_name() == selector_part {
                            Some(child)
                        } else {
                            None
                        }
                    }))
            })
            .map(|child| &child.element);

        return Box::new(iterator);
    }

    pub fn children_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Node> + 'a> {
        if let Some(ref children) = self.children {
            Box::new(children.iter().map(|node| -> &'a Node { node }))
        } else {
            Box::new(empty::<&'a Node>())
        }
    }

    pub fn children_deep_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Node> + 'a> {
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
}

pub struct Element {
    tag_name: String
}

impl Element {
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }
}
