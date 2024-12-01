mod day_01;

pub fn run(day: u64, data: &str) {
    match day {
        1 => day_01::run(data),
        _ => println!("Day {} not implemented yet", day),
    }
}
