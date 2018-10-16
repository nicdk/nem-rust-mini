extern crate nem;
#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate reqwest;
extern crate json_pretty;
extern crate url;

use clap::{Arg, SubCommand};
use json_pretty::PrettyFormatter;
use url::Url;

type App = clap::App<'static, 'static>;
type Exitcode = u8;

pub fn main() {
    println!("Hello.");

    env_logger::init();
    info!("starting up");

    cli_main().unwrap();
}

fn cli_main() -> Result<(), Exitcode> {
    let matches = cli().get_matches_safe();
    println!("matches {:?}", matches);
    let args = match matches {
        Ok(args) => args,
        Err(_e) => {
            return Err(255);
        }
    };

    if args.is_present("list") {
        list();
        return Ok(());
    } else if args.is_present("api") {
        api("/status");
        return Ok(());
    }

    Ok(())
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

fn list() {
    let url = "http://bigalice2.nem.ninja:7890/node/peer-list/all";
    let mut res = reqwest::get(url).unwrap();
    let formatter = PrettyFormatter::from_string(&res.text().unwrap());
    println!("{}", formatter.pretty());
}

fn api(s: &str) {
    let base = Url::parse("http://bigalice2.nem.ninja:7890/").unwrap();
    let url = base.join(s).unwrap();
    let client = reqwest::Client::new();
    let mut res = client.get(url).send().unwrap();
    let formatter = PrettyFormatter::from_string(&res.text().unwrap());
    println!("{}", res.url().path());
    println!("{}", formatter.pretty());
}
