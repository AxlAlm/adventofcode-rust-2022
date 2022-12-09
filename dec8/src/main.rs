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
    let mut nr_visible = 0;

    nr_visible += (grid.len() * 2) + ((grid.len() - 2) * 2);

    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            // figure out differnet trees in the each line
            let left = &grid[i][0..j];
            let right = &grid[i][j + 1..];
            let up_down: &Vec<i32> = &grid.iter().map(|x| x[j]).collect();
            let up = &up_down[0..i];
            let down = &up_down[i + 1..];

            let lines_of_sight = vec![&left, &right, &up, &down];

            // loop over lines and find lines where the tallest tree
            // in a line is smaller than the target tree
            for line in lines_of_sight {
                if line.iter().max().unwrap() < &grid[i][j] {
                    // println!("VISIBLE");
                    nr_visible += 1;
                    break;
                }
            }
        }
    }
    println!("nr_visible={}", nr_visible);
}

fn main() {
    let grid = parse_input().unwrap();
    find_visible(grid)
}
