//! The ConfigNode text format that Kerbal Space Program writes its saves in.
//!
//! A document is a sequence of lines. A line `key = value` adds a value to the node that
//! is open. A line holding a name, followed by a line holding `{`, opens a child node,
//! which a line holding `}` closes. The name and the brace may also share a line, as in
//! `NAME {`. Text from `//` to the end of a line is a comment. Blank lines are ignored.
//!
//! # Assumptions
//!
//! - A value runs from the first `=` on its line to the end of the line, so it may hold
//!   further `=` signs, spaces and braces. Leading and trailing white space is trimmed
//!   from keys and values.
//! - `//` starts a comment wherever it appears, as in the game's own reader. This is an
//!   inference from the game's behaviour, not verified against its code. A value that
//!   contains `//` is therefore cut short, in the game and here alike.
//! - Keys may repeat, and their order is kept. Child nodes may repeat, and their order is
//!   kept. The order of values relative to child nodes is not kept.
//!
//! # Limits and failure modes
//!
//! The parser is iterative, so its stack use does not grow with the input. It refuses
//! input larger than [`ParseLimits::max_bytes`] and nesting deeper than
//! [`ParseLimits::max_depth`], and the depth limit cannot exceed [`MAX_DEPTH_CEILING`],
//! because dropping a tree is recursive in its depth. Every refusal is a [`ParseError`]
//! that names the line, numbered from one.

use core::fmt;

/// The default largest input accepted, 64 MiB. A career save with a large fleet runs to
/// a few megabytes, so this leaves an ample margin while bounding memory.
pub const DEFAULT_MAX_BYTES: usize = 64 * 1024 * 1024;

/// The default deepest nesting accepted, counting the top-level `GAME` node as depth 1.
/// A save observed on 2026-11-18 nested nine deep, through stored parts in a kerbal's
/// inventory, so 32 leaves a wide margin.
pub const DEFAULT_MAX_DEPTH: usize = 32;

/// The deepest nesting any [`ParseLimits`] may allow. Dropping a [`Node`] recurses once
/// per level, so the depth must stay bounded for the drop to be safe on any thread.
pub const MAX_DEPTH_CEILING: usize = 256;

/// The longest excerpt of input quoted in an error, in characters, so that an error
/// message stays short whatever the input holds.
const EXCERPT_CHARS: usize = 64;

/// The bounds the parser enforces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLimits {
    max_bytes: usize,
    max_depth: usize,
}

impl ParseLimits {
    /// The defaults, [`DEFAULT_MAX_BYTES`] and [`DEFAULT_MAX_DEPTH`].
    pub const DEFAULT: Self = Self {
        max_bytes: DEFAULT_MAX_BYTES,
        max_depth: DEFAULT_MAX_DEPTH,
    };

    /// Checks and builds a set of limits.
    ///
    /// # Errors
    ///
    /// Refuses a size or depth of zero, and a depth above [`MAX_DEPTH_CEILING`].
    pub fn new(max_bytes: usize, max_depth: usize) -> Result<Self, ParseError> {
        if max_bytes == 0 || max_depth == 0 || max_depth > MAX_DEPTH_CEILING {
            return Err(ParseError::InvalidLimits {
                max_bytes,
                max_depth,
            });
        }
        Ok(Self {
            max_bytes,
            max_depth,
        })
    }

    /// The largest input accepted, in bytes.
    #[must_use]
    pub fn max_bytes(self) -> usize {
        self.max_bytes
    }

    /// The deepest nesting accepted, with a top-level node at depth 1.
    #[must_use]
    pub fn max_depth(self) -> usize {
        self.max_depth
    }
}

impl Default for ParseLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Why a document was refused. Line numbers count from one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The limits asked for were zero or above the ceiling.
    InvalidLimits {
        /// The size asked for.
        max_bytes: usize,
        /// The depth asked for.
        max_depth: usize,
    },
    /// The input is larger than the limit.
    TooLarge {
        /// The input's size in bytes, or the limit plus one when the input was read
        /// from a stream and reading stopped there.
        size: usize,
        /// The limit in bytes.
        limit: usize,
    },
    /// The input is not valid UTF-8.
    InvalidUtf8 {
        /// The line holding the first invalid byte.
        line: usize,
    },
    /// A node opened deeper than the limit.
    TooDeep {
        /// The line that opened it.
        line: usize,
        /// The depth limit.
        limit: usize,
    },
    /// A `}` closed a node when none was open.
    UnexpectedClose {
        /// The line of the brace.
        line: usize,
    },
    /// A `{` appeared with no node name before it.
    StrayOpen {
        /// The line of the brace.
        line: usize,
    },
    /// A node name was not followed by `{`, or a line that is neither a value nor a
    /// brace was taken as a name and nothing opened.
    MissingOpen {
        /// The name, shortened if long.
        name: String,
        /// The line of the name.
        line: usize,
    },
    /// The input ended with a node still open.
    Unclosed {
        /// The name of the innermost open node, shortened if long.
        name: String,
        /// The line that opened it.
        line: usize,
    },
    /// A line could not be read as a value, a name or a brace.
    Malformed {
        /// The line.
        line: usize,
        /// What was wrong with it.
        reason: &'static str,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits {
                max_bytes,
                max_depth,
            } => write!(
                f,
                "invalid parse limits: {max_bytes} bytes and depth {max_depth}, \
                 which must be above zero with depth at most {MAX_DEPTH_CEILING}"
            ),
            Self::TooLarge { size, limit } => {
                write!(f, "input of {size} bytes exceeds the limit of {limit}")
            }
            Self::InvalidUtf8 { line } => write!(f, "line {line}: invalid UTF-8"),
            Self::TooDeep { line, limit } => {
                write!(f, "line {line}: nesting exceeds the depth limit of {limit}")
            }
            Self::UnexpectedClose { line } => {
                write!(f, "line {line}: closing brace with no open node")
            }
            Self::StrayOpen { line } => write!(f, "line {line}: opening brace with no name"),
            Self::MissingOpen { name, line } => {
                write!(
                    f,
                    "line {line}: `{name}` is not followed by an opening brace"
                )
            }
            Self::Unclosed { name, line } => {
                write!(f, "line {line}: node `{name}` is never closed")
            }
            Self::Malformed { line, reason } => write!(f, "line {line}: {reason}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// One `key = value` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    key: String,
    value: String,
    line: usize,
}

impl Value {
    /// The key, trimmed.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// The value, trimmed, which may be empty.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// The line it was read from, counting from one.
    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }
}

/// A node, with its values and child nodes in the order they were read.
///
/// The document itself is a root node with an empty name at line zero, whose children
/// are the top-level nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    name: String,
    line: usize,
    values: Vec<Value>,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String, line: usize) -> Self {
        Self {
            name,
            line,
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    /// The node's name, which is empty for the document root.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The line of the node's name, counting from one, or zero for the document root.
    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    /// Every value, in order.
    #[must_use]
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Every child node, in order.
    #[must_use]
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    /// The first value with this key.
    #[must_use]
    pub fn value(&self, key: &str) -> Option<&Value> {
        self.values.iter().find(|v| v.key == key)
    }

    /// Every value with this key, in order.
    pub fn values_named<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
        self.values.iter().filter(move |v| v.key == key)
    }

    /// The first child with this name.
    #[must_use]
    pub fn child(&self, name: &str) -> Option<&Node> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Every child with this name, in order.
    pub fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Node> + 'a {
        self.children.iter().filter(move |c| c.name == name)
    }

    /// Takes the first child with this name out of the node.
    pub(crate) fn take_child(&mut self, name: &str) -> Option<Node> {
        let index = self.children.iter().position(|c| c.name == name)?;
        Some(self.children.remove(index))
    }

    /// Writes the node back as text, with tabs for indentation and each node's values
    /// before its children. The document root writes only its contents.
    ///
    /// Parsing the output gives a tree with the same names, keys and values in the same
    /// order, though at other line numbers. The writer is iterative, like the parser.
    #[must_use]
    pub fn to_config_text(&self) -> String {
        let mut out = String::new();
        let root = self.name.is_empty();
        // Each frame is a node and the index of its next child to write.
        let mut stack: Vec<(&Node, usize)> = Vec::new();
        let base = if root { 0 } else { 1 };
        if !root {
            push_line(&mut out, 0, &self.name);
            push_line(&mut out, 0, "{");
        }
        write_values(&mut out, base, self);
        stack.push((self, 0));
        // Each pass either writes one child's header or closes one node, and a tree has
        // finitely many of each, so the loop ends.
        while let Some((node, next)) = stack.pop() {
            let depth = base + stack.len();
            match node.children.get(next) {
                Some(child) => {
                    stack.push((node, next + 1));
                    push_line(&mut out, depth, &child.name);
                    push_line(&mut out, depth, "{");
                    write_values(&mut out, depth + 1, child);
                    stack.push((child, 0));
                }
                None => {
                    if depth > 0 {
                        push_line(&mut out, depth - 1, "}");
                    }
                }
            }
        }
        out
    }
}

fn push_line(out: &mut String, depth: usize, text: &str) {
    for _ in 0..depth {
        out.push('\t');
    }
    out.push_str(text);
    out.push('\n');
}

fn write_values(out: &mut String, depth: usize, node: &Node) {
    for v in &node.values {
        for _ in 0..depth {
            out.push('\t');
        }
        out.push_str(&v.key);
        out.push_str(" = ");
        out.push_str(&v.value);
        out.push('\n');
    }
}

/// Shortens text for quoting in an error.
pub(crate) fn excerpt(text: &str) -> String {
    let mut chars = text.chars();
    let mut out: String = chars.by_ref().take(EXCERPT_CHARS).collect();
    if chars.next().is_some() {
        out.push_str("...");
    }
    out
}

/// What one line holds, once its comment is removed and it is trimmed.
enum Line<'a> {
    Value(&'a str, &'a str),
    Name(&'a str),
    NameOpen(&'a str),
    Open,
    Close,
}

fn classify(line: &str) -> Result<Line<'_>, &'static str> {
    if let Some((key, value)) = line.split_once('=') {
        let key = key.trim();
        if key.is_empty() {
            return Err("a value has an empty key");
        }
        if key.contains(['{', '}']) {
            return Err("a key contains a brace");
        }
        return Ok(Line::Value(key, value.trim()));
    }
    match line {
        "{" => return Ok(Line::Open),
        "}" => return Ok(Line::Close),
        _ => {}
    }
    if let Some(name) = line.strip_suffix('{') {
        let name = name.trim();
        if name.is_empty() || name.contains(['{', '}']) {
            return Err("a brace shares its line with other text");
        }
        return Ok(Line::NameOpen(name));
    }
    if line.contains(['{', '}']) {
        return Err("a brace shares its line with other text");
    }
    Ok(Line::Name(line))
}

fn strip_comment(line: &str) -> &str {
    match line.find("//") {
        Some(at) => &line[..at],
        None => line,
    }
}

/// The parser's state, a root and the nodes open beneath it, innermost last.
struct Builder {
    root: Node,
    open: Vec<Node>,
    max_depth: usize,
}

impl Builder {
    fn current(&mut self) -> &mut Node {
        match self.open.last_mut() {
            Some(node) => node,
            None => &mut self.root,
        }
    }

    fn open(&mut self, name: &str, line: usize) -> Result<(), ParseError> {
        if self.open.len() >= self.max_depth {
            return Err(ParseError::TooDeep {
                line,
                limit: self.max_depth,
            });
        }
        self.open.push(Node::new(name.to_owned(), line));
        Ok(())
    }

    fn close(&mut self, line: usize) -> Result<(), ParseError> {
        let done = self
            .open
            .pop()
            .ok_or(ParseError::UnexpectedClose { line })?;
        self.current().children.push(done);
        Ok(())
    }
}

/// Parses a document under the default limits.
///
/// # Errors
///
/// Returns a [`ParseError`] for input that is too large or too deep, or that is not well
/// formed.
pub fn parse(text: &str) -> Result<Node, ParseError> {
    parse_with_limits(text, ParseLimits::DEFAULT)
}

/// Parses a document from bytes, checking the size before decoding.
///
/// # Errors
///
/// As [`parse_with_limits`], and [`ParseError::InvalidUtf8`] for bytes that are not
/// UTF-8.
pub fn parse_bytes(bytes: &[u8], limits: ParseLimits) -> Result<Node, ParseError> {
    if bytes.len() > limits.max_bytes {
        return Err(ParseError::TooLarge {
            size: bytes.len(),
            limit: limits.max_bytes,
        });
    }
    match core::str::from_utf8(bytes) {
        Ok(text) => parse_with_limits(text, limits),
        Err(error) => {
            let before = bytes.get(..error.valid_up_to()).unwrap_or_default();
            let line = 1 + before.iter().filter(|&&b| b == b'\n').count();
            Err(ParseError::InvalidUtf8 { line })
        }
    }
}

/// Parses a document under the given limits.
///
/// A leading byte order mark is skipped. Lines may end in a line feed or a carriage
/// return and line feed.
///
/// # Errors
///
/// Returns a [`ParseError`] for input that is too large or too deep, or that is not well
/// formed.
pub fn parse_with_limits(text: &str, limits: ParseLimits) -> Result<Node, ParseError> {
    if text.len() > limits.max_bytes {
        return Err(ParseError::TooLarge {
            size: text.len(),
            limit: limits.max_bytes,
        });
    }
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut builder = Builder {
        root: Node::new(String::new(), 0),
        open: Vec::new(),
        max_depth: limits.max_depth,
    };
    // A name read on one line, waiting for its brace on a later one.
    let mut pending: Option<(&str, usize)> = None;
    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some((name, at)) = pending.take() {
            if line == "{" {
                builder.open(name, at)?;
                continue;
            }
            return Err(ParseError::MissingOpen {
                name: excerpt(name),
                line: at,
            });
        }
        let kind = classify(line).map_err(|reason| ParseError::Malformed {
            line: number,
            reason,
        })?;
        match kind {
            Line::Value(key, value) => builder.current().values.push(Value {
                key: key.to_owned(),
                value: value.to_owned(),
                line: number,
            }),
            Line::Name(name) => pending = Some((name, number)),
            Line::NameOpen(name) => builder.open(name, number)?,
            Line::Open => return Err(ParseError::StrayOpen { line: number }),
            Line::Close => builder.close(number)?,
        }
    }
    if let Some((name, at)) = pending {
        return Err(ParseError::MissingOpen {
            name: excerpt(name),
            line: at,
        });
    }
    if let Some(node) = builder.open.last() {
        return Err(ParseError::Unclosed {
            name: excerpt(&node.name),
            line: node.line,
        });
    }
    Ok(builder.root)
}
