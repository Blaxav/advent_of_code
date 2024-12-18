use crate::utils::read_file;
use regex::{Captures, Regex};

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let mut data: Vec<Machine> = Vec::new();

            let re_a = Regex::new(r"Button A: X\+(\d{1,5}), Y\+(\d{1,5})").unwrap();
            let re_b = Regex::new(r"Button B: X\+(\d{1,5}), Y\+(\d{1,5})").unwrap();
            let re_prize = Regex::new(r"Prize: X=(\d{1,5}), Y=(\d{1,5})").unwrap();

            let mut iter = lines.into_iter();
            while let Some(line) = iter.next() {
                let button_a = parse_from_match(&re_a.captures(&line).unwrap());
                let button_b = parse_from_match(&re_b.captures(&iter.next().unwrap()).unwrap());
                let prize = parse_from_match(&re_prize.captures(&iter.next().unwrap()).unwrap());

                data.push(Machine {
                    a: button_a,
                    b: button_b,
                    prize: prize,
                });

                // Check if no new line, then stop parsing
                if iter.next().is_none() {
                    break;
                }
            }

            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

fn parse_from_match(capture: &Captures) -> (u64, u64) {
    (capture[1].parse().unwrap(), capture[2].parse().unwrap())
}

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

const A_COST: u64 = 3;
const B_COST: u64 = 1;

// Reverts the equation
fn solution(m: &Machine) -> (f64, f64) {
    let n_b: f64 = ((m.a.1 * m.prize.0) as f64 - (m.a.0 * m.prize.1) as f64)
        / ((m.a.1 * m.b.0) as f64 - (m.a.0 * m.b.1) as f64);
    let n_a: f64 = (m.prize.0 as f64 - n_b * m.b.0 as f64) / m.a.0 as f64;
    return (n_a, n_b);
}

fn part_1(data: &Vec<Machine>) -> () {
    let mut res: u64 = 0;

    for m in data {
        let (n_a, n_b) = solution(m);
        if n_a == (n_a as u64) as f64 && n_b == (n_b as u64) as f64 {
            res += A_COST * n_a as u64 + B_COST * n_b as u64;
        }
    }
    println!("Part 1 : {}", res);
}

fn part_2(data: &Vec<Machine>) -> () {
    let add: u64 = 10000000000000;
    let mut res: u64 = 0;

    for m in data {
        let new_m = Machine {
            a: m.a,
            b: m.b,
            prize: (m.prize.0 + add, m.prize.1 + add),
        };

        let (n_a, n_b) = solution(&new_m);
        if n_a == (n_a as u64) as f64 && n_b == (n_b as u64) as f64 {
            res += A_COST * n_a as u64 + B_COST * n_b as u64;
        }
    }
    println!("Part 2 : {}", res);
}
