extern crate env_logger;
extern crate structopt;
extern crate chrootable_https;

use chrootable_https::{Resolver, Client};
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(short="-t", long="--timeout")]
    timeout: Option<u64>,
    urls: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::from_args();

    let resolver = Resolver::cloudflare();

    let mut client = Client::new(resolver);
    if let Some(timeout) = args.timeout {
        client.timeout(Duration::from_millis(timeout));
    }

    for url in &args.urls {
        let reply = client.get(&url).expect("request failed");
        eprintln!("{:#?}", reply);
        io::stdout().write(&reply.body).expect("failed to write body");
    }
}
