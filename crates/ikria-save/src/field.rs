//! Typed reads of a node's fields, and the error an extractor returns.

use core::fmt;

use crate::node::{Node, Value, excerpt};

/// Why a record could not be extracted from a parsed save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    /// A node the record needs is absent.
    MissingNode {
        /// What was looked for, such as `SCENARIO name = Funding`.
        node: String,
        /// The node it was looked for in.
        parent: String,
        /// The line of that parent.
        line: usize,
    },
    /// A required field is absent.
    MissingField {
        /// The node that lacks it.
        node: String,
        /// The field's key.
        field: &'static str,
        /// The line of the node.
        line: usize,
    },
    /// A field is present but its value is not of the expected form.
    InvalidField {
        /// The node that holds it.
        node: String,
        /// The field's key.
        field: &'static str,
        /// The value found, shortened if long.
        value: String,
        /// What was expected.
        expected: &'static str,
        /// The line of the value.
        line: usize,
    },
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { node, parent, line } => {
                write!(f, "line {line}: `{parent}` has no `{node}`")
            }
            Self::MissingField { node, field, line } => {
                write!(f, "line {line}: `{node}` has no field `{field}`")
            }
            Self::InvalidField {
                node,
                field,
                value,
                expected,
                line,
            } => write!(
                f,
                "line {line}: field `{field}` of `{node}` is `{value}`, expected {expected}"
            ),
        }
    }
}

impl std::error::Error for ExtractError {}

/// A finite number, as every number read from a save must be.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Finite(f64);

impl Finite {
    /// Wraps a number that is finite.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self(value))
    }

    /// The number.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

/// A finite number that is not negative.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NonNegative(f64);

impl NonNegative {
    /// Wraps a number that is finite and not negative.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value >= 0.0).then_some(Self(value))
    }

    /// The number.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

pub(crate) fn missing_node(node: impl Into<String>, parent: &Node) -> ExtractError {
    let parent_name = if parent.name().is_empty() {
        "the document root".to_owned()
    } else {
        excerpt(parent.name())
    };
    ExtractError::MissingNode {
        node: node.into(),
        parent: parent_name,
        line: parent.line(),
    }
}

pub(crate) fn invalid(
    node: &Node,
    field: &'static str,
    value: &Value,
    expected: &'static str,
) -> ExtractError {
    ExtractError::InvalidField {
        node: excerpt(node.name()),
        field,
        value: excerpt(value.value()),
        expected,
        line: value.line(),
    }
}

/// The first value of a required field.
pub(crate) fn required<'a>(node: &'a Node, field: &'static str) -> Result<&'a Value, ExtractError> {
    node.value(field).ok_or_else(|| ExtractError::MissingField {
        node: excerpt(node.name()),
        field,
        line: node.line(),
    })
}

/// A required field's text, which may be empty.
pub(crate) fn text(node: &Node, field: &'static str) -> Result<String, ExtractError> {
    required(node, field).map(|v| v.value().to_owned())
}

/// A required field's text, which must not be empty.
pub(crate) fn non_empty(node: &Node, field: &'static str) -> Result<String, ExtractError> {
    let value = required(node, field)?;
    if value.value().is_empty() {
        return Err(invalid(node, field, value, "a non-empty value"));
    }
    Ok(value.value().to_owned())
}

/// An optional field's text, absent when the field is absent or empty.
pub(crate) fn optional_text(node: &Node, field: &str) -> Option<String> {
    node.value(field)
        .map(Value::value)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

fn float_of(text: &str) -> Option<f64> {
    text.parse::<f64>().ok()
}

pub(crate) fn finite_of(
    node: &Node,
    field: &'static str,
    value: &Value,
) -> Result<Finite, ExtractError> {
    float_of(value.value())
        .and_then(Finite::new)
        .ok_or_else(|| invalid(node, field, value, "a finite number"))
}

pub(crate) fn finite(node: &Node, field: &'static str) -> Result<Finite, ExtractError> {
    finite_of(node, field, required(node, field)?)
}

pub(crate) fn non_negative(node: &Node, field: &'static str) -> Result<NonNegative, ExtractError> {
    let value = required(node, field)?;
    float_of(value.value())
        .and_then(NonNegative::new)
        .ok_or_else(|| invalid(node, field, value, "a finite number not below zero"))
}

pub(crate) fn optional_finite(
    node: &Node,
    field: &'static str,
) -> Result<Option<Finite>, ExtractError> {
    node.value(field)
        .map(|value| finite_of(node, field, value))
        .transpose()
}

pub(crate) fn unsigned(node: &Node, field: &'static str) -> Result<u32, ExtractError> {
    let value = required(node, field)?;
    unsigned_of(node, field, value)
}

pub(crate) fn unsigned_of(
    node: &Node,
    field: &'static str,
    value: &Value,
) -> Result<u32, ExtractError> {
    value
        .value()
        .parse::<u32>()
        .map_err(|_| invalid(node, field, value, "a whole number from 0 to 4294967295"))
}

pub(crate) fn optional_unsigned(
    node: &Node,
    field: &'static str,
) -> Result<Option<u32>, ExtractError> {
    node.value(field)
        .map(|value| unsigned_of(node, field, value))
        .transpose()
}

/// An optional boolean, written by the game as `True` or `False`. Case is ignored.
pub(crate) fn optional_bool(
    node: &Node,
    field: &'static str,
) -> Result<Option<bool>, ExtractError> {
    let Some(value) = node.value(field) else {
        return Ok(None);
    };
    if value.value().eq_ignore_ascii_case("true") {
        Ok(Some(true))
    } else if value.value().eq_ignore_ascii_case("false") {
        Ok(Some(false))
    } else {
        Err(invalid(node, field, value, "True or False"))
    }
}

/// A required field whose value is one of a closed set of names.
pub(crate) fn one_of<T: Copy>(
    node: &Node,
    field: &'static str,
    table: &[(&str, T)],
    expected: &'static str,
) -> Result<T, ExtractError> {
    let value = required(node, field)?;
    table
        .iter()
        .find(|(name, _)| *name == value.value())
        .map(|(_, kind)| *kind)
        .ok_or_else(|| invalid(node, field, value, expected))
}

/// The first child with this name, which must be present.
pub(crate) fn child<'a>(node: &'a Node, name: &'static str) -> Result<&'a Node, ExtractError> {
    node.child(name).ok_or_else(|| missing_node(name, node))
}
