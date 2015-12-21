use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use unquote::unquote_plus;


pub fn parse_qs(s: &str) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
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
