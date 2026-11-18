//! Tests of the ConfigNode parser and writer, and of reading a save from a file,
//! including every refusal.

use std::path::{Path, PathBuf};

use ikria_save::node::{self, MAX_DEPTH_CEILING, parse, parse_bytes, parse_with_limits};
use ikria_save::{ExtractError, Node, ParseError, ParseLimits, Save, SaveError};

const FIXTURES: [&str; 6] = [
    "agency.sfs",
    "facilities.sfs",
    "research.sfs",
    "roster.sfs",
    "vessels.sfs",
    "contracts.sfs",
];

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).expect("fixture is readable")
}

fn only_child(root: &Node) -> &Node {
    assert_eq!(root.children().len(), 1, "one top-level node");
    &root.children()[0]
}

/// Builds `depth` nested nodes, each named `N`, with one value in the innermost.
fn nested(depth: usize) -> String {
    let mut text = String::new();
    for _ in 0..depth {
        text.push_str("N\n{\n");
    }
    text.push_str("k = v\n");
    for _ in 0..depth {
        text.push_str("}\n");
    }
    text
}

fn limits(max_bytes: usize, max_depth: usize) -> ParseLimits {
    ParseLimits::new(max_bytes, max_depth).expect("valid limits")
}

#[test]
fn values_keep_spaces_equals_signs_and_repeats_in_order() {
    let root = parse(
        "GAME\n{\n\tTitle = A title = with equals\n\tpart = a\n\tpart = b\n\tpart = a\n\tempty =\n}\n",
    )
    .unwrap();
    let game = only_child(&root);
    assert_eq!(game.name(), "GAME");
    assert_eq!(game.line(), 1);
    let title = game.value("Title").unwrap();
    assert_eq!(title.value(), "A title = with equals");
    assert_eq!(title.line(), 3);
    let parts: Vec<&str> = game.values_named("part").map(|v| v.value()).collect();
    assert_eq!(parts, ["a", "b", "a"]);
    assert_eq!(game.value("empty").unwrap().value(), "");
    assert!(game.value("absent").is_none());
}

#[test]
fn comments_blank_lines_and_line_endings_are_ignored() {
    let text = "\u{feff}// a leading comment\r\n\r\nGAME // trailing comment\r\n// between name and brace\r\n\r\n{\r\n  key = value // cut here\r\n\t\r\n}\r\n";
    let root = parse(text).unwrap();
    let game = only_child(&root);
    assert_eq!(game.line(), 3);
    assert_eq!(game.value("key").unwrap().value(), "value");
}

#[test]
fn a_comment_cuts_a_value_as_the_game_does() {
    let root = parse("GAME\n{\nlinkURL = http://example\n}\n").unwrap();
    assert_eq!(only_child(&root).value("linkURL").unwrap().value(), "http:");
}

#[test]
fn a_name_may_share_its_line_with_the_opening_brace() {
    let root = parse("GAME {\n\tINNER {\n\t\tk = v\n\t}\n}\n").unwrap();
    let inner = only_child(&root).child("INNER").unwrap();
    assert_eq!(inner.value("k").unwrap().value(), "v");
    assert_eq!(inner.line(), 2);
}

#[test]
fn repeated_child_nodes_keep_their_order() {
    let root = parse("G\n{\nC\n{\ni = 1\n}\nD\n{\n}\nC\n{\ni = 2\n}\n}\n").unwrap();
    let game = only_child(&root);
    let ids: Vec<&str> = game
        .children_named("C")
        .map(|c| c.value("i").unwrap().value())
        .collect();
    assert_eq!(ids, ["1", "2"]);
    assert_eq!(game.children().len(), 3);
    assert_eq!(game.child("C").unwrap().value("i").unwrap().value(), "1");
}

#[test]
fn braces_inside_a_value_belong_to_the_value() {
    let root = parse("G\n{\nname = odd { name }\n}\n").unwrap();
    assert_eq!(
        only_child(&root).value("name").unwrap().value(),
        "odd { name }"
    );
}

#[test]
fn an_empty_document_is_an_empty_root() {
    let root = parse("// nothing\n\n").unwrap();
    assert!(root.children().is_empty() && root.values().is_empty());
    assert_eq!(root.name(), "");
    assert_eq!(root.line(), 0);
}

#[test]
fn unbalanced_braces_are_refused_with_line_numbers() {
    assert_eq!(
        parse("G\n{\nk = v\n"),
        Err(ParseError::Unclosed {
            name: "G".into(),
            line: 1
        })
    );
    assert_eq!(
        parse("G\n{\nH\n{\n}\n"),
        Err(ParseError::Unclosed {
            name: "G".into(),
            line: 1
        })
    );
    assert_eq!(
        parse("G\n{\n}\n}\n"),
        Err(ParseError::UnexpectedClose { line: 4 })
    );
    assert_eq!(parse("}\n"), Err(ParseError::UnexpectedClose { line: 1 }));
    assert_eq!(
        parse("G\n{\n{\n}\n}\n"),
        Err(ParseError::StrayOpen { line: 3 })
    );
}

#[test]
fn malformed_lines_are_refused_with_line_numbers() {
    // A line that is neither a value nor a brace is taken as a name, and is refused when
    // no brace follows it.
    assert_eq!(
        parse("G\n{\nnot a value\nk = v\n}\n"),
        Err(ParseError::MissingOpen {
            name: "not a value".into(),
            line: 3
        })
    );
    assert_eq!(
        parse("G\n{\n}\nTRAILING\n"),
        Err(ParseError::MissingOpen {
            name: "TRAILING".into(),
            line: 4
        })
    );
    for (text, line) in [
        ("G\n{\n = value\n}\n", 3),
        ("G\n{\nk{ = value\n}\n", 3),
        ("G\n{\n} extra\n", 3),
        ("G\n{\nH { }\n}\n", 3),
        ("{}\n", 1),
        ("G\n{\n  {\n}\n}\n", 3),
    ] {
        match parse(text) {
            Err(ParseError::Malformed { line: got, .. })
            | Err(ParseError::StrayOpen { line: got }) => {
                assert_eq!(got, line, "for {text:?}");
            }
            other => panic!("{text:?} gave {other:?}"),
        }
    }
}

#[test]
fn a_long_name_is_shortened_in_the_error() {
    let name = "X".repeat(500);
    match parse(&format!("{name}\nk = v\n")) {
        Err(ParseError::MissingOpen { name: shown, line }) => {
            assert_eq!(line, 1);
            assert!(shown.len() < 100, "excerpt has {} bytes", shown.len());
            assert!(shown.ends_with("..."));
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn depth_up_to_the_limit_is_accepted_and_beyond_it_refused() {
    let at = limits(1 << 20, 5);
    let root = parse_with_limits(&nested(5), at).unwrap();
    // Walk down without recursion to check the depth that was built.
    let mut node = &root;
    let mut depth = 0;
    while let Some(next) = node.children().first() {
        node = next;
        depth += 1;
    }
    assert_eq!(depth, 5);
    assert_eq!(node.value("k").unwrap().value(), "v");
    assert_eq!(
        parse_with_limits(&nested(6), at),
        Err(ParseError::TooDeep { line: 11, limit: 5 })
    );
}

#[test]
fn the_ceiling_depth_parses_and_a_hostile_depth_is_refused_early() {
    let ceiling = limits(1 << 24, MAX_DEPTH_CEILING);
    assert!(parse_with_limits(&nested(MAX_DEPTH_CEILING), ceiling).is_ok());
    // A hundred thousand opening braces would overflow a recursive parser. Here the
    // parse stops at the default depth limit.
    let hostile = "N\n{\n".repeat(100_000);
    assert_eq!(
        parse(&hostile),
        Err(ParseError::TooDeep {
            line: 2 * node::DEFAULT_MAX_DEPTH + 1,
            limit: node::DEFAULT_MAX_DEPTH
        })
    );
}

#[test]
fn size_up_to_the_limit_is_accepted_and_beyond_it_refused() {
    let text = "G\n{\nk = v\n}\n";
    let exact = limits(text.len(), 4);
    assert!(parse_with_limits(text, exact).is_ok());
    let under = limits(text.len() - 1, 4);
    assert_eq!(
        parse_with_limits(text, under),
        Err(ParseError::TooLarge {
            size: text.len(),
            limit: text.len() - 1
        })
    );
    assert_eq!(
        parse_bytes(text.as_bytes(), under),
        Err(ParseError::TooLarge {
            size: text.len(),
            limit: text.len() - 1
        })
    );
}

#[test]
fn invalid_limits_are_refused() {
    for (bytes, depth) in [(0, 4), (10, 0), (10, MAX_DEPTH_CEILING + 1)] {
        assert_eq!(
            ParseLimits::new(bytes, depth),
            Err(ParseError::InvalidLimits {
                max_bytes: bytes,
                max_depth: depth
            })
        );
    }
    let ok = limits(10, MAX_DEPTH_CEILING);
    assert_eq!((ok.max_bytes(), ok.max_depth()), (10, MAX_DEPTH_CEILING));
    assert_eq!(ParseLimits::default(), ParseLimits::DEFAULT);
}

#[test]
fn invalid_utf8_is_refused_with_its_line() {
    let bytes = b"G\n{\nk = \xff\n}\n";
    assert_eq!(
        parse_bytes(bytes, ParseLimits::DEFAULT),
        Err(ParseError::InvalidUtf8 { line: 3 })
    );
    assert!(parse_bytes(b"G\n{\n}\n", ParseLimits::DEFAULT).is_ok());
}

#[test]
fn every_error_has_a_message_naming_its_line() {
    let errors = [
        ParseError::TooDeep { line: 7, limit: 2 },
        ParseError::UnexpectedClose { line: 7 },
        ParseError::StrayOpen { line: 7 },
        ParseError::MissingOpen {
            name: "N".into(),
            line: 7,
        },
        ParseError::Unclosed {
            name: "N".into(),
            line: 7,
        },
        ParseError::Malformed {
            line: 7,
            reason: "why",
        },
        ParseError::InvalidUtf8 { line: 7 },
    ];
    for error in errors {
        assert!(error.to_string().starts_with("line 7: "), "{error}");
    }
    assert!(
        ParseError::TooLarge { size: 2, limit: 1 }
            .to_string()
            .contains("limit")
    );
}

#[test]
fn writing_and_reparsing_every_fixture_is_a_fixed_point() {
    for name in FIXTURES {
        let first = parse(&fixture_text(name)).unwrap();
        let written = first.to_config_text();
        let second = parse(&written).unwrap();
        assert_eq!(second.to_config_text(), written, "{name}");
        // The trees agree in every name, key and value, though not in line numbers.
        assert_eq!(count_nodes(&first), count_nodes(&second), "{name}");
    }
}

#[test]
fn a_named_node_writes_itself_with_its_braces() {
    let root = parse("G\n{\nk = v\nC\n{\n}\n}\n").unwrap();
    assert_eq!(
        only_child(&root).to_config_text(),
        "G\n{\n\tk = v\n\tC\n\t{\n\t}\n}\n"
    );
}

fn count_nodes(root: &Node) -> (usize, usize) {
    let mut nodes = 0;
    let mut values = 0;
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        nodes += 1;
        values += node.values().len();
        stack.extend(node.children());
    }
    (nodes, values)
}

#[test]
fn a_save_is_read_from_a_file() {
    let save = Save::read(&fixture_path("agency.sfs")).unwrap();
    assert_eq!(save.game().name(), "GAME");
    assert_eq!(save.game().value("version").unwrap().value(), "1.12.5");
    let from_text = Save::parse(&fixture_text("agency.sfs")).unwrap();
    assert_eq!(save, from_text);
}

#[test]
fn a_file_over_the_limit_is_refused() {
    let path = fixture_path("agency.sfs");
    let size = usize::try_from(std::fs::metadata(&path).unwrap().len()).unwrap();
    match Save::read_with_limits(&path, limits(size - 1, 8)) {
        Err(SaveError::Parse(ParseError::TooLarge { size: read, limit })) => {
            assert_eq!(limit, size - 1);
            // Reading stops one byte past the limit.
            assert_eq!(read, size);
        }
        other => panic!("unexpected {other:?}"),
    }
    assert!(Save::read_with_limits(&path, limits(size, 8)).is_ok());
}

#[test]
fn a_missing_file_is_an_io_error() {
    let error = Save::read(&fixture_path("no-such-file.sfs")).unwrap_err();
    assert!(matches!(error, SaveError::Io(_)), "{error:?}");
    assert!(error.to_string().starts_with("reading the save"));
}

#[test]
fn a_document_without_game_is_not_a_save() {
    let error = Save::parse("OTHER\n{\n}\n").unwrap_err();
    match &error {
        SaveError::Extract(ExtractError::MissingNode { node, line, .. }) => {
            assert_eq!(node, "GAME");
            assert_eq!(*line, 0);
        }
        other => panic!("unexpected {other:?}"),
    }
    assert!(std::error::Error::source(&error).is_some());
    assert!(matches!(
        Save::parse("GAME\n{\n"),
        Err(SaveError::Parse(ParseError::Unclosed { .. }))
    ));
}
