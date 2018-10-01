extern crate nem;
extern crate restson;

#[macro_use]
extern crate serde_derive;

use restson::{RestClient,RestPath,Error};

#[derive(Serialize,Deserialize)]
struct HttpBinAnything {
    method: String,
    url: String,
}

// plain API call without parameters
impl RestPath<()> for HttpBinAnything {
    fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("anything")) }
}

// API call with one u32 parameter (e.g. "http://httpbin.org/anything/1234")
impl RestPath<u32> for HttpBinAnything {
    fn get_path(param: u32) -> Result<String,Error> { Ok(format!("anything/{}", param)) }
}

#[test]
fn test_restson_get() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    let data: HttpBinAnything = client.get(1234).unwrap();
    println!("{:?}", data.url);

    let query = vec![("a", "2"), ("b", "abcd")];
    let querydata: HttpBinAnything = client.get_with(1234, &query).unwrap();
    println!("{:?}", querydata.url);
}
