extern crate nem;
#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate reqwest;

use clap::{Arg, SubCommand};
use std::io;

pub type App = clap::App<'static, 'static>;

pub fn main() {
    println!("Hello.");

    env_logger::init();
    info!("starting up");

    let matches = cli().get_matches_safe();
    println!("matches {:?}", matches);
    let args = match matches {
        Ok(args) => args,
        Err(e) => {
            panic!(e);
        }
    };

    if args.is_present("list") {
        let url = "http://bigalice2.nem.ninja:7890/node/peer-list/all";
        let mut res = reqwest::get(url).unwrap();
        res.copy_to(&mut io::stdout()).unwrap();
    }
}

fn cli() -> App {
    // https://github.com/evias/nem-cli/blob/master/core/nem-cli.js
    App::new("nem-mini")
        .version("0.1.0")
        .about("commandline nem client")
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("api").arg(Arg::with_name("url").long("url")))
        .subcommand(SubCommand::with_name("wallet").arg(Arg::with_name("address").long("address")))
        .arg(Arg::with_name("node").long("node"))
        .arg(Arg::with_name("port").long("port"))
        .arg(
            Arg::with_name("network")
                .long("network")
                .value_name("network"),
        )
}
