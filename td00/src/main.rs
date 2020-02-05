extern crate ansi_term;
extern crate bincode;
extern crate byteorder;
extern crate chan;
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate serde_json;
extern crate tantivy;
extern crate time;

use std::io::Write;

use clap::{App, AppSettings, Arg, SubCommand};
mod commands;
pub mod timer;
use self::commands::*;

pub const TANINDEX: &str = "/tmp/tantivy/idxhn";

fn main() {
    env_logger::init().unwrap();

    let index_arg = Arg::with_name("index")
        .short("i")
        .long("index")
        .value_name("directory")
        .help("Tantivy index directory filepath")
        .required(true);

    let cli_options = App::new("Tantivy Hackernews")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael Angerman <stormasm@gmail.com>")
        .about("Tantivy Search for Hackernews.")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new index directory for the hacker news schema"),
        )
        .subcommand(
            SubCommand::with_name("index")
                .about("Index files")
                //.arg(index_arg.clone())
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("file")
                        .help("File containing the documents to index."),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search an index.")
                .arg(index_arg.clone())
                .arg(
                    Arg::with_name("query")
                        .short("q")
                        .long("query")
                        .value_name("query")
                        .help("Query")
                        .required(true),
                ),
        )
        /*
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Create a new index. The schema will be populated with a simple example schema")
                        //.arg(index_arg.clone())
                )
        */
        .get_matches();

    let (subcommand, some_options) = cli_options.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "create" => run_create_cli,
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
