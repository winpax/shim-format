# Scoop Shims

## Usage

```rust
use scoop_shim::Shim;

// Serialize a shim
let shim = Shim::new(
    String::from("sfsu.exe"),
    vec!["search".to_string()],
);

shim.to_string(); // "path = \"sfsu.exe\"\r\nargs = search"

// Deserialize a shim
let shim = scoop_shim::from_str("path = \"sfsu.exe\"\r\nargs = search").unwrap();

assert_eq!(shim.path(), String::from("sfsu.exe"));
assert_eq!(shim.args(), ["search".to_string()]);

```

**Made with 💗 by Juliette Cordor**
