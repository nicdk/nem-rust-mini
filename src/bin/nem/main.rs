extern crate nem;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    println!("Hello.");

    env_logger::init();
    info!("starting up");

    // https://github.com/evias/nem-cli/blob/master/core/nem-cli.js
    let matches = App::new("nem-mini")
        .version("0.1.0")
        .about("commandline nem client")
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("api")
                    .arg(Arg::with_name("url")
                    .long("url")))
        .subcommand(SubCommand::with_name("wallet")
                    .arg(Arg::with_name("address")
                    .long("address")))
        .arg(Arg::with_name("node")
             .long("node"))
        .arg(Arg::with_name("port")
             .long("port"))
        .arg(Arg::with_name("network")
             .long("network")
             .value_name("network"))
        .get_matches();

    println!("matches {:?}", matches);
}
