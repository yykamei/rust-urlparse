/* Copyright (C) 2015 Yutaka Kamei */
#![feature(test)]

extern crate urlparse;
extern crate test;
use urlparse::*;
use test::Bencher;


#[bench]
fn bench_quote(b: &mut Bencher) {
    b.iter(|| quote("/a/テスト !/", &[b'/']));
}


#[bench]
fn bench_quote_plus(b: &mut Bencher) {
    b.iter(|| quote_plus("/a/テスト !/", &[b'/']));
}


#[bench]
fn bench_unquote(b: &mut Bencher) {
    b.iter(|| unquote("/a/%E3%83%86%E3%82%B9%E3%83%88%20%21/"));
}


#[bench]
fn bench_unquote_plus(b: &mut Bencher) {
    b.iter(|| unquote_plus("/a/%E3%83%86%E3%82%B9%E3%83%88%20%21/"));
}


#[bench]
fn bench_parse_qs(b: &mut Bencher) {
    b.iter(|| parse_qs("q=%E3%83%86%E3%82%B9%E3%83%88+%E3%83%86%E3%82%B9%E3%83%88&e=utf-8"));
}


#[bench]
fn bench_urlparse(b: &mut Bencher) {
    b.iter(|| urlparse("http://Example.com:8080/foo?filter=%28%21%28cn%3Dbar%29%29"));
}


#[bench]
fn bench_urlunparse(b: &mut Bencher) {
    b.iter(|| {
        let url = Url::new();
        let url = Url{
            scheme: "http".to_string(),
            netloc: "www.example.com".to_string(),
            path: "/foo".to_string(),
            query: Some("filter=%28%21%28cn%3Dbar%29%29".to_string()),
            .. url};
        urlunparse(url)
    });
}
