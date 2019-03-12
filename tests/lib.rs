#[cfg(test)]
use urltemplate::{UrlTemplate, UrlTemplateError, UrlTemplateErrorKind};

use std::collections::HashMap;


#[test]
fn test_substitute_str() {
    let mut params = HashMap::new();
    params.insert("key1".to_string(), "value1".to_string());
    params.insert("key2".to_string(), "value2".to_string());

    assert_eq!(UrlTemplate::from("data:text/plain,Hello?World#").substitute_str(&params), Err(UrlTemplateError::from(UrlTemplateErrorKind::InvalidScheme)));
    assert_eq!(UrlTemplate::from("Ôunfortunately").substitute_str(&params), Err(UrlTemplateError::from(UrlTemplateErrorKind::IsNotAnUrl)));

    assert_eq!(UrlTemplate::from("http://google.com/?utm_source={key1}&utm_medium={key2}&someotherparam={key3}").substitute_str(&params).expect("ok"), "http://google.com/?utm_source=value1&utm_medium=value2&someotherparam=");
    assert_eq!(UrlTemplate::from("http://google.com").substitute_str(&params).expect("ok"), String::from("http://google.com"));

    assert_eq!(UrlTemplate::from("http://google.com/?utm_source={key1{}").substitute_str(&params), Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, 35))));
    assert_eq!(UrlTemplate::from("http://google.com/?utm_source=key1{").substitute_str(&params), Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, 34))));
    assert_eq!(UrlTemplate::from("http://google.com/?utm_source=key1}").substitute_str(&params), Err(UrlTemplateError::from((UrlTemplateErrorKind::InvalidPattern, 34))));

    assert_eq!(UrlTemplate::from("http://google.com/?utm_source={key1}"), String::from("http://google.com/?utm_source={key1}").into());
    assert_eq!(UrlTemplate::from("http://google.com/?utm_source={key1}&subid={key2}"), String::from("http://google.com/?utm_source={key1}&subid={key2}").into());
}
