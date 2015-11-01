use forest;

pub fn new_document() -> forest::Document {
    forest::Document::new_from_file("tests/fixtures/simple_sample.xml").unwrap()
}

#[test]
fn it_supports_the_tag_selector() {
    let document = new_document();

    let elements: Vec<&forest::Element> = document.select_all("note").unwrap().collect();

    assert_eq!(elements.len(), 1);

    let element = elements[0];
    assert_eq!(element.tag_name(), "note");
}

#[test]
fn it_supports_the_nested_tag_selector() {
    let document = new_document();

    let elements: Vec<&forest::Element> = document.select_all("related title").unwrap().collect();

    assert_eq!(elements.len(), 2);

    let element_tag_names: Vec<String> = elements.iter()
        .map(|el| el.tag_name().to_string())
        .collect();
    assert_eq!(element_tag_names, vec!("title", "title"));
}

#[test]
fn it_supports_nesting_selectors() {
    let document = new_document();

    let elements: Vec<&forest::Element> = document.select_all("related").unwrap()
        .flat_map(|element| element.select_all("title").unwrap())
        .collect();

    assert_eq!(elements.len(), 2);

    let element_tag_names: Vec<String> = elements.iter()
        .map(|el| el.tag_name().to_string())
        .collect();
    assert_eq!(element_tag_names, vec!("title", "title"));
}

// #[test]
// fn it_supports_the_direct_child_tag_selector() {
//     let document = new_document();
//
//     let elements: Vec<&forest::Element> = document.select_all("sample > title").unwrap().collect();
//
//     assert_eq!(elements.len(), 1);
//
//     let element = elements[0];
//     assert_eq!(element.tag_name(), "title");
// }
