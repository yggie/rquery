use std::io::BufReader;
use std::fs::File;
use std::rc::Rc;
use std::path::Path;
use std::collections::HashMap;

use xml::reader::{ EventReader, XmlEvent };

use super::Element;

#[derive(Debug)]
pub enum DocumentError {
    InvalidFileExtension(String),
    UnableToOpenFile(String),
    ParseError(String),
}

pub struct Document {
    root: Element,
}

impl Document {
    pub fn new_from_file(filename: &str) -> Result<Document, DocumentError> {
        let path = Path::new(filename);

        if let Some(os_string) = path.extension() {
            match os_string.to_str() {
                Some("xml") => Document::new_from_xml_file(path),
                Some(other) => Err(DocumentError::InvalidFileExtension(other.to_string())),
                _ => unimplemented!(),
            }
        } else {
            Err(DocumentError::InvalidFileExtension("".to_string()))
        }
    }

    fn create_event_reader(path: &Path) -> Result<EventReader<BufReader<File>>, DocumentError> {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            Ok(EventReader::new(reader))
        } else {
            Err(DocumentError::UnableToOpenFile(path.to_str().unwrap().to_string()))
        }
    }

    fn new_from_xml_file(path: &Path) -> Result<Document, DocumentError> {
        Document::create_event_reader(path).and_then(|event_reader| {
            let mut elements: Vec<Element> = Vec::new();

            for event in event_reader {
                match event {
                    Ok(XmlEvent::StartElement { ref name, ref attributes, .. }) => {
                        let attr_map = attributes.iter()
                            .fold(HashMap::new(), |mut hash_map, attribute| {
                                hash_map.insert(attribute.name.local_name.clone(), attribute.value.clone());

                                return hash_map;
                            });

                        elements.push(Element {
                            children: None,
                            tag_name: name.local_name.clone(),
                            attr_map: attr_map,
                            text: String::new(),
                        });
                    },

                    Ok(XmlEvent::EndElement { ref name, .. }) if elements.last().unwrap().tag_name() == name.local_name  => {
                        let child_node = elements.pop().unwrap();

                        if let Some(mut parent) = elements.pop() {
                            if let Some(ref mut children) = parent.children {
                                children.push(Rc::new(child_node));
                            } else {
                                parent.children = Some(vec!(Rc::new(child_node)));
                            }

                            elements.push(parent);
                        } else {
                            return Ok(Document {
                                root: child_node
                            });
                        }
                    },

                    Ok(XmlEvent::Characters(string)) => {
                        elements.last_mut().unwrap().text.push_str(&string);
                    },

                    Ok(XmlEvent::Whitespace(string)) => {
                        elements.last_mut().unwrap().text.push_str(&string);
                    },

                    Err(error) => {
                        return Err(DocumentError::ParseError(error.to_string()));
                    },

                    Ok(_) => { },
                }
            }

            panic!("Root element was not properly returned!");
        })
    }

    pub fn root(&self) -> &Element {
        &self.root
    }

    pub fn number_of_elements(&self) -> usize {
        self.root.subtree_size()
    }

    pub fn select_all<'a>(&'a self, selector: &'a str) -> Result<Box<Iterator<Item=&'a Element> + 'a>, ()> {
        self.root.select_all(selector)
    }
}
