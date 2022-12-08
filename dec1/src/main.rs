use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_sums(file_path: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sums = vec![];
    let mut current_sum: usize = 0;
    for line in reader.lines() {
        let line = &line.unwrap();
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            let my_int: usize = line.parse().unwrap();
            current_sum += my_int;
        }
    }

    sums.sort();
    Ok(sums)
}

fn cumulative_sum(sums: &Vec<usize>, n: usize) -> usize {
    let mut cum_sum = 0;
    for (i, &sum) in sums.iter().rev().enumerate() {
        if i == n {
            break;
        }

        cum_sum += sum;
    }

    cum_sum
}

fn main() {
    let file_path = "input.txt";
    let sums = parse_sums(file_path).unwrap();
    let sum = cumulative_sum(&sums, 3);
    println!("{:?}", sums);
    println!("{}", sum);
}
