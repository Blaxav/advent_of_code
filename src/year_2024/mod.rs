mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub fn run(day: u64, data: &str) {
    match day {
        1 => day_01::run(data),
        2 => day_02::run(data),
        3 => day_03::run(data),
        4 => day_04::run(data),
        5 => day_05::run(data),
        _ => println!("Day {} not implemented yet", day),
    }
}
