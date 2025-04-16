use std::str::FromStr;

use scoop_shim::Shim;

#[test]
fn test_parse_args_quoted_comment_line_breaks() {
    let shim = Shim::new(
        String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
        vec!["search".to_string(), "--installed".to_string()],
    );

    let args_quoted_str = include_str!("../tests/fixtures/args_quoted_comment_line_breaks.shim");

    let parsed_shim = Shim::from_str(args_quoted_str).unwrap();

    assert_eq!(parsed_shim, shim);
}

#[test]
fn test_serialize() {
    let shim = Shim::new(
        String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
        vec!["search".to_string(), "--installed".to_string()],
    );

    let serialized_shim = shim.to_string();

    let expected_serialized_shim = include_str!("../tests/fixtures/args_quoted_no_comment.shim");

    assert_eq!(serialized_shim, expected_serialized_shim);
}
