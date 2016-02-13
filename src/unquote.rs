/* Copyright (C) 2015 Yutaka Kamei */

use std::string::FromUtf8Error;

const OFFSET : usize = 6;
const DIGIT : &'static [u8] = b"0123456789ABCDEFabcdef";

/// Replaces %xx escapes by their single-character equivalent.
///
/// # Examples
///
/// ```
/// use urlparse::unquote;
///
/// let s = unquote("ABC%3D123%21%20DEF%3D%23%23");
/// assert_eq!(s.ok().unwrap(), "ABC=123! DEF=##");
/// ```
///
pub fn unquote<S: AsRef<str>>(s: S) -> Result<String, FromUtf8Error> {
    let mut result : Vec<u8> = Vec::new();
    let mut items = s.as_ref().as_bytes().split(|&b| b == b'%');
    match items.next() {
        Some(item) => result.append(&mut item.to_vec()),
        None       => return String::from_utf8(result),
    }
    for item in items {
        match item.len() {
            0 => result.push(b'%'),
            1 => {
                    result.push(b'%');
                    result.append(&mut item.to_vec());
            },
            _ => {
                let fs = &item[..2];
                let ls = &item[2..];
                if let Some(digit) = to_digit(*fs.get(0).unwrap(), *fs.get(1).unwrap()) {
                    result.push(digit);
                    result.append(&mut ls.to_vec());
                } else {
                    result.push(b'%');
                    result.append(&mut item.to_vec());
                }
            },
        }
    }
    return String::from_utf8(result);
}

/// Like unquote(), but also replaces plus signs by spaces, as required for
/// unquoting HTML form values.
///
/// # Examples
///
/// ```
/// use urlparse::unquote_plus;
///
/// let s = unquote_plus("ABC%3D123%21+DEF%3D%23%23");
/// assert_eq!(s.ok().unwrap(), "ABC=123! DEF=##");
/// ```
///
pub fn unquote_plus<S: AsRef<str>>(s: S) -> Result<String, FromUtf8Error> {
    let _s = s.as_ref().replace("+", " ");
    return unquote(_s);
}


fn to_digit(n1: u8, n2: u8) -> Option<u8> {
    let mut byte : u8 = 0;
    match DIGIT.binary_search(&n1) {
        Ok(_n1) => byte += if _n1 < 16 {_n1 as u8 * 16} else {(_n1 - OFFSET) as u8 * 16},
        Err(_)  => return None,
    }
    match DIGIT.binary_search(&n2) {
        Ok(_n2) => byte += if _n2 < 16 {_n2 as u8} else {(_n2 - OFFSET) as u8},
        Err(_)  => return None,
    }
    return Some(byte);
}


#[test]
fn test_to_digit() {
    assert_eq!(to_digit(b'1', b'2'), Some(0x12));
    assert_eq!(to_digit(b'e', b'a'), Some(0xEA));
    assert_eq!(to_digit(b'E', b'A'), Some(0xEA));
    assert_eq!(to_digit(b'F', b'F'), Some(0xFF));
    assert_eq!(to_digit(b'X', b'1'), None);
    assert_eq!(to_digit(b'A', b'x'), None);
}
