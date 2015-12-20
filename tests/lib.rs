extern crate urlparse;
use urlparse::{quote, unquote};


#[test]
fn test_quote() {
    assert!(quote("ab亀/q井x", &[b'/']).ok().unwrap() == "ab%E4%BA%80/q%E4%BA%95x");
    assert!(quote("テスト+na", &[b'/']).ok().unwrap() == "%E3%83%86%E3%82%B9%E3%83%88%2Bna");
}


#[test]
fn test_unquote() {
    println!("{}", unquote("q=dark+%E6%97%A5%E6%9C%AC%E8%AA%9E").ok().unwrap());
    assert!(unquote("%E8%91%97%E5%90%8D%E4%BA%BA").ok().unwrap() == "著名人");
    println!("{:?}", unquote("%E8%91%97%E5%90%8D%E4%BA%B").err());
}
