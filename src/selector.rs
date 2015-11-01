/// Represents a component of a parsed CSS selector is used to match a single
/// element.
pub struct CompoundSelector {
    /// The scope of the selector.
    pub scope: Scope,
    /// The individual parts that make up the compound selector.
    pub parts: Vec<Selector>,
}

/// The scope of the `CompoundSelector`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scope {
    /// Implies that the selector must be a direct descendent of the previous
    /// match (e.g. `body > header`).
    DirectChild,
    /// Implies that the selector is a descendent of the previous match (e.g.,
    /// `body header`).
    IndirectChild,
}

/// The individual parts of the `CompoundSelector`. For example, the selector
/// `input[type="radio"]` has two parts, the `TagName` and `Attribute`
/// selectors.
#[derive(Clone)]
pub enum Selector {
    /// Represents a tag name selector (e.g. `input`)
    TagName(String),
    /// Represents an attribute selector (e.g. `[type="radio"]`)
    Attribute(String),
}

struct SelectorParts<I: Iterator<Item=String>> {
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
    /// Parses the string and converts it to a list of `CompoundSelector`s.
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
