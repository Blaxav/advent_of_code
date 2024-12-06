use crate::utils::read_file;
use regex::Regex;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let concat = lines.concat();
            part_1(&concat);
            part_2(&concat);
        }
        Err(e) => println!("{}", e),
    }
}

fn compute_mul(subline: &String) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(subline)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .fold(0, |acc, (a, b)| acc + a * b)
}

fn part_1(data: &String) -> () {
    println!("Part 1: {}", compute_mul(data));
}

fn part_2(concat: &String) -> () {
    let active_part_re = Regex::new(r"^(.*?)don't\(\)|do\(\)(.*?)don't\(\)|do\(\)(.*?)$").unwrap();

    let res = active_part_re
        .captures_iter(concat.as_str())
        .map(|c| {
            let (_, [founds]) = c.extract();
            compute_mul(&String::from(founds))
        })
        .fold(0, |acc, val| acc + val);

    println!("Part 2: {}", res);
}
