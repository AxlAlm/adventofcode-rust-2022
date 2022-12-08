use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

fn get_common_item(items_lists: &Vec<String>) -> String {
    // convert list of strings to sets of chars
    let mut sets = items_lists
        .iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    // get the first set and
    let (first_set, other_sets) = sets.split_at_mut(1);
    let first_set = &mut first_set[0];

    // for each other set we check if all items int
    for other in other_sets {
        first_set.retain(|e| other.contains(e));
    }

    return first_set.iter().next().unwrap().to_string();
}

fn solution(file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let n_lines = lines.len();
    let n = 3; // NOTE! set to 3 for part 2, 1 for part 1
    let mut priority_sum = 0;
    let mut i = 0;
    while i < n_lines {
        let items_lists = &lines[i..i + n].to_vec();
        println!("{:?}", items_lists);

        // NOTE! Uncomment for part 2
        // let (x, y) = items_lists[0].split_at(items_lists[0].len() / 2);
        // let items_lists: Vec<String> = vec![x.to_string(), y.to_string()];

        let common_item = get_common_item(&items_lists);

        let priority = a.find(&String::from(common_item.to_string())).unwrap() + 1;
        priority_sum += priority;

        i += n
    }

    Ok(priority_sum)
}

fn main() {
    let output = solution("input.txt");
    println!("Answer: {:?}", output.unwrap());
}
