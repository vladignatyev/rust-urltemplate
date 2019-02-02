use std::collections::HashMap;
use std::ops::Add;

extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate urltemplate;
use urltemplate::UrlTemplate;


fn bench(c: &mut Criterion) {
    c.bench_function("substitute", move |b| {
        let mut params = HashMap::new();
        let mut url_with_placeholders = UrlTemplate::from("http://example.com/?");
        for i in 0..15 {
            params.insert(format!("subid{}", i), format!("value{}", i));
            url_with_placeholders = url_with_placeholders.add(&format!("subid{}", i));
        }

        b.iter(|| {
            url_with_placeholders.substitute(&params).expect("valid url");
        });
    });

    c.bench_function("substitute_str", move |b| {
        let mut params = HashMap::new();
        let mut url_with_placeholders = UrlTemplate::from("http://example.com/?");
        for i in 0..15 {
            params.insert(format!("subid{}", i), format!("value{}", i));
            url_with_placeholders = url_with_placeholders.add(&format!("subid{}", i));
        }

        b.iter(|| {
            url_with_placeholders.substitute_str(&params).expect("valid url");
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
