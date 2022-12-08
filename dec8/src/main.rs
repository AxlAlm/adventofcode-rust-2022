use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let row_vec: Vec<i32> = line
            .chars()
            .into_iter()
            .map(|v| v.to_string().parse::<i32>().unwrap())
            .collect();

        grid.push(row_vec);
    }
    Ok(grid)
}

fn find_visible(grid: Vec<Vec<i32>>) {
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            println!("---------");
            println!("v={}", grid[i][j]);
            println!("x={:?}", &grid[i][0..i]);
            println!("x={:?}", &grid[i][i + 1..])
            // take the max() for each direction, if its more than v, we its not visible
        }
    }
}

fn main() {
    let grid = parse_input().unwrap();
    find_visible(grid)
}
