extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"\bzeek\b").unwrap();
    let text = "The quick brown fox jumped over the lazy zeek.";

    for mat in re.find_iter(text) {
        println!("Match found at {}", mat.start());
    }
}

