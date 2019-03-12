//!# URLTemplate
//! Utility that enables URLs with placeholders, i.e. `https://www.mozilla.org/?utm_source={source}&utm_medium={medium}
//!# Usage
//! ```
//! extern crate urltemplate;
//!
//! use urltemplate::UrlTemplate;
//! use std::collections::HashMap;
//!
//! let mut params = HashMap::new();
//! params.insert("source".to_string(), "url-template-crate-❤".to_string());
//! let url_with_placeholders = UrlTemplate::from("https://www.mozilla.org/?utm_source={source}");
//! let url =  url_with_placeholders.substitute_str(&params).expect("valid url");
//! assert_eq!(url, "https://www.mozilla.org/?utm_source=url-template-crate-❤")
//! ```
use url::Url;

use std::fmt;

use std::collections::{HashMap};
use std::error::{self, Error};

use std::ops::Add;



#[derive(Debug, Clone)]
pub struct UrlTemplate(pub String);


#[derive(Debug)]
#[derive(PartialEq)]
pub enum UrlTemplateErrorKind {
    /// provided String is not an URL
    IsNotAnUrl,
    /// provided URL scheme is differ from expected `http` or `https`
    InvalidScheme,
    /// provided pattern has incorrect syntax
    InvalidPattern
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct UrlTemplateError {
    position: usize,
    kind: UrlTemplateErrorKind
}

impl From<(UrlTemplateErrorKind, usize)> for UrlTemplateError {
    fn from((kind, position): (UrlTemplateErrorKind, usize)) -> UrlTemplateError {
        UrlTemplateError {
            kind: kind,
            position: position
        }
    }
}

impl From<UrlTemplateErrorKind> for UrlTemplateError {
    fn from(kind: UrlTemplateErrorKind) -> UrlTemplateError {
        UrlTemplateError {
            kind: kind,
            position: 0
        }
    }
}

impl fmt::Display for UrlTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            UrlTemplateErrorKind::IsNotAnUrl => {
                write!(f, "Provided pattern is not a valid URL.")
            }
            UrlTemplateErrorKind::InvalidScheme => {
                write!(f, "URL scheme is differ from expected `http` or `https`.")
            }
            UrlTemplateErrorKind::InvalidPattern => {
                write!(f, "The pattern has invalid syntax.")
            }
        }
    }
}

impl error::Error for UrlTemplateError {
    fn cause(&self) -> Option<&Error> { None }
    fn source(&self) -> Option<&(Error + 'static)> { None }
}


impl From<String> for UrlTemplate {
    fn from(s: String) -> UrlTemplate {
        UrlTemplate(s)
    }
}

impl From<&str> for UrlTemplate {
    fn from(s: &str) -> UrlTemplate {
        UrlTemplate(String::from(s))
    }
}

impl Into<String> for UrlTemplate {
    fn into(self) -> String {
        self.0.clone()
    }
}

impl PartialEq for UrlTemplate {
    fn eq(&self, other: &UrlTemplate) -> bool {
        let s: String = other.into();
        self.0 == s
    }
}

impl ToString for UrlTemplate {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<&UrlTemplate> for String {
    fn from(tpl: &UrlTemplate) -> String { tpl.to_string() }
}

impl UrlTemplate {
    pub fn substitute(&self, values: &HashMap<String, String>) -> Result<Url, UrlTemplateError> {
        match self.substitute_str(values) {
            Ok(url_string) => {
                Ok(Url::parse(url_string.as_str()).expect("Valid URL string"))
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn substitute_str(&self, values: &HashMap<String, String>) -> Result<String, UrlTemplateError> {
        // sanity check
        match Url::parse(self.0.as_str()) {
            Ok(parsed) => {
                let scheme_valid = parsed.scheme() == "http" || parsed.scheme() == "https";
                if !scheme_valid {
                    return Err(UrlTemplateError::from(UrlTemplateErrorKind::InvalidScheme));
                }
            }
            _ => {
                return Err(UrlTemplateError::from(UrlTemplateErrorKind::IsNotAnUrl));
            }
        }

        let mut chars = self.0.char_indices();
        let mut out = String::new();

        let mut current_placeholder = String::new();
        let mut inside_placeholder = false;

        loop{
            match chars.next() {
                None => {
                    break
                }
                Some((charnum, '{')) => {
                    if inside_placeholder {
                        return Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, charnum)));
                    }
                    current_placeholder = String::new();
                    inside_placeholder = true;
                }
                Some((charnum, '}')) => {
                    if !inside_placeholder {
                        return Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, charnum)));
                    }

                    match values.get(&current_placeholder) {
                        Some(s) => {
                            out = out.add(s);
                        }
                        None => {
                            out = out.add("");
                        }
                    }
                    inside_placeholder = false;
                }
                Some((_charnum, ch)) => {
                    if inside_placeholder {
                        current_placeholder.push(ch);
                    } else {
                        out.push(ch);
                    }
                }
            }
        }

        if inside_placeholder {
            return Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, self.0.len() - 1)));
        }

        Ok(out)

    }
}
