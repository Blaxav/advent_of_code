use crate::utils::read_file;
use std::collections::HashMap;

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
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    data.into_iter().for_each(|s| {
        let splitted = s.split("   ").collect::<Vec<&str>>();
        left.push(splitted[0].parse().unwrap());
        right.push(splitted[1].parse().unwrap());
    });

    left.sort();
    right.sort();

    let res: u64 = left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + (r as i64 - l as i64).abs() as u64);

    println!("Part 1: {}", res);
}

fn part_2(data: &Vec<String>) -> () {
    let mut left: Vec<u64> = Vec::new();
    let mut right_count: HashMap<u64, u64> = HashMap::new();

    data.into_iter().for_each(|s| {
        let splitted = s.split("   ").collect::<Vec<&str>>();
        let left_v: u64 = splitted[0].parse().unwrap();
        let right_v: u64 = splitted[1].parse().unwrap();

        left.push(left_v);

        // Maintain hashset with a count of all encountered values
        let _ = *right_count.entry(left_v).or_insert(0);
        *right_count.entry(right_v).or_insert(0) += 1;
    });

    let res: u64 = left
        .into_iter()
        .fold(0, |acc, l| acc + (l * right_count[&l]));

    println!("Part 2: {}", res);
}
