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

fn hand_picker(
    opponent_hand: &str,
    needed_outcome: &str,
    win_cheatsheet: &HashMap<&str, &str>,
    lose_cheatsheet: &HashMap<&str, &str>,
) -> String {
    let my_hand = match needed_outcome {
        "draw" => opponent_hand.to_string(),
        "lose" => lose_cheatsheet.get(opponent_hand).unwrap().to_string(),
        _ => win_cheatsheet.get(opponent_hand).unwrap().to_string(),
    };

    my_hand
}

fn parse_input_file(
    input_file: &str,
    opponent_hand_encodings: &HashMap<&str, &str>,
    needed_outcome_encodings: &HashMap<&str, &str>,
    win_conditions: &HashSet<(&str, &str)>,
    hand_points: &HashMap<&str, usize>,
    win_cheatsheet: &HashMap<&str, &str>,
    lose_cheatsheet: &HashMap<&str, &str>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let vec2: Vec<&str> = line.split_whitespace().collect();

        let opponent_hand = opponent_hand_encodings.get(&vec2[0]).unwrap();
        let needed_outcome = needed_outcome_encodings.get(&vec2[1]).unwrap();

        let my_hand = hand_picker(
            &opponent_hand,
            &needed_outcome,
            &win_cheatsheet,
            &lose_cheatsheet,
        );

        let match_score = calc_match_score(&opponent_hand, &my_hand, &win_conditions, &hand_points);
        total_score += match_score;
    }
    Ok(total_score)
}

pub fn solution2() -> usize {
    // we can deal with tie above
    let win_conditions = HashSet::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    let lose_cheatsheet = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    let win_cheatsheet = HashMap::from([
        ("scissors", "rock"),
        ("rock", "paper"),
        ("paper", "scissors"),
    ]);

    let hand_points = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);

    let needed_outcome_encodings = HashMap::from([("X", "lose"), ("Y", "draw"), ("Z", "win")]);
    let opponent_hand_encodings = HashMap::from([("A", "rock"), ("B", "paper"), ("C", "scissors")]);
    let output = parse_input_file(
        "input.txt",
        &opponent_hand_encodings,
        &needed_outcome_encodings,
        &win_conditions,
        &hand_points,
        &win_cheatsheet,
        &lose_cheatsheet,
    );

    output.unwrap()
}
