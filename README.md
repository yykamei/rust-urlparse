# rust-urlparse

[![Build Status](https://travis-ci.org/yykamei/rust-urlparse.svg?branch=master)](https://travis-ci.org/yykamei/rust-urlparse)

This is a URL parsing library written in Rust.

The goal of this project is to provide simple parsing URL library
like urllib.parse in Python3.x.

## Example

### Source code

``` rust
extern crate urlparse;
use urlparse::urlparse;
    
fn main() {
    let url = urlparse("http://www.example.com/foo?bar=123");
    println!("{:?}", url);
}
```

### Output

```
Url { scheme: "http", netloc: "www.example.com", path: "/foo", query: Some("bar=123"), fragment: None, username: None, password: None, hostname: Some("www.example.com"), port: None }
```


## License

MIT

## Installation

If you're using Cargo, just add urlparse to your Cargo.toml:

    [dependencies.urlparse]
    version = "*"

## Documentation

The documentation is hosted [online][urlparse-doc].

[urlparse-doc]: https://yykamei.github.io/rust-urlparse
