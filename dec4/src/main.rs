use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// fn get_overlap(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
//     let start = cmp::max(a[0], b[0]);
//     let end = cmp::min(a[a.len() - 1], b[b.len() - 1]) + 1;

//     let overlap: Vec<u32> = (start..end).collect();

//     overlap
// }

// fn is_overlap(range_a: &Vec<i32>, range_b: &Vec<i32>) -> bool {
//     (range_a[0] <= range_b[0]) & (range_a[1] >= range_b[1])
// }

fn parse_input() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let line = &line.unwrap();
        let x: Vec<&str> = line.split(",").collect();

        let range_a: Vec<i32> = x[0]
            .split("-")
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();

        let range_b: Vec<i32> = x[1]
            .split("-")
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();

        let x: HashSet<i32> = HashSet::from_iter(range_a[0]..*range_a.last().unwrap() + 1);
        let y: HashSet<i32> = HashSet::from_iter(range_b[0]..*range_b.last().unwrap() + 1);

        // PART 2
        if x.intersection(&y).count() == 0 {
            continue;
        }

        // // part 1
        // if !(x.is_superset(&y) | y.is_superset(&x)) {
        //     continue;
        // }

        count += 1
    }

    Ok(count)
}

fn main() {
    println!("{:?}", parse_input());
}
