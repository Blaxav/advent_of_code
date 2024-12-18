mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

pub fn run(day: u64, data: &str) {
    match day {
        1 => day_01::run(data),
        2 => day_02::run(data),
        3 => day_03::run(data),
        4 => day_04::run(data),
        5 => day_05::run(data),
        6 => day_06::run(data),
        7 => day_07::run(data),
        8 => day_08::run(data),
        9 => day_09::run(data),
        10 => day_10::run(data),
        11 => day_11::run(data),
        12 => day_12::run(data),
        13 => day_13::run(data),
        _ => println!("Day {} not implemented yet", day),
    }
}
