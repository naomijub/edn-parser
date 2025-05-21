//! EDN syntax tree generator

mod lexer;
mod parser;

use std::io::BufWriter;

use codespan_reporting::diagnostic::Severity;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::NoColor;
use codespan_reporting::term::{self, Config};
use parser::*;

pub use codespan_reporting::diagnostic::Diagnostic;
pub use lexer::Token;
pub use parser::{Cst, CstChildren, Node, NodeRef, Rule, Span};

/// Generates the syntax tree (CST) for the given EDN source string and returns it as a vector of strings.
///
/// # Panics
///
/// May panic if fails to emit the [`Diagnostic`]s to the terminal.
#[wasm_bindgen]
#[must_use]
pub fn generate_syntax_tree(source: &str) -> Vec<String> {
    let mut diags = vec![];
    let cst = Parser::parse(source, &mut diags);
    let mut writer = NoColor::new(BufWriter::new(Vec::new()));
    let config = Config::default();
    let file = SimpleFile::new("<input>", source);
    for diag in &diags {
        term::emit(&mut writer, &config, &file, diag).unwrap();
    }
    vec![
        format!("{cst}"),
        String::from_utf8(writer.into_inner().into_inner().unwrap()).unwrap(),
    ]
}

/// Parses the given EDN source string and returns a [`ParsedEdn`] object containing
/// the concrete syntax tree (CST) and any non-error diagnostics (warnings).
///
/// # Arguments
///
/// * `source`: A string that holds the EDN source to be parsed.
///
/// # Returns
///
/// * [`ParsedEdn`] - If the parsing is successful, returns a [`ParsedEdn`] object
///   containing the [`Cst`] and an optional vector of [`Diagnostic<()>`] if there
///   are any warnings.
///
/// # Errors
///
/// * [`Vec<Diagnostic>`] - If there are any parsing errors, returns a vector
///   of diagnostics representing those errors.
pub fn edn_parse(source: &str) -> Result<ParsedEdn<'_>, Vec<Diagnostic<()>>> {
    let mut diags = vec![];
    let cst = Parser::parse(source, &mut diags);
    let errors = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .cloned()
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(errors);
    }
    let warnings = diags
        .iter()
        .filter(|d| d.severity != Severity::Error)
        .cloned()
        .collect::<Vec<_>>();
    let warnings = if warnings.is_empty() {
        None
    } else {
        Some(warnings)
    };

    Ok(ParsedEdn { cst, warnings })
}

/// A struct that holds the parsed EDN concrete syntax tree (CST) and any warnings encountered during parsing.
#[derive(Debug, PartialEq, Serialize)]
pub struct ParsedEdn<'parsed> {
    /// EDN concrete syntax tree (CST)
    pub cst: Cst<'parsed>,
    /// Any warnings encountered during parsing
    pub warnings: Option<Vec<Diagnostic<()>>>,
}
