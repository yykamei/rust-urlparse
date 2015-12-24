/* Copyright (C) 2015 Yutaka Kamei */

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use unquote::unquote_plus;

type QueryValue = Vec<String>;
pub type Query = HashMap<String, QueryValue>;


pub trait GetQuery {
    fn get_first(&self, k: &String) -> Option<&String>;
    fn get_from_str(&self, k: &str) -> Option<QueryValue>;
    fn get_first_from_str(&self, k: &str) -> Option<String>;
}


impl GetQuery for Query {
    fn get_first(&self, k: &String) -> Option<&String> {
        match self.get(k) {
            Some(value) => value.get(0),
            None        => None,
        }
    }

    fn get_from_str(&self, k: &str) -> Option<QueryValue> {
        match self.get(&k.to_string()) {
            Some(value) => Some(value.iter().map(|e| e.to_string()).collect()),
            None        => None,
        }
    }

    fn get_first_from_str(&self, k: &str) -> Option<String> {
        match self.get(&k.to_string()) {
            Some(value) => match value.get(0) {
                Some(string) => Some(string.to_string()),
                None         => None,
            },
            None        => None,
        }
    }
}


/// Parse a query given as a string argument.
///
/// # Examples
///
/// ```
/// use urlparse::parse_qs;
///
/// let map = parse_qs("a=123&a=90&a=%E4%BA%80%E4%BA%95&b=0;n1;n2");
/// let a = map.get(&"a".to_string()).unwrap();
/// let b = map.get(&"b".to_string()).unwrap();
/// assert_eq!(a.len(), 3);
/// assert_eq!(b.len(), 1);
/// assert_eq!(*a.get(2).unwrap(), "亀井");
/// ```
///
pub fn parse_qs(s: &str) -> Query {
    let mut map : Query = Query::new();
    for item in s.split(|c| c == '&' || c == ';') {
        match item.find('=') {
            Some(index) => {
                let (key, value) = item.split_at(index);
                let _key = match unquote_plus(key) {
                    Ok(k)  => k,
                    Err(_) => continue,  // FIXME: This is not strict mode.
                };
                let _value = match unquote_plus(value.trim_left_matches('=')) {
                    Ok(v)  => v,
                    Err(_) => continue,  // FIXME: This is not strict mode.
                };
                if _value.is_empty() {
                    continue;
                }
                let mut result = match map.entry(_key) {
                    Vacant(entry)   => entry.insert(Vec::new()),
                    Occupied(entry) => entry.into_mut(),
                };
                result.push(_value);
            },
            None        => continue,
        }
    }
    return map;
}
