use rquery::Document;

fn new_document() -> Document {
    Document::new_from_xml_string(r#"
<?xml version="1.0" encoding="UTF-8"?>
<body>
  <one/>
  <two/>
  <three/>
</body>
"#).unwrap()
}

#[test]
fn it_captures_the_correct_number_of_elements() {
    let document = new_document();

    assert_eq!(document.number_of_elements(), 4);
}
