use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::read_to_string;

#[derive(Debug, Clone, Default)]
struct Monkey {
    items: Vec<i32>,
    operation: String,
    operation_v: Vec<i32>,
    divisible_by: i32,
    throw_to: HashMap<bool, i32>,
    n_inspects: i32,
}

impl Monkey {
    fn test(&mut self) -> (i32, i32) {
        let mut item_worry_level = self.items.pop().unwrap();

        let mut worry_modifier_value = 0;
        if self.operation_v.is_empty() {
            worry_modifier_value = item_worry_level;
        } else {
            worry_modifier_value = self.operation_v[0]
        }

        match self.operation.as_str() {
            "+" => item_worry_level += worry_modifier_value,
            "*" => item_worry_level *= worry_modifier_value,
            _ => panic!("oh no!"),
        }

        item_worry_level /= 3;

        let pass_test = item_worry_level % self.divisible_by == 0;
        let throw_to_id = self.throw_to.get(&pass_test).unwrap().clone();

        self.n_inspects += 1;
        (throw_to_id, item_worry_level)
    }

    fn add_item(&mut self, item: i32) {
        self.items.insert(0, item);
    }
}

fn extract_numbers(input: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]+").unwrap();
    }
    RE.find_iter(input)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn extract_operation(input: &str) -> String {
    println!("{}", input);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\*|-|\+|\\").unwrap();
    }
    RE.find_iter(input)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()[0]
        .to_string()
}

fn parse_input() -> Result<HashMap<i32, Monkey>, Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let input = read_to_string(file).unwrap();
    let x = input.split("\n\n");

    let mut monkeys: HashMap<i32, Monkey> = HashMap::new();

    for (i, m) in x.enumerate() {
        let lines: Vec<&str> = m.split("\n").collect();

        let mut throw_map: HashMap<bool, i32> = HashMap::new();
        throw_map.insert(true, extract_numbers(lines[4])[0]);
        throw_map.insert(false, extract_numbers(lines[5])[0]);

        let mut items = extract_numbers(lines[1]);
        items.reverse();

        let monkey = Monkey {
            items: items,
            operation: extract_operation(lines[2]),
            operation_v: extract_numbers(lines[2]),
            divisible_by: extract_numbers(lines[3])[0],
            throw_to: throw_map,
            ..Default::default()
        };

        println!("{:?}", monkey);

        monkeys.insert(i as i32, monkey.clone());
    }
    Ok(monkeys)
}

fn monkey_around(monkeys: &mut HashMap<i32, Monkey>, n_round: i32) {
    // let mut worry_level = 0;

    for _ in 0..n_round {
        for i in 0..monkeys.len() {
            let nr_items = monkeys.get(&(i as i32)).unwrap().items.len();

            for _ in 0..nr_items {
                let monkey = monkeys.get_mut(&(i as i32)).unwrap();
                let (throw_to_id, item_worry_level) = monkey.test();

                // println!("{} - {}", throw_to_id, item_worry_level);

                monkeys
                    .get_mut(&throw_to_id)
                    .unwrap()
                    .add_item(item_worry_level)

                // let mut worry_level = monkey.items.pop().unwrap();

                // let mut v = 0;
                // if monkey.operation_v.is_empty() {
                //     v = worry_level;
                // } else {
                //     v = monkey.operation_v[0]
                // }

                // match monkey.operation.as_str() {
                //     "+" => worry_level += v,
                //     "*" => worry_level *= v,
                //     _ => panic!("oh no!"),
                // }

                // worry_level /= 3;
                // let pass_test = worry_level % monkey.divisible_by == 0;

                // let throw_to_id = monkey.throw_to.get(&pass_test).unwrap().clone();

                // monkeys
                //     .get_mut(&throw_to_id)
                //     .unwrap()
                //     .items
                //     .insert(0, item_worry_level);
            }
        }
    }
}

fn main() {
    let mut monkeys = parse_input().unwrap();
    // println!("{:?}", monkeys);
    monkey_around(&mut monkeys, 20);

    let mut all_n_inspects: Vec<i32> = vec![];
    for (i, m) in monkeys.iter() {
        all_n_inspects.push(m.n_inspects.clone());
        // println!("{} - {:?}", i, m.n_inspects.clone());
    }
    all_n_inspects.sort();
    all_n_inspects.reverse();

    println!("{:?}", all_n_inspects[0] * all_n_inspects[1]);

    // let x = monkeys.get_mut(&1).unwrap();
    // x.items.pop();
    // println!("{:?}", monkeys.get_mut(&1));
}
