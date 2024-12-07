use crate::utils::read_file;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let mut rules: HashSet<(u32, u32)> = HashSet::new();
            let mut data: Vec<Vec<u32>> = Vec::new();

            let mut is_rule_part = true;
            for l in lines {
                if l == "" {
                    is_rule_part = false;
                } else if is_rule_part {
                    let rule: Vec<u32> = l.split("|").map(|n| n.parse().unwrap()).collect();
                    rules.insert((rule[1], rule[0]));
                } else {
                    let c_data: Vec<u32> = l.split(",").map(|n| n.parse().unwrap()).collect();
                    data.push(c_data);
                }
            }

            part_1(&rules, &data);
            part_2(&rules, &data);
        }
        Err(e) => println!("{}", e),
    }
}

/// Evaluates a rule, returns None if not valid, else the middle number
fn evaluate_rule(rules: &HashSet<(u32, u32)>, data: &Vec<u32>) -> Option<u32> {
    for i in 0..(data.len() - 1) {
        for j in (i + 1)..data.len() {
            // rules contain exceptions
            if rules.contains(&(data[i], data[j])) {
                return None;
            }
        }
    }

    return Some(data[data.len() / 2]);
}

fn part_1(rules: &HashSet<(u32, u32)>, data: &Vec<Vec<u32>>) -> () {
    let res = data
        .into_par_iter()
        .filter_map(|d| evaluate_rule(rules, d))
        .collect::<Vec<u32>>()
        .into_iter()
        .fold(0, |acc, val| acc + val);

    println!("Part 1: {}", res);
}

fn sort(rules: &HashSet<(u32, u32)>, data: &Vec<u32>) -> u32 {
    let mut res: Vec<u32> = data.clone();

    for i in 0..(data.len() - 1) {
        let mut j = i + 1;
        while j <= data.len() - 1 {
            if rules.contains(&(res[i], res[j])) {
                // Inverting them
                let tmp = res[i];
                res[i] = res[j];
                res[j] = tmp;

                // Restart check after swap for the sub-vec
                j = i + 1;
            } else {
                // Increase j to next
                j += 1;
            }
        }
    }

    return res[res.len() / 2];
}

fn part_2(rules: &HashSet<(u32, u32)>, data: &Vec<Vec<u32>>) -> () {
    let res = data
        .into_iter()
        .filter_map(|d| match evaluate_rule(rules, d) {
            None => Some(sort(rules, d)),
            Some(_) => None,
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .fold(0, |acc, val| acc + val);

    println!("Part 2: {}", res);
}
