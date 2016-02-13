/* Copyright (C) 2015 Yutaka Kamei */

extern crate urlparse;
use urlparse::*;
use std::collections::HashMap;
use std::iter::FromIterator;


#[test]
fn test_qsl() {
    let query = parse_qs("");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("&");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("&&");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("=");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("=a");
    assert_eq!(query, HashMap::from_iter(vec![("".to_string(), vec!["a".to_string()])]));
    let query = parse_qs("a");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("a=");
    assert_eq!(query, HashMap::new());
    let query = parse_qs("&a=b");
    assert_eq!(query, HashMap::from_iter(vec![("a".to_string(), vec!["b".to_string()])]));
    let query = parse_qs("a=a+b&b=b+c");
    assert_eq!(query, HashMap::from_iter(vec![("a".to_string(), vec!["a b".to_string()]),
                                              ("b".to_string(), vec!["b c".to_string()])]));
    let query = parse_qs("a=1&a=2");
    assert_eq!(query, HashMap::from_iter(vec![("a".to_string(), vec!["1".to_string(), "2".to_string()])]));
}


#[test]
fn test_parse_qs() {
    let map1 = parse_qs("q=%E3%83%86%E3%82%B9%E3%83%88+%E3%83%86%E3%82%B9%E3%83%88&e=utf-8");
    let map2 = parse_qs("a=123&a=90&a=%E4%BA%80%E4%BA%95&b=0;n1;n2");
    let map3 = parse_qs("a=v1&a=v2&b=cv1&c=".to_string());
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
