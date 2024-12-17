use crate::utils::read_file;
use std::collections::{HashMap, HashSet};

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let data: HashMap<(i32, i32), char> = lines
                .into_iter()
                .enumerate()
                .map(|(j, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(i, c)| ((i as i32, j as i32), c))
                        .collect::<Vec<((i32, i32), char)>>()
                })
                .flatten()
                .collect();

            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn part_1(data: &HashMap<(i32, i32), char>) -> () {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    // Initialize elements to check with the start
    let mut to_check: Vec<(i32, i32)> = Vec::new();
    to_check.push((0, 0));

    let mut res: u128 = 0;

    while !to_check.is_empty() {
        // Initialize a new field
        let (x, y) = to_check.pop().unwrap();

        // Belongs to a field that has been already explored
        if seen.contains(&(x, y)) {
            continue;
        }

        let c_val = data.get(&(x, y)).unwrap();

        let mut field_size: u64 = 0;
        let mut touched: u64 = 0;

        let mut current_to_check: Vec<(i32, i32)> = Vec::new();
        current_to_check.push((x, y));

        while !current_to_check.is_empty() {
            let (x, y) = current_to_check.pop().unwrap();

            // Was added several times in to_check, no worries
            if seen.contains(&(x, y)) {
                continue;
            }

            // Update current field area
            field_size += 1;

            // Check all neighbors
            for (dx, dy) in DIRECTIONS {
                // If we finish outside of the map, ciao
                if !data.contains_key(&(x + dx, y + dy)) {
                    continue;
                }

                // if this is not the current value, not in the same field
                if data.get(&(x + dx, y + dy)).unwrap() != c_val {
                    to_check.push((x + dx, y + dy));
                    continue;
                }

                touched += 1;
                // If this element has not been evaluated, add it to the current list
                if !seen.contains(&(x + dx, y + dy)) {
                    current_to_check.push((x + dx, y + dy));
                }
            }

            seen.insert((x, y));
        }

        // Finally found all field elements, update result
        res += (4 * field_size - touched) as u128 * field_size as u128;
    }

    println!("Part 1 : {}", res);
}

fn part_2(data: &HashMap<(i32, i32), char>) -> () {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    // Initialize elements to check with the start
    let mut to_check: Vec<(i32, i32)> = Vec::new();
    to_check.push((0, 0));

    let mut res: u128 = 0;

    while !to_check.is_empty() {
        // Initialize a new field
        let (x, y) = to_check.pop().unwrap();

        // Belongs to a field that has been already explored
        if seen.contains(&(x, y)) {
            continue;
        }

        let c_val = data.get(&(x, y)).unwrap();

        let mut field_size: u64 = 0;
        let mut touched: u64 = 0;

        let mut current_to_check: Vec<(i32, i32)> = Vec::new();
        current_to_check.push((x, y));

        let mut h_sides: HashSet<(i32, i32)> = HashSet::new();
        let mut v_sides: HashSet<(i32, i32)> = HashSet::new();

        while !current_to_check.is_empty() {
            let (x, y) = current_to_check.pop().unwrap();

            // Was added several times in to_check, no worries
            if seen.contains(&(x, y)) {
                continue;
            }

            // Update current field area
            field_size += 1;

            // Check all neighbors
            for (dx, dy) in DIRECTIONS {
                // If we finish outside of the map, ciao
                if !data.contains_key(&(x + dx, y + dy)) {
                    if (dx, dy) == (1, 0) {
                        v_sides.insert((x + dx, y + dy));
                    } else if (dx, dy) == (-1, 0) {
                        v_sides.insert((x, y));
                    } else if (dx, dy) == (0, 1) {
                        h_sides.insert((x + dx, y + dy));
                    } else if (dx, dy) == (0, -1) {
                        h_sides.insert((x, y));
                    }
                    continue;
                }

                // if this is not the current value, not in the same field
                // this defines a side
                if data.get(&(x + dx, y + dy)).unwrap() != c_val {
                    to_check.push((x + dx, y + dy));
                    // vertical sides
                    if (dx, dy) == (1, 0) {
                        v_sides.insert((x + dx, y + dy));
                    } else if (dx, dy) == (-1, 0) {
                        v_sides.insert((x, y));
                    } else if (dx, dy) == (0, 1) {
                        h_sides.insert((x + dx, y + dy));
                    } else if (dx, dy) == (0, -1) {
                        h_sides.insert((x, y));
                    }
                    continue;
                }

                touched += 1;
                // If this element has not been evaluated, add it to the current list
                if !seen.contains(&(x + dx, y + dy)) {
                    current_to_check.push((x + dx, y + dy));
                }
            }

            seen.insert((x, y));
        }

        let mut h_sides_vec: Vec<(i32, i32)> = h_sides.into_iter().collect();
        h_sides_vec.sort_by_key(|k| (k.1, k.0));
        let mut v_sides_vec: Vec<(i32, i32)> = v_sides.into_iter().collect();
        v_sides_vec.sort_by_key(|k| (k.0, k.1));

        let mut n_sides: u64 = 0;
        let mut previous: (i32, i32) = (0, 0);
        for (i, (_x, _y)) in v_sides_vec.into_iter().enumerate() {
            if i == 0 {
                n_sides += 1
            } else {
                if _x == previous.0 && _y == previous.1 + 1 {
                    let left: (char, char) = (
                        if data.contains_key(&(previous.0 - 1, previous.1)) {
                            data[&(previous.0 - 1, previous.1)]
                        } else {
                            '<'
                        },
                        if data.contains_key(&(_x - 1, _y)) {
                            data[&(_x - 1, _y)]
                        } else {
                            '<'
                        },
                    );

                    let right: (char, char) = (
                        if data.contains_key(&(previous.0, previous.1)) {
                            data[&(previous.0, previous.1)]
                        } else {
                            '<'
                        },
                        if data.contains_key(&(_x, _y)) {
                            data[&(_x, _y)]
                        } else {
                            '<'
                        },
                    );

                    if left != (c_val.clone(), c_val.clone())
                        && right != (c_val.clone(), c_val.clone())
                    {
                        n_sides += 1;
                    }
                } else {
                    n_sides += 1;
                }
            }
            previous = (_x, _y).clone()
        }

        for (i, (_x, _y)) in h_sides_vec.into_iter().enumerate() {
            if i == 0 {
                n_sides += 1
            } else {
                if _y == previous.1 && _x == previous.0 + 1 {
                    let above: (char, char) = (
                        if data.contains_key(&(previous.0, previous.1 - 1)) {
                            data[&(previous.0, previous.1 - 1)]
                        } else {
                            '<'
                        },
                        if data.contains_key(&(_x, _y - 1)) {
                            data[&(_x, _y - 1)]
                        } else {
                            '<'
                        },
                    );

                    let behind: (char, char) = (
                        if data.contains_key(&(previous.0, previous.1)) {
                            data[&(previous.0, previous.1)]
                        } else {
                            '<'
                        },
                        if data.contains_key(&(_x, _y)) {
                            data[&(_x, _y)]
                        } else {
                            '<'
                        },
                    );

                    if above != (c_val.clone(), c_val.clone())
                        && behind != (c_val.clone(), c_val.clone())
                    {
                        n_sides += 1;
                    }
                } else {
                    //println!("New side");
                    n_sides += 1;
                }
            }
            previous = (_x, _y).clone()
        }

        // Finally found all field elements, update result
        res += n_sides as u128 * field_size as u128;
    }

    println!("Part 2 : {}", res);
}
