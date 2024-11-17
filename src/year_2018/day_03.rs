use crate::utils::read_file;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {}
        Err(e) => println!("Error {}", e),
    }
}
