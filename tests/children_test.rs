use rquery::Document;

fn new_document() -> Document {
    Document::new_from_xml_string(r"
<html>
<head></head>
<body>
<h2>
  Excerpt from <em>Cré na Cille</em>
</h2>
<div id='div-with-children'>
  <p>
  <strong>Ní mé</strong> an ar Áit <em>an Phuint</em> nó <em>na gCúig Déag</em> atá mé curtha?
  </p>
</div>
</body>
</html>
").unwrap()
}

#[test]
fn it_knows_its_combined_child_text_contents() {
    let document = new_document();
    
    let element = document.select("div").unwrap();
    assert_eq!(element.text().trim(), "Ní mé an ar Áit an Phuint nó na gCúig Déag atá mé curtha?");
}