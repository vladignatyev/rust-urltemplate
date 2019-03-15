# Urltemplate
Convert URL template containing placeholders into url.
Whenever you need to turn `https://example.com/?utm_source={source}&key2={value2}` into URL like `https://example.com/?utm_source=github&key2=`

Core idea is to have minimalistic and comfortable to use placeholders for http/https URLs.
Comfortable means to be easily used in any web-related project, UTF-safe.

## Warning: This crate is experimental. Also, it is being actively developed.

# Usage
```rust
extern crate urltemplate;

use urltemplate::UrlTemplate;

use std::collections::HashMap;
let mut params = HashMap::new();
params.insert("source".to_string(), "url-template-crate-❤".to_string());

let url_with_placeholders = UrlTemplate::from("https://www.mozilla.org/?utm_source={source}");

let url_as_string =  url_with_placeholders.substitute_str(&params).expect("valid url");
let url_as_url =  url_with_placeholders.substitute(&params).expect("valid url");

assert_eq!(url_as_string, "https://www.mozilla.org/?utm_source=url-template-crate-❤");
assert_eq!(url_as_url.query(), Some("utm_source=url-template-crate-❤"));
```

# Notes
* UTF safe
* no external dependencies (only `url` crate is required)
* compact and clean implementation
