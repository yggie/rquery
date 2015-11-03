/// An error which is returned when parsing a selector encounters an unexpected
/// token
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnexpectedTokenError(pub char);

/// Represents a component of a parsed CSS selector is used to match a single
/// element.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum Selector {
    /// Represents an id selector (e.g. `#the-id`)
    Id(String),
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
    pub fn parse(selector: &str) -> Result<Vec<CompoundSelector>, UnexpectedTokenError> {
        let normalized_selector = selector.split(">").collect::<Vec<&str>>().join(" > ");

        let selector_parts = SelectorParts {
            inner_iter: normalized_selector.split_whitespace().into_iter().map(|s| s.to_string()),
        };

        selector_parts
           .fold(Ok(Vec::new()), |result_so_far, (scope, part)| {
               if let Ok(mut compound_selectors) = result_so_far {
                   compound_selectors.push(CompoundSelector {
                       scope: scope,
                       parts: vec!(Selector::TagName(part.to_string()))
                   });

                   Ok(compound_selectors)
               } else {
                   result_so_far
               }
           })
    }
}
