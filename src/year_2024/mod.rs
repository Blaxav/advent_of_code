mod day_01;
mod day_02;

pub fn run(day: u64, data: &str) {
    match day {
        1 => day_01::run(data),
        2 => day_02::run(data),
        _ => println!("Day {} not implemented yet", day),
    }
}
