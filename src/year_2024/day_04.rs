use crate::utils::read_file;
use rayon::prelude::*;

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

const LETTERS: &[char] = &['X', 'M', 'A', 'S'];
const DIRECTIONS: &[(i32, i32)] = &[
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

/// Function that check if we can form the expected word in a given direction
fn is_valid(data: &Vec<Vec<char>>, x: i32, y: i32, letter_id: usize, dx: i32, dy: i32) -> bool {
    // Are we still in the boundaries ?
    if x + dx < 0 || x + dx >= data[0].len() as i32 || y + dy < 0 || y + dy >= data.len() as i32 {
        return false;
    }

    // Not the expected letter
    if data[(y + dy) as usize][(x + dx) as usize] != LETTERS[letter_id] {
        return false;
    }

    // Otherwise it's the right letter
    if letter_id == LETTERS.len() - 1 {
        // If it's the last we're good
        return true;
    } else {
        return is_valid(data, x + dx, y + dy, letter_id + 1, dx, dy);
    }
}

fn part_1(data: &Vec<Vec<char>>) -> () {
    let width = data[0].len();
    let height = data.len();

    // In order to use Rayon parallelism, build the vec of tuple of positions first
    let all_starts: Vec<(i32, i32)> = (0..height)
        .flat_map(move |y| {
            (0..width).filter_map(move |x| {
                if data[y][x] == LETTERS[0] {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    // Evaluate all possible start concurrently
    let all_totals = all_starts
        .into_par_iter()
        .map(|(x, y)| {
            let mut current_valid_count = 0;
            for (dx, dy) in DIRECTIONS {
                if is_valid(data, x as i32, y as i32, 1, *dx, *dy) {
                    current_valid_count += 1;
                }
            }
            current_valid_count
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .fold(0, |acc, val| acc + val);

    println!("Part 1: {}", all_totals);
}

fn is_valid_p2(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || x == data[0].len() - 1 || y == 0 || y == data.len() - 1 {
        return false;
    }

    let outcomes: Vec<(char, char, char, char)> = vec![
        ('M', 'M', 'S', 'S'),
        ('M', 'S', 'S', 'M'),
        ('S', 'S', 'M', 'M'),
        ('S', 'M', 'M', 'S'),
    ];

    return outcomes
        .into_iter()
        .map(|o| {
            (
                data[y + 1][x + 1],
                data[y + 1][x - 1],
                data[y - 1][x - 1],
                data[y - 1][x + 1],
            ) == o
        })
        .fold(false, |acc, val| acc || val);
}

fn part_2(data: &Vec<Vec<char>>) -> () {
    let width = data[0].len();
    let height = data.len();

    let res = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .filter_map(move |x| {
                    if data[y][x] == 'A' {
                        if is_valid_p2(data, x, y) {
                            return Some(1);
                        }
                    }
                    None
                })
                .fold(0, |acc, val| acc + val)
        })
        .fold(0, |acc, val| acc + val);

    println!("Part 2: {}", res);
}
