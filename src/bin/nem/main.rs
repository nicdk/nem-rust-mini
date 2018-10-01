extern crate nem;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;

use clap::App; 

fn main() {
    println!("Hello.");

    env_logger::init();
    info!("starting up");

    App::new("myapp")
        .version("1.0")
        .about("Does great things!")
        .author("Kevin K.")
        .get_matches();     
}
