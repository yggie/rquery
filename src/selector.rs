use std::str::Chars;
use std::iter::Peekable;

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
    Attribute(String, MatchType, String),
}

/// The match type for an attribute selector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatchType {
    /// Indicates that the match must be identical
    Equals,
}

macro_rules! expect_token {
    ($token_option: expr, $token: expr) => {
        match $token_option {
            Some($token) => { },
            Some(token) => return Err(UnexpectedTokenError(token)),
            None => return Err(UnexpectedTokenError(' ')),
        }
    }
}

#[inline]
fn non_digit(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

#[inline]
fn allowed_character(c: char) -> bool {
    non_digit(c) || ('0' <= c && c <= '9') || c == '-' || c == '_'
}

fn extract_valid_string(chars: &mut Peekable<Chars>) -> Result<String, UnexpectedTokenError> {
    extract_valid_string_until_token(chars, ' ')
}

fn extract_valid_string_until_token(chars: &mut Peekable<Chars>, stop_token: char) -> Result<String, UnexpectedTokenError> {
    let mut string = String::new();

    while let Some(&c) = chars.peek() {
        if c == stop_token {
            chars.next().unwrap();
            break;
        } else if allowed_character(c) {
            string.push(chars.next().unwrap());
        } else {
            return Err(UnexpectedTokenError(c));
        }
    }

    return Ok(string);
}

impl Selector {
    fn create_list(string: &str) -> Result<Vec<Selector>, UnexpectedTokenError> {
        let mut selectors = Vec::new();

        let mut chars = string.chars().peekable();
        while let Some(&c) = chars.peek() {
            match Selector::next_selector(c, &mut chars) {
                Ok(selector) =>
                    selectors.push(selector),

                Err(err) =>
                    return Err(err),
            }
        }

        return Ok(selectors);
    }

    fn next_selector(c: char, chars: &mut Peekable<Chars>) -> Result<Selector, UnexpectedTokenError> {
        if non_digit(c) {
            Selector::create_tag_name(chars)
        } else if c == '#' {
            Selector::create_id(chars)
        } else if c == '[' {
            Selector::create_attribute(chars)
        } else {
            Err(UnexpectedTokenError(c))
        }
    }

    fn create_tag_name(chars: &mut Peekable<Chars>) -> Result<Selector, UnexpectedTokenError> {
        extract_valid_string(chars).map(|s| Selector::TagName(s))
    }

    fn create_id(chars: &mut Peekable<Chars>) -> Result<Selector, UnexpectedTokenError> {
        match chars.next() {
            Some('#') =>
                return extract_valid_string(chars).map(|s| Selector::Id(s)),

            Some(token) =>
                return Err(UnexpectedTokenError(token)),

            None =>
                return Err(UnexpectedTokenError(' ')),
        }
    }

    fn create_attribute(chars: &mut Peekable<Chars>) -> Result<Selector, UnexpectedTokenError> {
        expect_token!(chars.next(), '[');

        extract_valid_string_until_token(chars, '=').and_then(|attribute| {
            Ok((attribute, MatchType::Equals))
        }).and_then(|(attribute, match_type)| {
            let result = if Some(&'"') == chars.peek() {
                chars.next().unwrap();
                let result = extract_valid_string_until_token(chars, '"');
                expect_token!(chars.next(), ']');

                result
            } else {
                extract_valid_string_until_token(chars, ']')
            };

            result.map(|value| {
                Selector::Attribute(attribute, match_type, value)
            })
        })
    }
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
        let normalized_selector = selector.split(">")
            .collect::<Vec<&str>>()
            .join(" > ");

        let selector_parts = SelectorParts {
            inner_iter: normalized_selector.split_whitespace().into_iter().map(|s| s.to_string()),
        };

        selector_parts
           .fold(Ok(Vec::new()), |result_so_far, (scope, part)| {
               if let Ok(mut compound_selectors) = result_so_far {
                   Selector::create_list(&part).map(|parts| {
                       compound_selectors.push(CompoundSelector {
                           scope: scope,
                           parts: parts
                       });

                       compound_selectors
                   })
               } else {
                   result_so_far
               }
           })
    }
}
