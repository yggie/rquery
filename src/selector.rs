pub struct CompoundSelector {
    pub scope: Scope,
    pub parts: Vec<Selector>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scope {
    DirectChild,
    IndirectChild,
}

#[derive(Clone)]
pub enum Selector {
    TagName(String),
    Attribute(String),
}

pub struct SelectorParts<I: Iterator<Item=String>> {
    inner_iter: I,
}

impl<I: Iterator<Item=String>> Iterator for SelectorParts<I> {
    type Item = (Scope, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().and_then(|next_part| {
            if &next_part == ">" {
                Some((Scope::DirectChild, self.inner_iter.next().unwrap()))
            } else {
                Some((Scope::IndirectChild, next_part))
            }
        })
    }
}

impl CompoundSelector {
    pub fn parse(selector: &str) -> Result<Vec<CompoundSelector>, ()> {
        let normalized_selector = selector.split(">").collect::<Vec<&str>>().join(" > ");

        let selector_parts = SelectorParts {
            inner_iter: normalized_selector.split_whitespace().into_iter().map(|s| s.to_string()),
        };

        Ok(selector_parts
           .map(|(scope, part)| {
               CompoundSelector {
                   scope: scope,
                   parts: vec!(Selector::TagName(part.to_string()))
               }
           })
           .collect())
    }
}
