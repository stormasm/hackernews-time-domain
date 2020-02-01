#[macro_use]
extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate ansi_term;
extern crate bincode;
extern crate byteorder;
extern crate chan;
extern crate env_logger;
//extern crate futures;

// extern crate mount;
//extern crate persistent;
//extern crate staticfile;
extern crate tantivy;
extern crate time;
//extern crate urlencoded;

/*
#[macro_use]
extern crate serde_derive;
*/

use std::io::Write;

use clap::{App, AppSettings, Arg, SubCommand};
mod commands;
pub mod timer;
use self::commands::*;

fn main() {
    env_logger::init().unwrap();

    let index_arg = Arg::with_name("index")
        .short("i")
        .long("index")
        .value_name("directory")
        .help("Tantivy index directory filepath")
        .required(true);

    let cli_options = App::new("Tantivy")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul Masurel <paul.masurel@gmail.com>")
        .about("Tantivy Search Engine's command line interface.")
        .subcommand(
            SubCommand::with_name("index")
                .about("Index files")
                .arg(index_arg.clone())
                .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("file")
                    .help("File containing the documents to index."))
                .arg(Arg::with_name("num_threads")
                    .short("t")
                    .long("num_threads")
                    .value_name("num_threads")
                    .help("Number of indexing threads. By default num cores - 1 will be used")
                    .default_value("3"))
                .arg(Arg::with_name("memory_size")
                    .short("m")
                    .long("memory_size")
                    .value_name("memory_size")
                    .help("Total memory_size in bytes. It will be split for the different threads.")
                    .default_value("1000000000"))
                .arg(Arg::with_name("nomerge")
                    .long("nomerge")
                    .help("Do not merge segments"))
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search an index.")
                .arg(index_arg.clone())
                .arg(Arg::with_name("query")
                    .short("q")
                    .long("query")
                    .value_name("query")
                    .help("Query")
                    .required(true))
        )
        .get_matches();

    let (subcommand, some_options) = cli_options.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "index" => run_index_cli,
        "search" => run_search_cli,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    if let Err(ref e) = run_cli(options) {
        let stderr = &mut std::io::stderr();
        let errmsg = "Error writing ot stderr";
        writeln!(stderr, "{}", e).expect(errmsg);
        std::process::exit(1);
    }
}
