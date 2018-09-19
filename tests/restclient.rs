extern crate hyper;

use std::io::{self,Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

#[test]
fn test_hyper_get() {
    let url = "http://httpbin.org/status/200".parse().unwrap();
    println!("> get: {}", url);

    let client = Client::new();
    client
        .get(url)
        .map(|res| {
            println!("Response: {}", res.status());
        })
    .map_err(|err| {
        println!("Error: {}", err);
    });
}
