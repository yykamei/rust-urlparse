/* Copyright (C) 2015 Yutaka Kamei */

extern crate urlparse;
use urlparse::*;


#[test]
fn test_urlparse_rfc2368() {
    let url = Url::parse("mailto:1337@example.org");
    assert_eq!(url.scheme, "mailto");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "1337@example.org");
    assert_eq!(url.username, None);
    assert_eq!(url.hostname, None);
    assert_eq!(url.port, None);
}


#[test]
fn test_urlparse_rfc2732() {
    let url = Url::parse("http://Test.python.org:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "Test.python.org:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("test.python.org".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://12.34.56.78:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "12.34.56.78:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("12.34.56.78".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[::1]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::1]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::1".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[dead:beef::1]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::1]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::1".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[dead:beef::]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[dead:beef:cafe:5417:affe:8FA3:deaf:feed]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef:cafe:5417:affe:8FA3:deaf:feed]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef:cafe:5417:affe:8fa3:deaf:feed".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[::12.34.56.78]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::12.34.56.78]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::12.34.56.78".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://[::ffff:12.34.56.78]:5432/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::ffff:12.34.56.78]:5432");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::ffff:12.34.56.78".to_string()));
    assert_eq!(url.port, Some(5432));
    let url = Url::parse("http://Test.python.org/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "Test.python.org");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("test.python.org".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://12.34.56.78/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "12.34.56.78");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("12.34.56.78".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::1]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::1]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::1".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef::1]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::1]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::1".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef::]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef:cafe:5417:affe:8FA3:deaf:feed]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef:cafe:5417:affe:8FA3:deaf:feed]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef:cafe:5417:affe:8fa3:deaf:feed".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::12.34.56.78]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::12.34.56.78]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::12.34.56.78".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::ffff:12.34.56.78]/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::ffff:12.34.56.78]");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::ffff:12.34.56.78".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://Test.python.org:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "Test.python.org:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("test.python.org".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://12.34.56.78:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "12.34.56.78:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("12.34.56.78".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::1]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::1]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::1".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef::1]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::1]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::1".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef::]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef::]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef::".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[dead:beef:cafe:5417:affe:8FA3:deaf:feed]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[dead:beef:cafe:5417:affe:8FA3:deaf:feed]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("dead:beef:cafe:5417:affe:8fa3:deaf:feed".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::12.34.56.78]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::12.34.56.78]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::12.34.56.78".to_string()));
    assert_eq!(url.port, None);
    let url = Url::parse("http://[::ffff:12.34.56.78]:/foo/");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "[::ffff:12.34.56.78]:");
    assert_eq!(url.path, "/foo/");
    assert_eq!(url.hostname, Some("::ffff:12.34.56.78".to_string()));
    assert_eq!(url.port, None);
}


#[test]
fn test_urlparse_noslash() {
    let url = Url::parse("http://example.com?blahblah=/foo");
    let query = url.get_parsed_query().unwrap();
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "example.com");
    assert_eq!(url.path, "");
    assert_eq!(query.get_first_from_str("blahblah").unwrap(), "/foo");
}


#[test]
fn test_urlparse_withoutscheme() {
    let url = Url::parse("path");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "path");
    let url = Url::parse("//www.python.org:80");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "www.python.org:80");
    assert_eq!(url.path, "");
    let url = Url::parse("http://www.python.org:80");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "www.python.org:80");
    assert_eq!(url.path, "");
}


#[test]
fn test_urlparse_portseparator() {
    let url = Url::parse("path:80");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "path:80");
    let url = Url::parse("http:");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "");
    let url = Url::parse("http://www.python.org:80");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "www.python.org:80");
    assert_eq!(url.path, "");
}



#[test]
fn test_urlparse_anyscheme() {
    let url = Url::parse("file:///tmp/junk.txt");
    assert_eq!(url.scheme, "file");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "/tmp/junk.txt");
    assert_eq!(url.hostname, None);
    let url = Url::parse("imap://mail.python.org/mbox1");
    assert_eq!(url.scheme, "imap");
    assert_eq!(url.netloc, "mail.python.org");
    assert_eq!(url.path, "/mbox1");
    assert_eq!(url.hostname, Some("mail.python.org".to_string()));
    let url = Url::parse("mms://wms.sys.hinet.net/cts/Drama/09006251100.asf");
    assert_eq!(url.scheme, "mms");
    assert_eq!(url.netloc, "wms.sys.hinet.net");
    assert_eq!(url.path, "/cts/Drama/09006251100.asf");
    assert_eq!(url.hostname, Some("wms.sys.hinet.net".to_string()));
    let url = Url::parse("nfs://server/path/to/file.txt");
    assert_eq!(url.scheme, "nfs");
    assert_eq!(url.netloc, "server");
    assert_eq!(url.path, "/path/to/file.txt");
    assert_eq!(url.hostname, Some("server".to_string()));
    let url = Url::parse("svn+ssh://svn.zope.org/repos/main/ZConfig/trunk/");
    assert_eq!(url.scheme, "svn+ssh");
    assert_eq!(url.netloc, "svn.zope.org");
    assert_eq!(url.path, "/repos/main/ZConfig/trunk/");
    assert_eq!(url.hostname, Some("svn.zope.org".to_string()));
    let url = Url::parse("git+ssh://git@github.com/user/project.git");
    assert_eq!(url.scheme, "git+ssh");
    assert_eq!(url.netloc, "git@github.com");
    assert_eq!(url.path, "/user/project.git");
    assert_eq!(url.username, Some("git".to_string()));
    assert_eq!(url.hostname, Some("github.com".to_string()));
    let url = Url::parse("s3://foo.com/stuff");
    assert_eq!(url.scheme, "s3");
    assert_eq!(url.netloc, "foo.com");
    assert_eq!(url.path, "/stuff");
    let url = Url::parse("x-newscheme://foo.com/stuff");
    assert_eq!(url.scheme, "x-newscheme");
    assert_eq!(url.netloc, "foo.com");
    assert_eq!(url.path, "/stuff");
    let url = Url::parse("x-newscheme://foo.com/stuff?query#fragment");
    assert_eq!(url.scheme, "x-newscheme");
    assert_eq!(url.netloc, "foo.com");
    assert_eq!(url.path, "/stuff");
    assert_eq!(url.query, Some("query".to_string()));
    assert_eq!(url.fragment, Some("fragment".to_string()));
    let url = Url::parse("x-newscheme://foo.com/stuff?query");
    assert_eq!(url.scheme, "x-newscheme");
    assert_eq!(url.netloc, "foo.com");
    assert_eq!(url.path, "/stuff");
    assert_eq!(url.query, Some("query".to_string()));
    assert_eq!(url.fragment, None);
}


#[test]
fn test_urlparse_withoutnetloc() {
    let url = Url::parse("sip:alice@atlanta.com;maddr=239.255.255.1;ttl=15");
    assert_eq!(url.scheme, "sip");
    assert_eq!(url.netloc, "");
    assert_eq!(url.username, None);
    assert_eq!(url.password, None);
    assert_eq!(url.hostname, None);
    assert_eq!(url.port, None);
}


#[test]
fn test_urlparse_fragments() {
    let url = Url::parse("http:#frag");
    assert_eq!(url.scheme, "http");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "");
    assert_eq!(url.fragment, Some("frag".to_string()));
    let url = Url::parse("//example.net#frag");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "example.net");
    assert_eq!(url.path, "");
    assert_eq!(url.fragment, Some("frag".to_string()));
    let url = Url::parse("index.html#frag");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "index.html");
    assert_eq!(url.fragment, Some("frag".to_string()));
    let url = Url::parse("?a=b#frag");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "");
    assert_eq!(url.fragment, Some("frag".to_string()));
    let url = Url::parse("#frag");
    assert_eq!(url.scheme, "");
    assert_eq!(url.netloc, "");
    assert_eq!(url.path, "");
    assert_eq!(url.fragment, Some("frag".to_string()));
}
