use std::io::BufReader;
use std::fs::File;
use std::rc::Rc;
use std::path::Path;

use xml::reader::{ EventReader, XmlEvent };

use super::{ Node, Element };

#[derive(Debug)]
pub enum DocumentError {
    InvalidFileExtension(String),
    UnableToOpenFile(String),
    ParseError(String),
}

pub struct Document {
    root: Node,
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
            let mut nodes: Vec<Node> = Vec::new();

            for event in event_reader {
                match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) => {
                        nodes.push(Node {
                            children: None,
                            element: Element {
                                tag_name: name.local_name.clone()
                            },
                        });
                    },

                    Ok(XmlEvent::EndElement { ref name, .. }) if nodes[nodes.len() - 1].element.tag_name() == name.local_name  => {
                        let child_node = nodes.pop().unwrap();

                        if let Some(mut parent) = nodes.pop() {
                            if let Some(ref mut children) = parent.children {
                                children.push(Rc::new(child_node));
                            } else {
                                parent.children = Some(vec!(Rc::new(child_node)));
                            }

                            nodes.push(parent);
                        } else {
                            return Ok(Document {
                                root: child_node
                            });
                        }
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

    pub fn number_of_elements(&self) -> usize {
        self.root.subtree_size()
    }

    pub fn select_all<'a>(&'a self, selector: &'a str) -> Box<Iterator<Item=&'a Element> + 'a> {
        self.root.select_all(selector)
    }
}

// pub struct Element {
//     pub name: String,
//     pub attrs: HashMap<String, String>,
//     pub children: Vec<Rc<Element>>,
// }
