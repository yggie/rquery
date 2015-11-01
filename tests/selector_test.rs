use rquery::{ CompoundSelector, Scope, Selector };

fn assert_as_single_tag(compound_selector: &CompoundSelector, tag_name: &str) {
    assert_eq!(compound_selector.parts.len(), 1);

    if let &Selector::TagName(ref string) = compound_selector.parts.last().unwrap() {
        assert_eq!(string, tag_name)
    } else {
        panic!(format!("Did not match tag name \"{}\"", tag_name));
    }
}

#[test]
fn it_can_parse_a_single_tag_selector() {
    let compound_selectors = CompoundSelector::parse("apples").unwrap();

    assert_eq!(compound_selectors.len(), 1);

    assert_eq!(compound_selectors[0].scope, Scope::IndirectChild);
    assert_as_single_tag(&compound_selectors[0], "apples");
}

#[test]
fn it_can_parse_a_nested_tag_selectors() {
    let compound_selectors = CompoundSelector::parse("basket apple").unwrap();

    assert_eq!(compound_selectors.len(), 2);

    assert_eq!(compound_selectors[0].scope, Scope::IndirectChild);
    assert_as_single_tag(&compound_selectors[0], "basket");

    assert_eq!(compound_selectors[1].scope, Scope::IndirectChild);
    assert_as_single_tag(&compound_selectors[1], "apple");
}

#[test]
fn it_can_parse_a_direct_child_selector() {
    let compound_selectors = CompoundSelector::parse("basket > apple").unwrap();

    assert_eq!(compound_selectors.len(), 2);

    assert_eq!(compound_selectors[0].scope, Scope::IndirectChild);
    assert_as_single_tag(&compound_selectors[0], "basket");

    assert_eq!(compound_selectors[1].scope, Scope::DirectChild);
    assert_as_single_tag(&compound_selectors[1], "apple");
}
