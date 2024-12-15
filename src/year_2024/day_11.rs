use crate::utils::read_file;
use std::collections::HashMap;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            if lines.len() != 1 {
                panic!("Only one line expected");
            }

            let data: Vec<u128> = lines[0]
                .split(" ")
                .map(|c| c.parse::<u128>().unwrap())
                .collect();

            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

fn next(val: u128) -> Vec<u128> {
    if val == 0 {
        return vec![1];
    }

    let val_str = val.to_string();
    if val_str.len() % 2 == 0 {
        let mid_size = val_str.len() / 2;

        return vec![
            val_str[..mid_size].parse::<u128>().unwrap(),
            val_str[mid_size..].parse::<u128>().unwrap(),
        ];
    }

    vec![val * 2024]
}

fn apply_n(val: u128, n: u64, cache: &mut HashMap<(u128, u64), u128>) -> u128 {
    if cache.contains_key(&(val, n)) {
        return cache[&(val, n)];
    }

    // Return itself it no more iteration to perform
    if n == 0 {
        return 1;
    }

    // Apply n-1 iteration on the next
    let res: u128 = next(val)
        .iter()
        .map(|v| apply_n(*v, n - 1, cache))
        .fold(0, |acc, val| acc + val);

    // insert in cache and return
    cache.insert((val, n), res);

    res
}

fn part_1(data: &Vec<u128>) -> () {
    let mut cache: HashMap<(u128, u64), u128> = HashMap::new();

    let n_iter: u64 = 25;

    let res: u128 = data
        .iter()
        .fold(0, |acc, val| acc + apply_n(*val, n_iter, &mut cache));

    println!("Part 1 : {}", res);
}

fn part_2(data: &Vec<u128>) -> () {
    let mut cache: HashMap<(u128, u64), u128> = HashMap::new();

    let n_iter: u64 = 75;

    let res: u128 = data
        .iter()
        .fold(0, |acc, val| acc + apply_n(*val, n_iter, &mut cache));

    println!("Part 2 : {}", res);
}
