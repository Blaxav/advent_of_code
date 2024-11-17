use std::collections::HashSet;

use crate::utils::read_file;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            part_1(&lines);
            part_2(&lines);
        }
        Err(e) => println!("{}", e),
    }
}

fn part_1(data: &Vec<String>) -> () {
    let mut sum: i64 = 0;

    for elt in data {
        sum += elt.parse::<i64>().unwrap();
    }

    println!("Part 1 : {}", sum);
}

fn part_2(data: &Vec<String>) -> () {
    let mut sum: i64 = 0;
    let mut seen: HashSet<i64> = HashSet::new();

    seen.insert(sum);
    loop {
        for elt in data {
            sum += elt.parse::<i64>().unwrap();

            if seen.contains(&sum) {
                println!("Part 2 : {}", sum);
                return;
            }
            seen.insert(sum);
        }
    }
}
