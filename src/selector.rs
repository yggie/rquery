pub struct CompoundSelector {
    pub scope: Scope,
    pub parts: Vec<Selector>,
}

#[derive(Clone, Copy)]
pub enum Scope {
    DirectChild,
    IndirectChild,
}

#[derive(Clone)]
pub enum Selector {
    TagName(String),
    Attribute(String),
}

impl CompoundSelector {
    pub fn parse(selector: &str) -> Result<Vec<CompoundSelector>, ()> {
        Ok(selector.split_whitespace()
           .map(|part| {
               CompoundSelector {
                   scope: Scope::IndirectChild,
                   parts: vec!(Selector::TagName(part.to_string()))
               }
           })
           .collect())
    }
}
