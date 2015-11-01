# rust-xml-query

[![Build Status](https://travis-ci.org/yggie/rust-xml-query.svg?branch=master)](https://travis-ci.org/yggie/rust-xml-query)

A simple implementation of a HTML/XML DOM tree which allows simple operations
like querying by CSS selectors, makes dealing with XML files less painful.

## Example

```rust
extern crate rquery;

use rquery::Document;

fn main() {
  let document = Document::new_from_xml_file("tests/fixtures/sample.xml").unwrap();

  let title = document.select("title").unwrap();
  assert_eq!(title.text(), "Sample Document");
  assert_eq!(title.attr("ref").unwrap(), "main-title");

  let item_count = document.select_all("item").unwrap().count();
  assert_eq!(item_count, 2);

  let item_titles = document.select_all("item > title").unwrap()
    .map(|element| element.text().clone())
    .collect::<Vec<String>>()
    .join(", ");
  assert_eq!(item_titles, "Another Sample, Other Sample");
}
```

## License

This software is distributed under the [MIT](/LICENSE) open source license.
