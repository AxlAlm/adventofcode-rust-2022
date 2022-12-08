use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn extract_file_size(input: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]+").unwrap();
    }
    let n: i32 = RE
        .find_iter(input)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();

    n
}
#[derive(Default, Debug, Clone)]
struct Dir {
    name: String,
    dirs: HashMap<String, Dir>,
    size: i32,
}

fn extract_dir_name(cmd: &str) -> String {
    let command_parts: Vec<&str> = cmd.split(" ").collect();
    let dir = command_parts.last().unwrap().to_owned();
    dir.to_string()
}

fn propogate_size(dir: &mut Dir) -> i32 {
    let mut sub_dir_sizes: i32 = 0;
    for (_, v) in dir.dirs.iter_mut() {
        sub_dir_sizes += propogate_size(v)
    }
    dir.size = dir.size + sub_dir_sizes;
    dir.size
}

fn find_sum(dir: &Dir, max_size: i32) -> i32 {
    let mut sum: i32 = 0;
    for (_, v) in dir.dirs.iter() {
        if v.size <= max_size {
            sum += v.size
        }

        sum += find_sum(v, max_size)
    }
    return sum;
}

fn find_dir_to_rm(dir: &Dir, min_size: i32) -> Vec<i32> {
    let mut options: Vec<i32> = vec![];
    for (_, v) in dir.dirs.iter() {
        if v.size >= min_size {
            options.push(v.size)
        }
        options.extend(find_dir_to_rm(v, min_size))
    }
    return options;
}

fn build_dir_tree(dir: &mut Dir, actions: &mut Vec<&str>) {
    if actions.is_empty() {
        return;
    }

    let action = actions.pop().unwrap();
    let v: Vec<&str> = action.split("\n").filter(|x| !x.is_empty()).collect();
    let cmd = v[0];

    if cmd.contains("cd") {
        if cmd.contains("..") {
            return;
        } else {
            let dir_name = extract_dir_name(cmd);
            let sub_dir = Dir {
                name: dir_name.clone(),
                dirs: HashMap::new(),
                size: 0,
            };
            dir.dirs.insert(dir_name.clone(), sub_dir);
            build_dir_tree(dir.dirs.get_mut(&dir_name.clone()).unwrap(), actions)
        }
    } else {
        let files_sizes: Vec<i32> = v
            .into_iter()
            .filter(|v| !v.contains("dir") & !v.contains(" ls"))
            .map(|v| extract_file_size(v))
            .collect();
        let sum_files: i32 = files_sizes.iter().sum();
        dir.size = sum_files;
    }

    build_dir_tree(dir, actions);
}

fn parse(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let d = read_to_string(file_path).unwrap();
    let actions: Vec<&str> = d.split("$").collect();
    let mut actions = actions[1..].to_vec().into_iter().rev().collect();

    let mut dir_tree = Dir {
        name: "root".to_string(),
        dirs: HashMap::new(),
        size: 0,
    };

    build_dir_tree(&mut dir_tree, &mut actions);

    let total_used_size = propogate_size(&mut dir_tree);

    // PART 1
    println!(
        "sum of all folder under 100000 = {:?}",
        find_sum(&dir_tree, 100000)
    );

    // PART 2
    let total_size = 70000000;
    let needed_free_space = 30000000;
    let total_free_space = total_size - total_used_size;
    let size_to_delete = needed_free_space - total_free_space;

    println!("total_used_size = {:?}", total_used_size);
    println!("total_free_space = {:?}", total_free_space);
    println!("size_to_delete = {:?}", size_to_delete);

    let mut rm_options = find_dir_to_rm(&dir_tree, size_to_delete);
    rm_options.sort();
    println!("size of best folder to rm = {:?}", rm_options[0]);

    Ok(())
}

fn main() {
    _ = parse("input.txt");
}
