#![deny(warnings)]
extern crate hyper;

extern crate reqwest;
use std::io;

#[test]
fn test_nem_heartbeat() {
    let url = "http://bigalice2.nem.ninja:7890/heartbeat";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_status() {
    let url = "http://bigalice2.nem.ninja:7890/status";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
