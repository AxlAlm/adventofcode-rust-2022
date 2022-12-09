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

    let mut scenic_scores: Vec<usize> = vec![];
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            // figure out differnet trees in the each line
            let target_tree = &grid[i][j];
            // let left = &grid[i][0..j];

            // let left = vec![0; j];
            // left[0..j].clone_from_slice(&grid[i][0..j]);

            let mut left: Vec<i32> = vec![];
            left.extend(grid[i][0..j].iter().rev());

            let right = &grid[i][j + 1..];

            // let right = vec![0; j];
            // left[0..j].clone_from_slice(&grid[i][0..j]);

            let up_down: &Vec<i32> = &grid.iter().map(|x| x[j]).collect();
            // let up = &up_down[0..i];

            let mut up: Vec<i32> = vec![];
            up.extend(up_down[0..i].iter().rev());

            let down = &up_down[i + 1..];

            let lines_trees = vec![&left[..], &right, &up, &down];
            println!("________");
            println!("trees={:?}", lines_trees);
            // loop over lines and find lines where the tallest tree
            // in a line is smaller than the target tree

            // let mut is_visible = false;

            let mut scenic_score = 1;
            for trees in lines_trees {
                // // UNCOMMENT FOR PART 1 Solution 2
                // if is_visible {
                //     break;
                // }

                // PART 1
                // for (k, tree) in trees.iter().enumerate() {
                //     if (tree >= target_tree) | (k == trees.len() - 1) {
                //         scenic_score *= k + 1;
                //         // // UNCOMMENT FOR PART 1 Solution 2
                //         // is_visible = false;
                //         break;
                //     }

                //     // // UNCOMMENT FOR PART 1 Solution 2
                //     // is_visible = true;
                // }

                //PART 1
                if trees.iter().max().unwrap() < &grid[i][j] {
                    // println!("VISIBLE");
                    nr_visible += 1;
                    break;
                }
            }

            // if is_visible {
            //     nr_visible += 1
            // }

            scenic_scores.push(scenic_score);
        }
    }
    println!("max scenic_score={:?}", scenic_scores.iter().max().unwrap());
    println!("nr_visible={}", nr_visible);
}

fn main() {
    let grid = parse_input().unwrap();
    find_visible(grid)
}
