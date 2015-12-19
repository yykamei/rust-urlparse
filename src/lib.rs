use  std::string::FromUtf8Error;


#[test]
fn it_works() {
    println!("{}", unquote("q=dark+%E6%97%A5%E6%9C%AC%E8%AA%9E").ok().unwrap());
    assert!(unquote("%E8%91%97%E5%90%8D%E4%BA%BA").ok().unwrap() == "著名人");
    println!("{:?}", unquote("%E8%91%97%E5%90%8D%E4%BA%B").err());
}


pub fn unquote(s: &str) -> Result<String, FromUtf8Error> {
    let mut result : Vec<u8> = Vec::new();
    let mut items = s.as_bytes().split(|&b| b == b'%');
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
                let mut bit : u8 = 0;
                let fs = &item[..2];
                let ls = &item[2..];
                for (i, b) in fs.iter().enumerate() {
                    let d : u8 = match std::char::from_u32(*b as u32) {
                        Some(c) => match c.to_digit(16) {
                            Some(d) => d as u8,
                            None => {
                                result.append(&mut item.to_vec());
                                break;
                            },
                        },
                        None    => {
                            result.append(&mut item.to_vec());
                            break;
                        },
                    };
                    if i == 0 {
                        bit += d * 16
                    } else {
                        bit += d
                    }
                }
                result.push(bit);
                result.append(&mut ls.to_vec());
            },
        }
    }
    return String::from_utf8(result);
}
