#![allow(missing_docs)]

use edn_parser::edn_parse;
use insta::assert_snapshot;

#[test]
fn unterminated_string() {
    let error = edn_parse("\"hello world").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}

#[test]
fn float_num_rational_not_supported() {
    let error = edn_parse("42.3/12").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}

#[test]
fn float_denom_rational_not_supported() {
    let error = edn_parse("42/12.3").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}
