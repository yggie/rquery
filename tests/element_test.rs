use rquery::Document;

fn new_document() -> Document {
    Document::new_from_xml_string(r#"
<?xml version="1.0" encoding="UTF-8"?>
<main type="simple">
  This is some text
</main>
"#).unwrap()
}


#[test]
fn it_knows_its_tag_name() {
    let document = new_document();

    let element = document.select("main").unwrap();
    assert_eq!(element.tag_name(), "main");
}

#[test]
fn it_knows_its_attributes() {
    let document = new_document();

    let element = document.select("main").unwrap();
    assert_eq!(element.attr("type").unwrap(), "simple");
}

#[test]
fn it_knows_its_inner_text_contents() {
    let document = new_document();


    let element = document.select("main").unwrap();
    assert_eq!(element.text().trim(), "This is some text");
}
