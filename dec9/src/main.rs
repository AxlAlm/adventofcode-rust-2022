// use std::collections::HashSet;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

// #[derive(Debug, Clone)]
// struct Pos {
//     x: i32,
//     y: i32,
// }

// fn abs(a: &i32, b: &i32) -> i32 {
//     if a < b {
//         b - a
//     } else {
//         a - b
//     }
// }

// fn parse_input() -> Result<(), Box<dyn std::error::Error>> {
//     let file = File::open("input.txt")?;
//     let reader = BufReader::new(file);

//     let n_ties = 9;
//     let mut head_pos = Pos { x: 0, y: 0 };
//     let mut tail_pos = Pos { x: 0, y: 0 };
//     let mut x: HashSet<(i32, i32)> = HashSet::new();

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let (direction, steps) = line.split_once(" ").unwrap();
//         let steps: i32 = steps.parse().unwrap();

//         for _ in 0..steps {
//             let old_head_post = head_pos.clone();
//             match direction {
//                 "R" => head_pos.x += 1,
//                 "L" => head_pos.x -= 1,
//                 "U" => head_pos.y += 1,
//                 "D" => head_pos.y -= 1,
//                 _ => println!("Nothing"),
//             };

//             for i in 0..n_ties {
//                 if (abs(&head_pos.x, &tail_pos.x) >= 2) | (abs(&head_pos.y, &tail_pos.y) >= 2) {
//                     tail_pos = old_head_post.clone();
//                     // visited += 1;

//                     if i - n_ties + 1 == 0 {
//                         x.insert((tail_pos.x.clone(), tail_pos.y.clone()));
//                     }
//                 }
//             }

//         }
//     }
//     println!("visited={}", x.len() + 1);

//     Ok(())
// }

// fn main() {
//     _ = parse_input();
// }
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn abs(a: &i32, b: &i32) -> i32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn parse_input() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let n_knots = 10; // change to 2 for PART 1
    let mut knots: Vec<Pos> = (0..n_knots).map(|_| Pos { x: 0, y: 0 }).collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();

        for _ in 0..steps {
            for i in 0..n_knots {
                if i == 0 {
                    match direction {
                        "R" => knots[0].x += 1,
                        "L" => knots[0].x -= 1,
                        "U" => knots[0].y += 1,
                        "D" => knots[0].y -= 1,
                        _ => println!("Nothing"),
                    };
                    continue;
                }

                let is_x_two_away = abs(&knots[i - 1].x, &knots[i].x) >= 2;
                let is_y_two_away = abs(&knots[i - 1].y, &knots[i].y) >= 2;

                // could be made nicer...remove duplicate code etc
                if is_x_two_away & is_y_two_away {
                    knots[i].x = vec![knots[i].x, knots[i - 1].x].iter().max().unwrap() - 1;
                    knots[i].y = vec![knots[i].y, knots[i - 1].y].iter().max().unwrap() - 1;

                    if i == n_knots - 1 {
                        visited.insert((knots[i].x.clone(), knots[i].y.clone()));
                    }
                } else if abs(&knots[i - 1].x, &knots[i].x) >= 2 {
                    knots[i].x = vec![knots[i].x, knots[i - 1].x].iter().max().unwrap() - 1;
                    knots[i].y = knots[i - 1].y;

                    if i == n_knots - 1 {
                        visited.insert((knots[i].x.clone(), knots[i].y.clone()));
                    }
                } else if abs(&knots[i - 1].y, &knots[i].y) >= 2 {
                    knots[i].x = knots[i - 1].x;
                    knots[i].y = vec![knots[i].y, knots[i - 1].y].iter().max().unwrap() - 1;

                    if i == n_knots - 1 {
                        visited.insert((knots[i].x.clone(), knots[i].y.clone()));
                    }
                }
            }
        }
    }
    println!("visited={}", visited.len());

    Ok(())
}

fn main() {
    _ = parse_input();
}
