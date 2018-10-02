#![deny(warnings)]
extern crate reqwest;
use std::io;

#[test]
fn test_reqwest() {
    let mut res = reqwest::get("http://httpbin.org/ip").unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
