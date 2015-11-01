use rquery::Document;

#[test]
fn it_captures_the_correct_number_of_elements() {
    let result = Document::new_from_xml_string(r#"
<?xml version="1.0" encoding="UTF-8"?>
<body>
  <one/>
  <two/>
  <three/>
</body>
"#);

    assert!(result.is_ok());
}

#[test]
fn it_can_be_created_from_a_file() {
    let result = Document::new_from_xml_file("tests/fixtures/sample.xml");

    assert!(result.is_ok());
}

#[test]
fn it_returns_an_error_for_non_existent_files() {
    let result = Document::new_from_xml_file("non-existent.why");

    assert!(result.is_err());
}

#[test]
fn it_returns_an_error_for_invalid_xml_files() {
    let result = Document::new_from_xml_file("non-existent.why");

    assert!(result.is_err());
}
