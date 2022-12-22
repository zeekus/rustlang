use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("myfile").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("zeek") {
            println!("Match found: {}", line);
        }
    }
}

