extern crate urlparse;
use urlparse::{quote, quote_plus, unquote};


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
    assert_eq!(unquote("%E8%91%97%E5%90%8D%E4%BA%BA").ok().unwrap(), "著名人");
}
