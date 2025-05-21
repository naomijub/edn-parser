#![allow(missing_docs)]

use edn_parser::*;
use insta::assert_snapshot;

#[test]
fn newline() {
    assert_snapshot!(edn_parse("\\newline").unwrap().cst);
}

#[test]
fn tab() {
    assert_snapshot!(edn_parse("\\tab").unwrap().cst);
}

#[test]
fn return_char() {
    assert_snapshot!(edn_parse("\\return").unwrap().cst);
}

#[test]
fn space() {
    assert_snapshot!(edn_parse("\\space").unwrap().cst);
}

#[test]
fn unicode() {
    assert_snapshot!(edn_parse("\\u0000").unwrap().cst);
}

#[test]
fn non_unicode_short_err() {
    let error = edn_parse("\\u000").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}

#[test]
fn non_unicode_long_err() {
    let error = edn_parse("\\u00000").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}

#[test]
fn random_err() {
    let error = edn_parse("\\random").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}
