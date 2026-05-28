use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let algorithm = File::open("test.txt").unwrap();
    let algorithm_reader = BufReader::new(algorithm);

    for (index, line) in algorithm_reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}|{}", index + 1, line);
    }


    let solution = File::open("test.txt").unwrap();
    let solution_reader = BufReader::new(solution);
    println!("\n[ SOLUTION ]");
    for line in solution_reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
