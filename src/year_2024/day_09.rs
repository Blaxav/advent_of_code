use crate::utils::read_file;
use std::collections::{HashMap, HashSet};

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            if lines.len() != 1 {
                panic!("Only one line expected");
            }

            let data: Vec<u32> = lines[0].chars().map(|c| c.to_digit(10).unwrap()).collect();
            part_1(&data);
            part_2(&&data);
        }
        Err(e) => println!("{}", e),
    }
}

fn part_1(data: &Vec<u32>) -> () {
    println!("{:?}", data);

    // Commpute the total length of the date representation
    let length = data.iter().fold(0, |acc, val| acc + val);

    let mut i: usize = 0;

    let mut j: usize = data.len() - 1;
    let mut j_val: u32 = j as u32 / 2;
    let mut j_k: u32 = data[j];

    let mut res: u128 = 0;
    let mut c_pos = 0;
    let mut j_pos: u32 = length;

    while c_pos < j_pos {
        if i % 2 == 0 {
            // Taking the i's
            res += (0..data[i]).fold(0, |acc, _| {
                c_pos += 1;
                if c_pos <= j_pos {
                    return acc + (c_pos - 1) as u128 * (i / 2) as u128;
                }
                acc as u128
            });
        } else {
            // Taking the j's
            res += (0..data[i]).fold(0, |acc, _| {
                c_pos += 1;
                j_pos -= 1;
                if c_pos < j_pos {
                    if j_k == 0 {
                        j -= 2;
                        j_pos -= data[j + 1];
                        j_val -= 1;
                        j_k = data[j];
                    }
                    j_k -= 1;
                    return acc + (c_pos - 1) as u128 * j_val as u128;
                }
                acc as u128
            })
        }
        i += 1;
    }

    println!("Part 1: {}", res);
}

fn part_2(data: &Vec<u32>) -> () {
    println!("{:?}", data);

    let mut holes: Vec<(u32, u32)> = Vec::new();
    let mut files: Vec<(u32, u32)> = Vec::new();
    let mut c_pos = 0;
    for i in 0..data.len() {
        if i % 2 == 1 {
            holes.push((c_pos, data[i]));
        } else {
            files.push((c_pos, data[i]));
        }
        c_pos += data[i];
    }

    let mut res: u128 = 0;
    let mut val: u32 = (data.len() - 1) as u32 / 2;
    let mut has_fit = false;
    for file in files.into_iter().rev() {
        has_fit = false;
        for h in holes.iter_mut() {
            // If the hole is after, break
            if h.0 > file.0 {
                break;
            }
            // If if fits in the hole, moving and updating the hole
            if h.1 >= file.1 {
                for i in 0..file.1 {
                    res += ((h.0 + i) * val) as u128;
                }

                // Updating the hole
                h.0 += file.1;
                h.1 -= file.1;
                has_fit = true;
                break;
            }
        }

        if !has_fit {
            // Computing it as it own place
            for i in 0..file.1 {
                res += ((file.0 + i) * val) as u128;
            }
        }

        if val == 0 {
            break;
        }
        val -= 1;
    }

    println!("Part 2: {}", res);
}
