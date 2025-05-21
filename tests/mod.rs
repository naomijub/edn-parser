#![allow(missing_docs)]

use edn_parser::*;
use insta::assert_snapshot;

mod error_cases;
mod special_chars;

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

#[test]
fn uuid() {
    assert_snapshot!(
        edn_parse("#uuid \"4877284c-1661-4efe-be83-57d9366700a8\"")
            .unwrap()
            .cst
    );
}

#[test]
fn inst_z() {
    assert_snapshot!(edn_parse("#inst \"1985-04-12T23:20:50.52Z\"").unwrap().cst);
}

#[test]
fn inst_plus_zpne() {
    assert_snapshot!(
        edn_parse("#inst \"1985-04-12T23:20:50+00:00\"")
            .unwrap()
            .cst
    );
}

#[test]
fn inst_sub_zpne() {
    assert_snapshot!(
        edn_parse("#inst \"1985-04-12T23:20:50-00:00\"")
            .unwrap()
            .cst
    );
}

#[test]
fn comment() {
    assert_snapshot!(
        edn_parse(
            "
    ; this is a comment
    123
    ; this is a long ass comment that has numbers 12345678901234567890 and stuff !@#$%^&*()
    "
        )
        .unwrap()
        .cst
    );
}

#[test]
fn symbol() {
    assert_snapshot!(edn_parse("this_is-a_symbol").unwrap().cst);
}
