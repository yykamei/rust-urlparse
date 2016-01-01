/* Copyright (C) 2015-2016 Yutaka Kamei */

//! This is a URL parsing library.
//!
//! The goal of this project is to provide simple parsing URL library
//! like urllib.parse in Python3.x.
//!
//! # Examples
//!
//! ```
//! use urlparse::urlparse;
//! use urlparse::GetQuery;  // Trait
//!
//! let url = urlparse("http://www.example.com/foo?bar=123&col=println%21%28%22TEST%21%22%29&col=sub");
//! let query = url.get_parsed_query().unwrap();
//! assert_eq!(url.scheme, "http");
//! assert_eq!(query.get_first_from_str("col").unwrap(), "println!(\"TEST!\")");
//! ```

mod quote;
mod unquote;
mod query_string;
mod url;
pub use quote::quote;
pub use quote::quote_plus;
pub use unquote::unquote;
pub use unquote::unquote_plus;
pub use query_string::parse_qs;
pub use query_string::{Query, QueryValue, GetQuery};
pub use url::urlparse;
pub use url::urlunparse;
pub use url::Url;
