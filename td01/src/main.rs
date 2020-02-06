use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;
mod commands;
use self::commands::*;

pub const TANINDEX: &str = "/tmp/tantivy/idxhn";

fn main() {
    let index_arg = Arg::with_name("index")
        .short("i")
        .long("index")
        .value_name("directory")
        .help("Tantivy index directory filepath")
        .required(false);

    let cli_options = App::new("Tantivy Hackernews")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael Angerman <stormasm@gmail.com>")
        .about("Tantivy Search for Hackernews.")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new index directory for the hacker news schema")
                .arg(index_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("index")
                .about("Index files")
                .arg(index_arg.clone())
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
