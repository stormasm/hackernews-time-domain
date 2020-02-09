// https://docs.rs/url/2.1.1/url/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::string::String;

use clap::ArgMatches;

use crossbeam::crossbeam_channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};

use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: u64,
    time: u64,
    url: String,
}

fn process_url(url: &str) {
    let urlp = Url::parse(url).unwrap();
    //println!("{:?}", urlp.host_str());

    match urlp.host_str().unwrap() {
        "github.com" => {
            println!("{:?}", urlp);
        }
        _ => {
            ();
        }
    }
}

fn process_lines(r: Receiver<String>) {
    let item_json = r.recv().unwrap();

    let item: Item = serde_json::from_str(&item_json).unwrap();
    let _id = &item.id;
    let _time = &item.time;
    let url = &item.url;

    process_url(url);
    // println!("{} {} {}", id, time, url);
}

fn read_file_to_buffer(filename: String) -> tantivy::Result<()> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    for (_num, line) in file.lines().enumerate() {
        // Create a channel of unbounded capacity.
        let (s, r) = unbounded();

        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();

        process_lines(r);
    }

    Ok(())
}

pub fn run_domain_cli(argmatch: &ArgMatches) -> Result<(), String> {
    let pb = PathBuf::from(argmatch.value_of("file").unwrap());
    let filename = pb.to_str().unwrap().to_string();

    show_domain(filename).map_err(|e| format!("Indexing file failed : {:?}", e))
}

fn show_domain(filename: String) -> tantivy::Result<()> {
    let _contents = read_file_to_buffer(filename.to_string());
    Ok(())
}
