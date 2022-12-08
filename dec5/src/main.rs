use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn update_stack(stacks: &mut HashMap<String, Vec<String>>, from: String, to: String, n: usize) {
    let to_stack_init_len = stacks.get_mut(&to).unwrap().len();
    for _ in 0..n {
        let value_to_move = stacks.get_mut(&from).unwrap().pop().unwrap();
        // PART 1 we just push to top of vec
        //stacks.get_mut(&to).unwrap().push(value_to_move);

        // PART 2 we insert at the inital length of the stack we are moving to
        stacks
            .get_mut(&to)
            .unwrap()
            .insert(to_stack_init_len, value_to_move);
    }
}

fn parse_initial_stacks(stacks: &mut HashMap<String, Vec<String>>, line: String) {
    for (i, x) in (0..line.len()).step_by(4).enumerate() {
        let item = line.chars().nth(x + 1).unwrap().to_string();
        let key = (i + 1).to_string();

        if (item.is_empty()) | (item == " ") {
            continue;
        }

        if !stacks.contains_key(&key) {
            stacks.insert(key.clone(), vec![]);
        }

        stacks.get_mut(&key).unwrap().insert(0, item);
    }
}

fn extract_numbers(input: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]+").unwrap();
    }
    RE.find_iter(input)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
}

fn get_inital_stacks(
    stacks: &mut HashMap<String, Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        // this is to just get string based on whitespace splits
        // it helps to indentify which type of line it is
        let line_x = line.split_whitespace().collect::<Vec<&str>>();

        // if line is empty then we skip
        if line_x.is_empty() {
            continue;
        }

        // if it starts with 1 we skip
        if line_x[0] == "1" {
            continue;
        }

        // if it contains [ we know its a line we use to build initial stacks
        // else we know the line is a move line
        if line.contains("[") {
            parse_initial_stacks(stacks, line);
        } else {
            let moves = extract_numbers(&line);
            let n: usize = moves[0].parse().unwrap();
            update_stack(stacks, moves[1].to_string(), moves[2].to_string(), n);
        }
    }
    Ok(())
}

fn main() {
    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();

    _ = get_inital_stacks(&mut stacks);

    let mut top_stacks: Vec<&str> = vec![];
    for i in 1..stacks.len() + 1 {
        top_stacks.push(stacks.get(&i.to_string()[..]).unwrap().last().unwrap());
    }

    println!("{:?}", top_stacks.join(""));
}
