use crate::utils::read_file;
use std::collections::{HashMap, HashSet};

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let data: HashMap<(i32, i32), u32> = lines
                .into_iter()
                .enumerate()
                .map(|(j, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(i, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
                        .collect::<Vec<((i32, i32), u32)>>()
                })
                .flatten()
                .collect();

            /* let data: Vec<Vec<i32>> = lines
            .into_iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(); */

            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn reach_from(
    data: &HashMap<(i32, i32), u32>,
    cache: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pos: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut res: HashSet<(i32, i32)> = HashSet::new();

    for (d_x, d_y) in DIRECTIONS {
        // data containing the (x,y) is enough to know that the value is in the map
        if !data.contains_key(&(pos.0 + d_x, pos.1 + d_y)) {
            continue;
        }

        // Consider only neighbors with +1 value
        if data[&(pos.0 + d_x, pos.1 + d_y)] != data[&pos] + 1 {
            continue;
        }

        // If the neighbor is a 9, add it to the res
        if data[&(pos.0 + d_x, pos.1 + d_y)] == 9 {
            res.insert((pos.0 + d_x, pos.1 + d_y));
            continue;
        }

        // Otherwise, we need to check further by computing recursively the value until it reached 9
        // Setting the value in the cache if it does'nt exist
        if !cache.contains_key(&(pos.0 + d_x, pos.1 + d_y)) {
            let reachable_nines = reach_from(data, cache, (pos.0 + d_x, pos.1 + d_y));
            cache.insert((pos.0 + d_x, pos.1 + d_y), reachable_nines);
        }
        res.extend(cache[&(pos.0 + d_x, pos.1 + d_y)].clone());
    }
    res
}

fn part_1(data: &HashMap<(i32, i32), u32>) -> () {
    // Cache count how many 9's can be reached from a given position
    let mut cache: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    let starts: Vec<(&(i32, i32), &u32)> = data.iter().filter(|(_, v)| **v == 0).collect();

    // Computing from each zero
    let res: u64 = starts.into_iter().fold(0, |acc, ((x, y), _)| {
        acc + reach_from(data, &mut cache, (*x, *y)).len() as u64
    });

    println!("Part 1: {}", res);
}

fn reach_from_p2(
    data: &HashMap<(i32, i32), u32>,
    cache: &mut HashMap<(i32, i32), u64>,
    pos: (i32, i32),
) -> u64 {
    let mut res: u64 = 0;

    for (d_x, d_y) in DIRECTIONS {
        // data containing the (x,y) is enough to know that the value is in the map
        if !data.contains_key(&(pos.0 + d_x, pos.1 + d_y)) {
            continue;
        }

        // Consider only neighbors with +1 value
        if data[&(pos.0 + d_x, pos.1 + d_y)] != data[&pos] + 1 {
            continue;
        }

        // If the neighbor is a 9, add it to the res
        if data[&(pos.0 + d_x, pos.1 + d_y)] == 9 {
            res += 1;
            continue;
        }

        // Otherwise, we need to check further by computing recursively the value until it reached 9
        // Setting the value in the cache if it does'nt exist
        if !cache.contains_key(&(pos.0 + d_x, pos.1 + d_y)) {
            let reachable_nines = reach_from_p2(data, cache, (pos.0 + d_x, pos.1 + d_y));
            cache.insert((pos.0 + d_x, pos.1 + d_y), reachable_nines);
        }
        res += cache[&(pos.0 + d_x, pos.1 + d_y)].clone();
    }
    res
}

fn part_2(data: &HashMap<(i32, i32), u32>) -> () {
    // Cache now count each 9 from a different path as a different value
    let mut cache: HashMap<(i32, i32), u64> = HashMap::new();

    let starts: Vec<(&(i32, i32), &u32)> = data.iter().filter(|(_, v)| **v == 0).collect();

    // Computing from each zero
    let res: u64 = starts.into_iter().fold(0, |acc, ((x, y), _)| {
        acc + reach_from_p2(data, &mut cache, (*x, *y)) as u64
    });

    println!("Part 2: {}", res);
}
