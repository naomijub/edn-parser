#![allow(missing_docs)]

use edn_parser::*;
use insta::assert_snapshot;

mod error_cases;

#[test]
fn int() {
    assert_snapshot!(edn_parse("123").unwrap().cst);
}

#[test]
fn neg_int() {
    assert_snapshot!(edn_parse("-123").unwrap().cst);
}

#[test]
fn float() {
    assert_snapshot!(edn_parse("12.3").unwrap().cst);
}

#[test]
fn neg_float() {
    assert_snapshot!(edn_parse("-12.3").unwrap().cst);
}

#[test]
fn bool_true() {
    assert_snapshot!(edn_parse("true").unwrap().cst);
}

#[test]
fn bool_false() {
    assert_snapshot!(edn_parse("false").unwrap().cst);
}

#[test]
fn nil() {
    assert_snapshot!(edn_parse("nil").unwrap().cst);
}

#[test]
fn simple_char() {
    assert_snapshot!(edn_parse("\\c").unwrap().cst);
}

#[test]
fn newline_char() {
    assert_snapshot!(edn_parse("\\newline").unwrap().cst);
}

#[test]
fn string() {
    assert_snapshot!(edn_parse("\"hello world\"").unwrap().cst);
}

#[test]
fn list_random_types() {
    assert_snapshot!(edn_parse("(1 nil true -4.2 \"hello world\")").unwrap().cst);
}

#[test]
fn set_random_types() {
    assert_snapshot!(edn_parse("#{1 nil true -4.2 \"hello world\"}").unwrap().cst);
}

#[test]
fn vector_random_types() {
    assert_snapshot!(edn_parse("[1 nil true -4.2 \"hello world\"]").unwrap().cst);
}

#[test]
fn map_random_types() {
    assert_snapshot!(
        edn_parse("{1 nil true -4.2 \"hello world\" :value}")
            .unwrap()
            .cst
    );
}
