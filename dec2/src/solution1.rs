// use self::Hand::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_match_score(
    opponent_hand: &str,
    my_hand: &str,
    win_conditions: &HashSet<(&str, &str)>,
    hand_points: &HashMap<&str, usize>,
) -> usize {
    let hand_score = hand_points.get(my_hand).unwrap();

    if opponent_hand == my_hand {
        return 3 + hand_score;
    }

    if win_conditions.contains(&(my_hand, opponent_hand)) {
        return 6 + hand_score;
    } else {
        return 0 + hand_score;
    }
}

fn parse_input_file(
    input_file: &str,
    opponent_hand_encodings: &HashMap<&str, &str>,
    my_hand_encodings: &HashMap<&str, &str>,
    win_conditions: &HashSet<(&str, &str)>,
    hand_points: &HashMap<&str, usize>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let vec2: Vec<&str> = line.split_whitespace().collect();
        let opponent_hand = opponent_hand_encodings.get(&vec2[0]).unwrap();
        let my_hand = my_hand_encodings.get(&vec2[1]).unwrap();
        let match_score = calc_match_score(&opponent_hand, &my_hand, &win_conditions, &hand_points);
        total_score += match_score;
    }
    Ok(total_score)
}

pub fn solution1() -> usize {
    // we can deal with tie above
    let win_conditions = HashSet::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    let hand_points = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);

    let my_hand_encodings = HashMap::from([("X", "rock"), ("Y", "paper"), ("Z", "scissors")]);
    let opponent_hand_encodings = HashMap::from([("A", "rock"), ("B", "paper"), ("C", "scissors")]);
    let output = parse_input_file(
        "input.txt",
        &opponent_hand_encodings,
        &my_hand_encodings,
        &win_conditions,
        &hand_points,
    );

    output.unwrap()
}
