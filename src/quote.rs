use std::string::FromUtf8Error;

const ALWAYS_SAFE_BYTES : &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                            abcdefghijklmnopqrstuvwxyz\
                                            0123456789\
                                            _.-";


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
            result.append(&mut format!("{:X}", item).as_bytes().to_vec());
        }
    }
    return String::from_utf8(result);
}


pub fn quote_plus(s: &str, safe: &[u8]) -> Result<String, FromUtf8Error> {
    let mut _safe : Vec<u8> = safe.to_vec();
    _safe.push(b' ');
    match quote(s, _safe.iter().as_slice()) {
        Ok(result) => return Ok(result.replace(" ", "+")),
        Err(e)     => return Err(e),
    }
}
