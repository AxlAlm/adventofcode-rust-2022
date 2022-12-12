use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut x = 1;
    let mut total_n_cycles = 0;
    let mut nth = 0;
    let mut sum_signal_strength = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // let mut instru = "noop";
        let mut v: i32 = 0;
        let mut n_cycles = 1;
        if line.contains("addx") {
            let (_, v_) = line.split_once(" ").unwrap();
            v = v_.parse::<i32>().unwrap();
            // instru = instru_;
            n_cycles = 2;
        }

        // println!("{} - {} - {}", instru, v, n_cycles);

        for _ in 0..n_cycles {
            total_n_cycles += 1;

            // println!("{} - {}", total_n_cycles, total_n_cycles % 20);
            if total_n_cycles % (20 + (40 * nth)) == 0 {
                println!("--- nth {}", (20 + (40 * nth)));
                // println!("x = {}, total_n_cycles={}", x, total_n_cycles);
                println!("signal_strength = {}", x * total_n_cycles);
                nth += 1;
                sum_signal_strength += x * total_n_cycles;
                println!("sum_signal_strength = {}", sum_signal_strength);
            }
        }

        x += v
    }

    Ok(())
}

fn main() {
    _ = parse_input();
}
