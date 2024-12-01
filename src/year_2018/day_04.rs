use crate::utils::read_file;
use chrono::{NaiveDateTime, Timelike};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

struct Event {
    datetime: NaiveDateTime,
    message: String,
}

impl Event {
    fn from_string(s: &String) -> Option<Event> {
        let re = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)").unwrap();

        if let Some(captures) = re.captures(s) {
            // Capture groups 1 and 2
            let datetime_str = captures.get(1).unwrap().as_str();
            let message = captures.get(2).unwrap().as_str();

            // Parse the datetime string into a NaiveDateTime
            if let Ok(datetime) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M") {
                return Some(Event {
                    datetime: datetime,
                    message: message.to_string(),
                });
            }
        }

        println!("Unable to parse line: {}", s);
        println!("Ignored lines");
        return None;
    }
}

pub fn run(filepath: &str) -> () {
    let mut timetable: Vec<Event> = match read_file(filepath) {
        Ok(lines) => lines
            .par_iter()
            .filter_map(|s| Event::from_string(s))
            .collect(),
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };

    timetable.sort_by(|a, b| a.datetime.cmp(&b.datetime));

    part_1(&timetable);
    part_2(&timetable);
}

fn part_1(data: &Vec<Event>) -> () {
    // For each guard, count both the total number of minutes and
    // records for each observed minute asleep the number of days asleep
    let mut shifts: HashMap<u64, (u32, HashMap<u32, u32>)> = HashMap::new();
    let mut current_guard: u64 = 0;
    let mut begin: u32 = 0;

    for event in data.iter() {
        if event.message.starts_with("Guard") {
            let id = event.message.split(" ").collect::<Vec<&str>>()[1].replace("#", "");
            current_guard = id.parse().unwrap();
        } else if event.message == "falls asleep" {
            begin = event.datetime.minute();
        } else if event.message == "wakes up" {
            let l: u32 = event.datetime.minute() - begin;

            shifts
                .entry(current_guard)
                .and_modify(|(total, minutes)| {
                    *total += l;
                    for m in (begin..event.datetime.minute()).into_iter() {
                        *minutes.entry(m).or_insert(0) += 1
                    }
                })
                .or_insert({
                    let minutes: HashMap<u32, u32> = (begin..event.datetime.minute())
                        .into_iter()
                        .map(|m| (m, 1))
                        .collect::<Vec<(u32, u32)>>()
                        .into_iter()
                        .collect();
                    (l, minutes)
                });
        }
    }

    let best_guard: &u64 = shifts.iter().max_by(|a, b| a.1 .0.cmp(&b.1 .0)).unwrap().0;

    let best_minutes: (u32, u32) = shifts.get(best_guard).unwrap().clone().1.into_iter().fold(
        (0, 0),
        |(acc_min, acc_count), (minute, count)| {
            if count > acc_count {
                (minute, count)
            } else {
                (acc_min, acc_count)
            }
        },
    );
    println!("Part 1: {}", best_guard * best_minutes.0 as u64);
}

fn part_2(data: &Vec<Event>) -> () {
    // For each guard, count both the best minute and
    // records for each observed minute asleep the number of days asleep
    let mut shifts: HashMap<u64, ((u32, u32), HashMap<u32, u32>)> = HashMap::new();
    let mut current_guard: u64 = 0;
    let mut begin: u32 = 0;

    for event in data.iter() {
        if event.message.starts_with("Guard") {
            let id = event.message.split(" ").collect::<Vec<&str>>()[1].replace("#", "");
            current_guard = id.parse().unwrap();
        } else if event.message == "falls asleep" {
            begin = event.datetime.minute();
        } else if event.message == "wakes up" {
            shifts
                .entry(current_guard)
                .and_modify(|((min, n_days), minutes)| {
                    // updates all minutes
                    for m in (begin..event.datetime.minute()).into_iter() {
                        *minutes.entry(m).or_insert(0) += 1;

                        // If a minute occures more often, update
                        // the best
                        if *minutes.get(&m).unwrap() > *n_days {
                            *min = m;
                            *n_days = *minutes.get(&m).unwrap();
                        }
                    }
                })
                .or_insert({
                    let minutes: HashMap<u32, u32> = (begin..event.datetime.minute())
                        .into_iter()
                        .map(|m| (m, 1))
                        .collect::<Vec<(u32, u32)>>()
                        .into_iter()
                        .collect();
                    // Randomly choose begin with 1 occurrence
                    ((begin, 1), minutes)
                });
        }
    }

    let best_guard: &u64 = shifts
        .iter()
        .max_by(|a, b| a.1 .0 .1.cmp(&b.1 .0 .1))
        .unwrap()
        .0;

    println!(
        "Part 2: {}",
        best_guard * shifts.get(best_guard).unwrap().0 .0 as u64
    );
}
