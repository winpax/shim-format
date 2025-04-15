use core::str::FromStr;
crate::prelude!();

use crate::Shim;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing key")]
    MissingKey,
    #[error("missing value")]
    MissingValue,
    #[error("key must be a valid identifier (path or args)")]
    InvalidKey,

    #[error("path is missing. this is a required field")]
    MissingPath,
}

pub fn parse_line(line: &str) -> Result<Option<(&str, &str)>, Error> {
    // Allow for empty lines and comments
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }

    let mut parts = line.splitn(2, '=');

    let key = parse_possible_string(parts.next().ok_or(Error::MissingKey)?.trim());
    let value = parse_possible_string(parts.next().ok_or(Error::MissingValue)?.trim());

    Ok(Some((key, value)))
}

fn parse_possible_string(key: &str) -> &str {
    key.trim_matches('"')
}

impl FromStr for Shim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path = None;
        let mut args = Vec::new();

        for line in s.lines() {
            let Some((key, value)) = parse_line(line)? else {
                continue;
            };

            match key {
                "path" => path = Some(parse_path(value)),
                "args" => args = parse_args(value),
                _ => return Err(Error::InvalidKey),
            }
        }

        let Some(path) = path else {
            return Err(Error::MissingPath);
        };

        Ok(Shim { path, args })
    }
}

fn parse_path(path: &str) -> String {
    String::from(path)
}

fn parse_args(args: &str) -> Vec<String> {
    args.split(' ').map(ToString::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_quoted() {
        let args_quoted_shim = Shim {
            path: String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
            args: vec!["search".to_string(), "--installed".to_string()],
        };

        let args_quoted_shim_str = include_str!("../tests/fixtures/args_quoted.shim");

        let parsed_shim = Shim::from_str(args_quoted_shim_str).unwrap();

        assert_eq!(parsed_shim, args_quoted_shim);
    }

    #[test]
    fn test_parse_args_quoted_no_comment() {
        let args_quoted_shim = Shim {
            path: String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
            args: vec!["search".to_string(), "--installed".to_string()],
        };

        let args_quoted_shim_str = include_str!("../tests/fixtures/args_quoted_no_comment.shim");

        let parsed_shim = Shim::from_str(args_quoted_shim_str).unwrap();

        assert_eq!(parsed_shim, args_quoted_shim);
    }

    #[test]
    fn test_parse_no_args_quoted_no_comment() {
        let no_args_quoted_shim = Shim {
            path: String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
            args: vec![],
        };

        let no_args_quoted_shim_str =
            include_str!("../tests/fixtures/no_args_quoted_no_comment.shim");

        let parsed_shim = Shim::from_str(no_args_quoted_shim_str).unwrap();

        assert_eq!(parsed_shim, no_args_quoted_shim);
    }

    #[test]
    fn test_parse_no_args_quoted() {
        let no_args_quoted_shim = Shim {
            path: String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
            args: vec![],
        };

        let no_args_quoted_shim_str = include_str!("../tests/fixtures/no_args_quoted.shim");

        let parsed_shim = Shim::from_str(no_args_quoted_shim_str).unwrap();

        assert_eq!(parsed_shim, no_args_quoted_shim);
    }

    #[test]
    fn test_parse_args_quoted_comment_line_breaks() {
        let args_quoted_shim = Shim {
            path: String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe"),
            args: vec!["search".to_string(), "--installed".to_string()],
        };

        let args_quoted_str =
            include_str!("../tests/fixtures/args_quoted_comment_line_breaks.shim");

        let parsed_shim = Shim::from_str(args_quoted_str).unwrap();

        assert_eq!(parsed_shim, args_quoted_shim);
    }

    #[test]
    fn test_parse_possible_string() {
        let quoted_string = parse_possible_string("\"path\"");
        assert_eq!(quoted_string, "path");

        let unquoted_string = parse_possible_string("path");
        assert_eq!(unquoted_string, "path");
    }

    #[test]
    fn test_path_parsing() {
        let path = parse_path("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe");
        assert_eq!(
            path,
            String::from("<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe")
        );
    }

    #[test]
    fn test_args_parsing() {
        let args = parse_args("search --installed");
        assert_eq!(args, vec!["search".to_string(), "--installed".to_string()]);
    }

    #[test]
    fn test_parse_line() {
        let line = "path = \"<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe\"";

        let Some((key, value)) = parse_line(line).unwrap() else {
            panic!("parse_line should return a Some");
        };

        assert_eq!(key, "path");
        assert_eq!(value, "<SCOOP_PATH>\\apps\\sfsu-beta\\current\\sfsu.exe");
    }
}
