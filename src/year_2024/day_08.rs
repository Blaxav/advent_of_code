use crate::utils::read_file;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    // For each antenna char, vec of all positions
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
            let mut data: Vec<Vec<char>> = Vec::new();
            let height: usize = lines.len();

            let mut i = 0;
            let mut j = 0;
            lines.into_iter().for_each(|s| {
                let current_line: Vec<char> = s.chars().collect();

                // Populate known antennas
                j = 0;
                current_line.iter().for_each(|c| {
                    if *c != '.' {
                        antennas
                            .entry(*c)
                            .or_insert(Vec::new())
                            .push((i as i32, j as i32));
                    }
                    j += 1;
                });

                data.push(current_line);

                i += 1;
            });

            let map: Map = Map {
                width: data[0].len(),
                height: height,
                data: data,
                antennas: antennas,
            };
            part_1(&map);
            part_2(&map);
        }
        Err(e) => println!("{}", e),
    }
}

fn part_1(map: &Map) -> () {
    let mut pos: HashSet<(i32, i32)> = HashSet::new();

    map.antennas.iter().for_each(|(_, positions)| {
        for (i, (y1, x1)) in positions.iter().enumerate() {
            for (y2, x2) in positions[i + 1..].iter() {
                pos.insert((2 * y1 - y2, 2 * x1 - x2));
                pos.insert((2 * y2 - y1, 2 * x2 - x1));
            }
        }
    });

    let unique_pos = pos
        .into_iter()
        .filter(|(y, x)| *y < map.height as i32 && *x < map.width as i32 && *y >= 0 && *x >= 0)
        .count();

    println!("Part 1: {}", unique_pos);
}

fn part_2(map: &Map) -> () {
    let mut pos: HashSet<(i32, i32)> = HashSet::new();

    map.antennas.iter().for_each(|(_, positions)| {
        for (i, (y1, x1)) in positions.iter().enumerate() {
            for (y2, x2) in positions[i + 1..].iter() {
                let mut k = 1;
                loop {
                    let c_y = y1 + k * (y2 - y1);
                    let c_x = x1 + k * (x2 - x1);

                    if c_y < 0 || c_x < 0 || c_y >= map.height as i32 || c_x >= map.width as i32 {
                        break;
                    }

                    k += 1;
                    pos.insert((c_y, c_x));
                }

                let mut k = 1;
                loop {
                    let c_y = y2 + k * (y1 - y2);
                    let c_x = x2 + k * (x1 - x2);

                    if c_y < 0 || c_x < 0 || c_y >= map.height as i32 || c_x >= map.width as i32 {
                        break;
                    }

                    k += 1;
                    pos.insert((c_y, c_x));
                }
            }
        }
    });

    let unique_pos: Vec<(i32, i32)> = pos
        .into_iter()
        .filter(|(y, x)| *y < map.height as i32 && *x < map.width as i32 && *y >= 0 && *x >= 0)
        .collect();

    println!("Part 2: {}", unique_pos.len());
}
