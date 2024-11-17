use crate::utils::read_file;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            part_1(&lines);
            part_2(&lines);
        }
        Err(e) => println!("Error {}", e),
    }
}

fn part_1(data: &Vec<String>) -> () {
    let mut n_two: u64 = 0;
    let mut n_three: u64 = 0;

    for s in data.iter() {
        let counter = s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        match counter.into_iter().fold(
            (false, false),
            |(mut two, mut three): (bool, bool), (_, v)| {
                match v {
                    2 => two = true,
                    3 => three = true,
                    _ => (),
                };
                (two, three)
            },
        ) {
            (true, true) => {
                n_two += 1;
                n_three += 1;
            }
            (true, false) => {
                n_two += 1;
            }
            (false, true) => {
                n_three += 1;
            }
            (false, false) => (),
        }
    }

    println!("Part 1 : {}", n_two * n_three);
}

fn has_exactly_one_diff(s1: &str, s2: &str) -> bool {
    let mut diff = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }
    diff == 1
}

fn part_2(data: &Vec<String>) -> () {
    let n = data.len();

    let all_pair_ids: Vec<(usize, usize)> = (0..n - 1)
        .flat_map(|i| ((i + 1)..=n - 1).map(move |j| (i, j)))
        .collect();

    let results: Vec<(usize, usize)> = all_pair_ids
        .par_iter()
        .filter_map(|(first, second)| {
            let s_first = data[first.clone()].clone();
            let s_second = data[second.clone()].clone();
            if has_exactly_one_diff(s_first.as_str(), s_second.as_str()) {
                Some((first.clone(), second.clone()))
            } else {
                None
            }
        })
        .collect();

    // Printing the result
    print!("Part 2 : ");
    let first: &String = &data[results[0].0];
    let second: &String = &data[results[0].1];
    for (x, y) in first.chars().zip(second.chars()) {
        if &x == &y {
            print!("{}", &x);
        }
    }
    println!("");
}
