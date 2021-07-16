use clap::{crate_authors, crate_version, Clap};
use kvs::common::KvsEngine;
use kvs::kvs_store::KvStore;
use std::{net::SocketAddr, path::Path, process};
#[derive(Clap)]
#[clap(version =crate_version!() , author = crate_authors!())]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap)]
enum SubCommand {
    Get(Key),
    Set(KeyValue),
    RM(Key),
}
#[derive(Clap)]
struct Key {
    key: String,
    #[clap(long, short, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
}
#[derive(Clap)]
struct KeyValue {
    key: String,
    value: String,
    #[clap(long, short, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
}
fn main() {
    let opts = Options::parse();
    let mut kvs = KvStore::open(Path::new("./db")).unwrap();
    match opts.subcmd {
        SubCommand::Get(m) => match kvs.get(m.key) {
            Ok(Some(value)) => {
                println!("{}", value);
            }
            _ => {
                println!("Key not found");
                process::exit(0);
            }
        },
        SubCommand::RM(m) => match kvs.remove(m.key) {
            Ok(_) => {}
            _ => {
                eprintln!("Key not found");
                process::exit(-1);
            }
        },
        SubCommand::Set(m) => match kvs.set(m.key, m.value) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(-1);
            }
        },
    }
}
