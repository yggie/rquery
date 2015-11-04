use rquery::{ Document, Element, SelectError, UnexpectedTokenError };

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
    <item id="id-1">
      <title>Another Sample</title>
      <ref>http://path.to.somewhere</ref>
    </item>

    <item id="id-2">
      <title>Other Sample</title>
      <ref>http://some.other.path</ref>
    </item>
  </related>

  <!-- div soup goodness -->
  <div></div>
  <div type="one">
    <other type="three">
      <div type="two"></div>
    </other>
    <div>
      <div type="three"></div>
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
    assert_eq!(elements[0].tag_name(), "note");
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
fn it_returns_a_no_match_error_when_the_selector_does_not_match_any_element() {
    let document = new_document();

    let result = document.select("nonexistentelement");

    if let Err(err) = result {
        assert_eq!(err, SelectError::NoMatchError);
    } else {
        panic!("The select did not result in an error!");
    }
}

#[test]
fn it_returns_a_parse_error_when_the_selector_is_invalid() {
    let document = new_document();

    let result = document.select_all("?");

    if let Err(err) = result {
        assert_eq!(err, SelectError::ParseError(UnexpectedTokenError('?')));
    } else {
        panic!("The invalid selector did not result in an error!");
    }
}

#[test]
fn it_supports_the_attribute_selector() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all(r#"[long="false"]"#).unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.text(), "Some unrecognisable scribbling");
}

#[test]
fn it_supports_the_id_selector() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("#id-1").unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.tag_name(), "item");
    assert_eq!(element.attr("id"), Some(&"id-1".to_string()));
}

#[test]
fn it_supports_the_compound_selectors() {
    let document = new_document();

    let elements: Vec<&Element> = document.select_all("div[type=three]").unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.tag_name(), "div");
    assert_eq!(element.attr("type"), Some(&"three".to_string()));
}

#[test]
fn it_does_not_repeat_elements() {
    let document = new_document();

    let unique_count = document.select_all("div").unwrap().count();
    assert_eq!(unique_count, 8);

    let direct_nested_count = document.select_all("div > div").unwrap().count();
    assert_eq!(direct_nested_count, 5);

    let nested_count = document.select_all("div div").unwrap().count();
    assert_eq!(nested_count, 6);
}
