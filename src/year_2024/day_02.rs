use crate::utils::read_file;
use rayon::prelude::*;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let data: Vec<Vec<i64>> = lines
                .into_iter()
                .map(|s| {
                    s.split(" ")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect();
            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

fn check_report(data: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = data.windows(2).map(|pair| pair[0] - pair[1]).collect();

    let mut decrease = false;
    let mut increase = false;
    let mut valid_threshold = true;

    diffs.into_iter().for_each(|diff| {
        if diff < 0 {
            decrease = true;
        } else if diff > 0 {
            increase = true;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            valid_threshold = false;
        }
    });

    // Both cannot be True at the same time, otherwise threshold is not valid
    // and the result is False anyway
    (!decrease || !increase) && valid_threshold
}

fn check_p2(data: &Vec<i64>) -> bool {
    if check_report(data) {
        return true;
    }

    let partials: Vec<Vec<i64>> = (0..data.len())
        .map(|i| {
            let mut clone = data.clone();
            clone.remove(i);
            clone
        })
        .collect();

    let valids = partials.par_iter().map(check_report).filter(|b| *b).count();
    return valids >= 1;
}

fn part_1(data: &Vec<Vec<i64>>) -> () {
    let valids = data
        .par_iter()
        .map(|d| check_report(d))
        .filter(|b| *b)
        .count();

    println!("Part 1: {}", valids);
}

fn part_2(data: &Vec<Vec<i64>>) -> () {
    let valids = data.par_iter().map(|d| check_p2(d)).filter(|b| *b).count();

    println!("Part 2: {}", valids);
}
