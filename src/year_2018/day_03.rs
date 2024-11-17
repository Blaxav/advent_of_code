use crate::utils::read_file;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn run(filepath: &str) -> () {
    let claims: Vec<Claim> = match read_file(filepath) {
        Ok(lines) => lines.par_iter().map(|s| Claim::from_string(s)).collect(),
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };

    part_1(&claims);
    part_2(&claims);
}

struct Claim {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

impl Claim {
    /// Claim string structure
    /// #1 @ 1,3: 4x4
    fn from_string(s: &String) -> Claim {
        let split = s.split(" ").collect::<Vec<&str>>();
        let pos = split[2].split(",").collect::<Vec<&str>>();
        let size = split[3].split("x").collect::<Vec<&str>>();
        Claim {
            id: split[0].replace("#", "").parse().unwrap(),
            x: pos[0].parse().unwrap(),
            y: pos[1].replace(":", "").parse().unwrap(),
            width: size[0].parse().unwrap(),
            height: size[1].parse().unwrap(),
        }
    }
}

fn part_1(data: &Vec<Claim>) -> () {
    let mut map: HashMap<(u64, u64), u64> = HashMap::new();

    for claim in data {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    let cnt: u64 = map.into_iter().filter(|(_, v)| *v > 1).count() as u64;

    println!("Part 1: {}", cnt);
}

fn part_2(data: &Vec<Claim>) -> () {
    let mut map: HashMap<(u64, u64), u64> = HashMap::new();

    // Set of ids observed as overlapped
    let mut overlapped: HashSet<u64> = HashSet::new();

    for claim in data {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if map.contains_key(&(x, y)) {
                    overlapped.insert(claim.id);
                    overlapped.insert(map[&(x, y)]);
                } else {
                    map.insert((x, y), claim.id);
                }
            }
        }
    }

    let not_overlapped: HashSet<u64> = HashSet::from_iter(data.into_iter().map(|c| c.id))
        .difference(&overlapped)
        .cloned()
        .collect();

    if not_overlapped.len() != 1 {
        println!("Part 2 not returning a unique element as expected.");
        println!("Returned {:?}", not_overlapped);
        return;
    }

    println!("Part 2 : {}", &not_overlapped.into_iter().next().unwrap());
}
