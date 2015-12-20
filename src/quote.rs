use  std::string::FromUtf8Error;

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
