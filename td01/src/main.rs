use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;
mod commands;
use self::commands::*;

pub const TANINDEX: &str = "/tmp/tantivy/idxhn";

fn main() {
    let cli_options = App::new("Tantivy Hackernews")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael Angerman <stormasm@gmail.com>")
        .about("Tantivy Search for Hackernews.")
        .subcommand(
            SubCommand::with_name("domain").about("Index files").arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("file")
                    .help("File containing the documents to index."),
            ),
        )
        .subcommand(
            SubCommand::with_name("time").about("Index files").arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("file")
                    .help("File containing the documents to index."),
            ),
        )
        .get_matches();

    let (subcommand, some_options) = cli_options.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "domain" => run_domain_cli,
        "time" => run_time_cli,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    if let Err(ref e) = run_cli(options) {
        let stderr = &mut std::io::stderr();
        let errmsg = "Error writing ot stderr";
        writeln!(stderr, "{}", e).expect(errmsg);
        std::process::exit(1);
    }
}
