use forest;

pub fn new_document() -> forest::Document {
    forest::Document::new_from_file("tests/fixtures/simple_sample.xml").unwrap()
}


#[test]
fn it_captures_the_correct_number_of_elements() {
    let document = new_document();

    assert_eq!(document.number_of_elements(), 10);
}

#[test]
fn it_returns_an_error_for_non_existent_files() {
    let result = forest::Document::new_from_file("non-existent.why");

    assert!(result.is_err());
}
