use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::string::String;

use clap::ArgMatches;

use crossbeam::crossbeam_channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: u64,
    time: u64,
}

fn process_lines(r: Receiver<String>) {
    let item_json = r.recv().unwrap();

    let item: Item = serde_json::from_str(&item_json).unwrap();
    let id = &item.id;
    let time = &item.time;

    println!("{} {}", id, time);
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

pub fn run_time_cli(argmatch: &ArgMatches) -> Result<(), String> {
    let pb = PathBuf::from(argmatch.value_of("file").unwrap());
    let filename = pb.to_str().unwrap().to_string();

    show_time(filename).map_err(|e| format!("Indexing file failed : {:?}", e))
}

fn show_time(filename: String) -> tantivy::Result<()> {
    let _contents = read_file_to_buffer(filename.to_string());
    Ok(())
}
