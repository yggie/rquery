use rquery::{ Document, Element };

pub fn new_document() -> Document {
    Document::new_from_xml_string(r#"
<?xml version="1.0" encoding="UTF-8"?>
<sample type="simple">
  This is some text
  <!-- This is a comment -->
  <title>Simple Sample</title>
  <note long="false">Some unrecognisable scribbling</note>

  <related>
    <!-- This is another comment -->
    <item index="1">
      <title>Another Sample</title>
      <ref>http://path.to.somewhere</ref>
    </item>

    <item index="2">
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
"#).unwrap()
}

#[test]
fn it_supports_the_tag_selector() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("note").unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.tag_name(), "note");
}

#[test]
fn it_supports_the_nested_tag_selector() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("related title").unwrap().collect();

    assert_eq!(elements.len(), 2);

    let element_tag_names: Vec<String> = elements.iter()
        .map(|el| el.tag_name().to_string())
        .collect();
    assert_eq!(element_tag_names, vec!("title", "title"));
}

#[test]
fn it_supports_nesting_selectors() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("related").unwrap()
        .flat_map(|element| element.select_all("title").unwrap())
        .collect();

    assert_eq!(elements.len(), 2);

    let element_tag_names: Vec<String> = elements.iter()
        .map(|el| el.tag_name().to_string())
        .collect();
    assert_eq!(element_tag_names, vec!("title", "title"));
}

#[test]
fn it_supports_the_direct_child_tag_selector() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("sample > title").unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.tag_name(), "title");
}

#[test]
fn it_does_not_repeat_elements() {
    let document = new_document();

    let unique_count = document.select_all("div").unwrap().count();
    assert_eq!(unique_count, 8);

    // TODO fix this
    // let direct_nested_count = document.select_all("div > div").unwrap().count();
    // assert_eq!(unique_count, 6);
    //
    // let nested_count = document.select_all("div div").unwrap().count();
    // assert_eq!(unique_count, 7);
}
