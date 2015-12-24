/* Copyright (C) 2015 Yutaka Kamei */

extern crate urlparse;
use urlparse::*;


#[test]
fn test_quote() {
    assert_eq!(quote("test@example.com", &[]).ok().unwrap(), "test%40example.com");
    assert_eq!(quote("123!'#$%&()", &[]).ok().unwrap(), "123%21%27%23%24%25%26%28%29");
    assert_eq!(quote("/a/テスト !/", &[b'/']).ok().unwrap(), "/a/%E3%83%86%E3%82%B9%E3%83%88%20%21/");
}


#[test]
fn test_quote_plus() {
    assert_eq!(quote_plus("Yutaka Kamei", &[]).ok().unwrap(), "Yutaka+Kamei");
    assert_eq!(quote_plus("/a/テスト !/", &[b'/']).ok().unwrap(), "/a/%E3%83%86%E3%82%B9%E3%83%88+%21/");
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
fn test_quote_unquote() {
    let text1 = "亀井 裕";
    let text2 = "亀井 裕 in Tokyo";
    let text3 = "/a/b/c/亀井 ページ";
    let quoted_text1 = quote(text1, &[]).ok().unwrap();
    let unquoted_text1 = unquote(&quoted_text1).ok().unwrap();
    let quoted_text2 = quote_plus(text2, &[]).ok().unwrap();
    let unquoted_text2 = unquote_plus(&quoted_text2).ok().unwrap();
    let quoted_text3 = quote_plus(text3, &[b'/']).ok().unwrap();
    let unquoted_text3 = unquote_plus(&quoted_text3).ok().unwrap();
    assert_eq!(text1, unquoted_text1);
    assert_eq!(text2, unquoted_text2);
    assert_eq!(text3, unquoted_text3);
}

#[test]
fn test_parse_qs() {
    let map1 = parse_qs("q=%E3%83%86%E3%82%B9%E3%83%88+%E3%83%86%E3%82%B9%E3%83%88&e=utf-8");
    let map2 = parse_qs("a=123&a=90&a=%E4%BA%80%E4%BA%95&b=0;n1;n2");
    let map3 = parse_qs("a=v1&a=v2&b=cv1&c=");
    let q = map1.get(&"q".to_string()).unwrap().get(0).unwrap();
    let e = map1.get(&"e".to_string()).unwrap().get(0).unwrap();
    let a = map2.get(&"a".to_string()).unwrap();
    let b = map2.get(&"b".to_string()).unwrap();
    let n1 = map2.get(&"n1".to_string());
    let n2 = map2.get(&"n2".to_string());
    let first = map3.get_first(&"a".to_string()).unwrap();
    let value = map3.get_from_str("a").unwrap();
    let value_first = map3.get_first_from_str("a").unwrap();
    let vn1 = map3.get_first(&"vn1".to_string());
    let vn2 = map3.get_from_str("vn2");
    let vn3 = map3.get_first_from_str("vn3");
    let c = map3.get_first_from_str("c");
    assert_eq!(*q, "テスト テスト".to_string());
    assert_eq!(*e, "utf-8".to_string());
    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 1);
    assert_eq!(*a.get(2).unwrap(), "亀井");
    assert_eq!(n1, None);
    assert_eq!(n2, None);
    assert_eq!(*first, "v1");
    assert_eq!(value, ["v1".to_string(), "v2".to_string()]);
    assert_eq!(value_first, "v1");
    assert_eq!(vn1, None);
    assert_eq!(vn2, None);
    assert_eq!(vn3, None);
    assert_eq!(c, None);
}
