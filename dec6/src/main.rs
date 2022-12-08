use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let n = 14;

        for i in 0..line.len() {
            let set: HashSet<char> = line[i..i + n].chars().collect();

            if set.len() == n {
                println!("{}", i + n);
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    _ = parse_input()
}
