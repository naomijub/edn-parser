#![allow(missing_docs)]

use edn_parser::edn_parse;
use insta::assert_snapshot;

#[test]
fn unterminated_string() {
    let error = edn_parse("\"hello world").unwrap_err();
    let error_json = serde_json::to_string_pretty(&error).unwrap();
    assert_snapshot!(error_json);
}
