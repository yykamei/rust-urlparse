/* Copyright (C) 2015 Yutaka Kamei */

use std::string::FromUtf8Error;

const ALWAYS_SAFE_BYTES : &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                            abcdefghijklmnopqrstuvwxyz\
                                            0123456789\
                                            _.-";


/// Replaces special characters in string using the %xx escape.
/// Letters, digits, and the characters '_.-' are never quoted.
///
/// # Examples
///
/// ```
/// use urlparse::quote;
///
/// let s = quote("test@example.com", &[]);
/// assert_eq!(s.ok().unwrap(), "test%40example.com");
/// let path = quote("/a/b/c", &[b'/']);
/// assert_eq!(path.ok().unwrap(), "/a/b/c");
/// ```
///
pub fn quote(s: &str, safe: &[u8]) -> Result<String, FromUtf8Error> {
    let mut result : Vec<u8> = Vec::new();
    let items = s.as_bytes();
    let mut _safe = ALWAYS_SAFE_BYTES.to_vec();
    _safe.extend(safe);
    for item in items {
        if _safe.contains(item) {
            result.push(*item);
        } else {
            result.push(b'%');
            result.append(&mut format!("{:02X}", item).as_bytes().to_vec());
        }
    }
    return String::from_utf8(result);
}


/// Like quote(), but also replaces ' ' with '+', as required for quoting HTML form values.
///
/// # Examples
///
/// ```
/// use urlparse::quote_plus;
///
/// let s = quote_plus("a - b = 123", &[]);
/// assert_eq!(s.ok().unwrap(), "a+-+b+%3D+123");
/// ```
///
pub fn quote_plus(s: &str, safe: &[u8]) -> Result<String, FromUtf8Error> {
    let mut _safe : Vec<u8> = safe.to_vec();
    _safe.push(b' ');
    match quote(s, _safe.iter().as_slice()) {
        Ok(result) => return Ok(result.replace(" ", "+")),
        Err(e)     => return Err(e),
    }
}
