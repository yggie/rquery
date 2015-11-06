use std::io::{ BufReader, Read };
use std::fs::File;
use std::rc::Rc;
use std::path::Path;
use std::collections::HashMap;

use xml::reader::{ EventReader, XmlEvent };

use super::{ Element, SelectError };

/// The various errors that can happen when creating a document.
#[derive(Clone, Debug)]
pub enum DocumentError {
    UnableToOpenFile(String),
    ParseError(String),
}

/// The DOM tree representation of the parsed document.
#[derive(Clone, Debug)]
pub struct Document {
    root: Element,
}

impl Document {
    /// Creates a new document from a byte stream.
    pub fn new_from_xml_stream<R: Read>(stream: R) -> Result<Document, DocumentError> {
        let event_reader = EventReader::new(stream);

        let mut elements: Vec<Element> = Vec::new();
        let mut next_node_index = 1;

        for event in event_reader {
            match event {
                Ok(XmlEvent::StartElement { ref name, ref attributes, .. }) => {
                    let attr_map = attributes.iter()
                        .fold(HashMap::new(), |mut hash_map, attribute| {
                            hash_map.insert(attribute.name.local_name.clone(), attribute.value.clone());

                            return hash_map;
                        });

                    elements.push(Element {
                        node_index: next_node_index,
                        children: None,
                        tag_name: name.local_name.clone(),
                        attr_map: attr_map,
                        text: String::new(),
                    });
                    next_node_index = next_node_index + 1;
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
                            root: Element {
                                node_index: 0,
                                tag_name: "[root]".to_string(),
                                children: Some(vec!(Rc::new(child_node))),
                                attr_map: HashMap::new(),
                                text: String::new(),
                            }
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
    }

    /// Creates a new document from a string.
    pub fn new_from_xml_string(string: &str) -> Result<Document, DocumentError> {
        Document::new_from_xml_stream(string.as_bytes())
    }

    /// Creates a new document from a file.
    pub fn new_from_xml_file(filename: &str) -> Result<Document, DocumentError> {
        let path = Path::new(filename);

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            Document::new_from_xml_stream(reader)
        } else {
            Err(DocumentError::UnableToOpenFile(path.to_str().unwrap().to_string()))
        }
    }

    /// Returns the total number of elements in the document.
    pub fn number_of_elements(&self) -> usize {
        self.root.subtree_size() - 1
    }

    /// Searches the document for elements matching the given CSS selector.
    pub fn select_all<'a>(&'a self, selector: &str) -> Result<Box<Iterator<Item=&'a Element> + 'a>, SelectError> {
        self.root.select_all(selector)
    }

    /// Just like `select_all` but only returns the first match.
    pub fn select<'a>(&'a self, selector: &str) -> Result<&'a Element, SelectError> {
        self.root.select(selector)
    }
}

#[test]
fn it_assigns_node_indices_in_monotonically_increasing_order() {
    let document = Document::new_from_xml_string(r#"
<?xml version="1.0" encoding="UTF-8"?>
<sample type="simple">
  This is some text
  <!-- This is a comment -->
  <title>Simple Sample</title>
  <note long="false">Some unrecognisable scribbling</note>

  <related>
    <!-- This is another comment -->
    <item id="1">
      <title>Another Sample</title>
      <ref>http://path.to.somewhere</ref>
    </item>

    <item id="2">
      <title>Other Sample</title>
      <ref>http://some.other.path</ref>
    </item>
  </related>

  <!-- div soup goodness -->
  <div></div>
  <div>
    <other>
      <div></div>
    </other>
    <div>
      <div></div>
      <div>
        <div></div>
        <div></div>
      </div>
    </div>
  </div>
</sample>
"#).unwrap();

    assert_eq!(document.root.node_index, 0);

    document.root.children_deep_iter().fold(0, |index, child| {
        assert!(index < child.node_index);
        child.node_index
    });
}
