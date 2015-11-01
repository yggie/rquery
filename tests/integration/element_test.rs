use forest;

pub fn new_document() -> forest::Document {
    forest::Document::new_from_file("tests/fixtures/simple_sample.xml").unwrap()
}


#[test]
fn it_knows_its_tag_name() {
    let document = new_document();

    let element = document.select("sample").unwrap();
    assert_eq!(element.tag_name(), "sample");
}

#[test]
fn it_knows_its_attributes() {
    let document = new_document();

    let element = document.select("sample").unwrap();
    assert_eq!(element.attr("type").unwrap(), "simple");
}

#[test]
fn it_knows_its_inner_text_contents() {
    let document = new_document();


    let element = document.select("sample").unwrap();
    assert_eq!(element.text().trim(), "This is some text");
}
