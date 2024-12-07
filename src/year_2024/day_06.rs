use crate::utils::read_file;
use std::collections::HashSet;
use std::time::Instant;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let data: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn next_dir(dx: i32, dy: i32) -> (i32, i32) {
    // Give next direction clockwise
    let len = DIRS.len();
    if let Some(index) = DIRS.iter().position(|&dir| dir == (dy, dx)) {
        DIRS[(index + 1) % len]
    } else {
        panic!("Unknown direction (dx, dy) = ({}, {})", dx, dy);
    }
}

fn find_start(data: &Vec<Vec<char>>) -> (i32, i32, i32, i32) {
    // Returns (y, x, dy, dx)
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == '<' {
                return (i as i32, j as i32, 0, -1);
            }
            if data[i][j] == '>' {
                return (i as i32, j as i32, 0, 1);
            }
            if data[i][j] == '^' {
                return (i as i32, j as i32, -1, 0);
            }
            if data[i][j] == 'v' {
                return (i as i32, j as i32, 1, 0);
            }
        }
    }

    panic!("No start found");
}

fn is_inside(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn part_1(data: &Vec<Vec<char>>) -> () {
    let width = data[0].len() as i32;
    let height = data.len() as i32;

    let mut observed: HashSet<(i32, i32)> = HashSet::new();

    let (mut y, mut x, mut dy, mut dx) = find_start(data);
    observed.insert((x, y));

    loop {
        if !is_inside(x + dx, y + dy, width, height) {
            // If next is outside, ciao
            break;
        } else if data[(y + dy) as usize][(x + dx) as usize] == '#' {
            // If next is a rock, change direction
            (dy, dx) = next_dir(dx, dy);
        } else {
            // Otherwise, move and store new position
            x += dx;
            y += dy;
            observed.insert((x, y));
        }
    }

    println!("Part 1: {}", observed.len());
}

fn does_loop(data: &Vec<Vec<char>>, obstacle: (i32, i32)) -> (bool, HashSet<(i32, i32)>) {
    let width = data[0].len() as i32;
    let height = data.len() as i32;

    let mut observed: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    let (mut y, mut x, mut dy, mut dx) = find_start(data);
    observed.insert((x, y, dx, dy));

    loop {
        if !is_inside(x + dx, y + dy, width, height) {
            // If next is outside, does not loop
            // Returns false, along with the list of positions where we could put an osbstable
            // to break the current path and create a loop
            return (
                false,
                observed
                    .into_iter()
                    .fold(HashSet::new(), |mut acc, (x, y, _, _)| {
                        acc.insert((x, y));
                        acc
                    }),
            );
        } else if data[(y + dy) as usize][(x + dx) as usize] == '#' || (x + dx, y + dy) == obstacle
        {
            // If next is a rock, change direction
            (dy, dx) = next_dir(dx, dy);
        } else {
            // Otherwise, move and store new position
            x += dx;
            y += dy;
            if observed.contains(&(x, y, dx, dy)) {
                // We're back to a known state, it's a loop !
                return (true, HashSet::new());
            }
            observed.insert((x, y, dx, dy));
        }
    }
}

fn part_2(data: &Vec<Vec<char>>) -> () {
    let now = Instant::now();

    let (y, x, _, _) = find_start(data);
    let mut loopers = 0;

    let (_, obstacles_to_check) = does_loop(data, (x, y));

    obstacles_to_check.into_iter().for_each(|(j, i)| {
        // If there is no block and it's not start, can try to pose a block
        if data[i as usize][j as usize] != '#' && (x, y) != (j, i) {
            let obstacle = (j, i);
            if does_loop(data, obstacle).0 {
                loopers += 1;
            }
        }
    });

    println!(
        "Part 2: {} in {:.2}s",
        loopers,
        now.elapsed().as_millis() as f64 / 1000.0
    );
}
