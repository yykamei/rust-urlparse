/* Copyright (C) 2015 Yutaka Kamei */

use query_string::{Query, parse_qs};


const SCHEMA_CHARS : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                     abcdefghijklmnopqrstuvwxyz\
                                     0123456789\
                                     +-.";


#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
pub struct Url {
    /// URL scheme specifier
    pub scheme: String,

    /// Network location part
    pub netloc: String,

    /// Hierarchical path
    pub path: String,

    /// Query component
    pub query: Option<String>,

    /// Fragment identifier
    pub fragment: Option<String>,

    /// User name
    pub username: Option<String>,

    /// Password
    pub password: Option<String>,

    /// Host name (lower case)
    pub hostname: Option<String>,

    /// Port number as integer
    pub port: Option<u16>,
}


impl Url {
    /// Creates a new `Url` initialized with the empty string or None value.
    ///
    pub fn new() -> Url {
        Url {
            scheme: "".to_string(),
            netloc: "".to_string(),
            path: "".to_string(),
            query: None,
            fragment: None,
            username: None,
            password: None,
            hostname: None,
            port: None,
        }
    }

    /// Parse a URL and return `Url` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use urlparse::Url;
    ///
    /// let url = Url::parse("http://Example.com:8080/foo?filter=%28%21%28cn%3Dbar%29%29");
    /// assert_eq!(url.scheme, "http");
    /// assert_eq!(url.netloc, "Example.com:8080");
    /// assert_eq!(url.path, "/foo");
    /// assert_eq!(url.query, Some("filter=%28%21%28cn%3Dbar%29%29".to_string()));
    /// assert_eq!(url.fragment, None);
    /// assert_eq!(url.username, None);
    /// assert_eq!(url.password, None);
    /// assert_eq!(url.hostname, Some("example.com".to_string()));
    /// assert_eq!(url.port, Some(8080));
    ///
    /// let query = match url.get_parsed_query() {
    ///     Some(q) => q,
    ///     None    => panic!("Failed to parse my query"),
    /// };
    /// assert_eq!(query.get(&"filter".to_string()).unwrap().get(0).unwrap(), "(!(cn=bar))");
    /// ```
    ///
    pub fn parse(s: &str) -> Url {
        let (scheme, extra) = match s.find(':') {
            Some(pos) => {
                let (a, b) = s.split_at(pos);
                let mut is_scheme = true;
                for c in a.chars() {
                    if !SCHEMA_CHARS.contains(c) {
                        is_scheme = false;
                        break;
                    }
                }
                let (_a, _b) = if is_scheme { (a, &b[1..]) } else { ("", s) };
                match _b.parse::<u16>() {
                    Ok(_)   => ("", s),  // It is not a scheme because ':'
                                         // after the scheme is port number.
                    Err(_)  => (_a, _b),
                }
            },
            None      => ("", s),
        };
        let (netloc, extra) = match extra.starts_with("//") {
            true  => {
                let _extra = &extra[2..];
                let mut a = _extra;
                let mut b = "";
                let mut delim = !0 as usize;
                for c in "/?#".chars() {
                    match _extra.find(c) {
                        Some(pos) => {
                            if delim >= pos {
                                delim = pos;
                                let pair = _extra.split_at(pos);
                                a = pair.0;
                                b = pair.1;
                            }
                        },
                        None      => continue,
                    }
                }
                (a, b)
            },
            false => ("", extra),
        };
        let (extra, fragment) = match extra.rfind('#') {
            Some(pos) => {
                let (a, b) = extra.split_at(pos);
                (a, &b[1..])
            },
            None      => (extra, ""),
        };
        let (path, query) = match extra.find('?') {
            Some(pos) => {
                let (a, b) = extra.split_at(pos);
                (a, &b[1..])
            },
            None      => (extra, ""),
        };
        let (userinfo, hostinfo) = match netloc.find('@') {
            Some(pos) => {
                let (a, b) = netloc.split_at(pos);
                (a, &b[1..])
            },
            None      => ("", netloc),
        };
        let (username, password) = match userinfo.find(':') {
                Some(pos) => {
                    let (a, b) = userinfo.split_at(pos);
                    (a, &b[1..])
                },
                None       => (userinfo, ""),
        };
        let (hostname, port) = match hostinfo.rfind(|c| c == ':' || c == ']') {
                Some(pos) => {
                    let (a, b) = hostinfo.split_at(pos);
                    let _b = &b[1..];
                    match _b.parse::<u16>() {
                        Ok(number) => (a, number),
                        Err(_)     => (a, 0),
                    }
                },
                None       => (hostinfo, 0),
        };
        let hostname = hostname.trim_matches(|c| c == '[' || c == ']');
        Url {
            scheme: scheme.to_string().to_lowercase(),
            netloc: netloc.to_string(),
            path: path.to_string(),
            query: if query.is_empty() { None } else { Some(query.to_string()) },
            fragment: if fragment.is_empty() { None } else { Some(fragment.to_string()) },
            username: if username.is_empty() { None } else { Some(username.to_string()) },
            password: if password.is_empty() { None } else { Some(password.to_string()) },
            hostname: if hostname.is_empty() { None } else { Some(hostname.to_string().to_lowercase()) },
            port: if port == 0 { None } else { Some(port) },
        }
    }

    /// Return a query object by executing `parse_qs()` with self.query.
    /// If parsing a query fails, None value will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use urlparse::urlparse;
    ///
    /// let url = urlparse("http://www.example.com/?a=123&b=A%20B");
    /// let query = url.get_parsed_query().unwrap();
    /// assert_eq!(query.get(&"b".to_string()).unwrap().get(0).unwrap(), "A B");
    /// ```
    ///
    pub fn get_parsed_query(&self) -> Option<Query> {
        match self.query {
            Some(ref q) => Some(parse_qs(&q)),
            None        => None,
        }
    }
}


/// Parse a URL and return `Url` object. This is synonymous with `Url::parse()`.
///
pub fn urlparse(s: &str) -> Url { Url::parse(s) }
