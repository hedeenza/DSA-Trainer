use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
