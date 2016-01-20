/* Copyright (C) 2016 Yutaka Kamei */

extern crate urlparse;
use urlparse::*;
use std::char::from_u32;


#[test]
fn test_quote() {
    assert_eq!(quote(" ", &[]).ok().unwrap(), "%20");
    assert_eq!(quote("test@example.com", &[]).ok().unwrap(), "test%40example.com");
    assert_eq!(quote("123!'#$%&()", &[]).ok().unwrap(), "123%21%27%23%24%25%26%28%29");
    assert_eq!(quote("/a/テスト !/", &[b'/']).ok().unwrap(), "/a/%E3%83%86%E3%82%B9%E3%83%88%20%21/");
}


#[test]
fn test_quote_plus() {
    assert_eq!(quote_plus("Yutaka Kamei", b"").ok().unwrap(), "Yutaka+Kamei");
    assert_eq!(quote_plus("/a/テスト !/", b"/").ok().unwrap(), "/a/%E3%83%86%E3%82%B9%E3%83%88+%21/");
}


#[test]
fn test_quote_default() {
    let should_quote : String = (0..128)
        .map(|i| from_u32(i as u32).unwrap())
        .filter(|&c| c != '-' && c != '.' && c != '/' && c != '_')
        .filter(|&c| !c.is_alphanumeric())
        .collect();
    for s in should_quote.chars() {
        let original : &str = &format!("{}", s);
        let expect : &str = &format!("%{:02X}", s as u32);
        assert_eq!(quote(original, b"").ok().unwrap(), expect);
    }
}


#[test]
fn test_quote_safe() {
    let should_not_quote = "<>";
    assert_eq!(quote(should_not_quote, should_not_quote.as_bytes()).ok().unwrap(), should_not_quote);
    assert_eq!(quote_plus(should_not_quote, should_not_quote.as_bytes()).ok().unwrap(), should_not_quote);
}


#[test]
fn test_unquote() {
    assert_eq!(unquote("%E4%BA%80%E4%BA%95%20%E8%A3%95").ok().unwrap(), "亀井 裕");
}


#[test]
fn test_unquote_plus() {
    assert_eq!(unquote_plus("%E4%BA%80%E4%BA%95+%E8%A3%95").ok().unwrap(), "亀井 裕");
}


#[test]
fn test_unquoting_badpercent() {
    assert_eq!(unquote("%xab").ok().unwrap(), "%xab");
    assert_eq!(unquote("%X").ok().unwrap(), "%X");
    assert_eq!(unquote("%").ok().unwrap(), "%");
    assert_eq!(unquote_plus("%xab").ok().unwrap(), "%xab");
    assert_eq!(unquote_plus("%X").ok().unwrap(), "%X");
    assert_eq!(unquote_plus("%").ok().unwrap(), "%");
}


#[test]
fn test_unquoting_parts() {
    assert_eq!(unquote("ab%63d").ok().unwrap(), "abcd");
    assert_eq!(unquote_plus("ab%63d").ok().unwrap(), "abcd");
}


#[test]
fn test_quote_unquote() {
    let text1 = "亀井 裕";
    let text2 = "亀井 裕 in Tokyo";
    let text3 = "/a/b/c/亀井 ページ";
    let quoted_text1 = quote(text1, b"").ok().unwrap();
    let unquoted_text1 = unquote(&quoted_text1).ok().unwrap();
    let quoted_text2 = quote_plus(text2, b"").ok().unwrap();
    let unquoted_text2 = unquote_plus(&quoted_text2).ok().unwrap();
    let quoted_text3 = quote_plus(text3, b"/").ok().unwrap();
    let unquoted_text3 = unquote_plus(&quoted_text3).ok().unwrap();
    assert_eq!(text1, unquoted_text1);
    assert_eq!(text2, unquoted_text2);
    assert_eq!(text3, unquoted_text3);
}

