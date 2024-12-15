use crate::utils::read_file;

pub fn run(filepath: &str) -> () {
    match read_file(filepath) {
        Ok(lines) => {
            let data: Vec<(u64, Vec<u64>)> = lines
                .into_iter()
                .map(|s| {
                    let splitted: Vec<&str> = s.split(": ").collect();
                    (
                        splitted[0].parse().unwrap(),
                        splitted[1].split(" ").map(|s| s.parse().unwrap()).collect(),
                    )
                })
                .collect();
            part_1(&data);
            part_2(&data);
        }
        Err(e) => println!("{}", e),
    }
}

fn operations(n: usize) -> impl Iterator<Item = Vec<u8>> {
    (0..1 << n).into_iter().map(move |i| {
        let mut binary_representation = Vec::new();
        for bit in (0..n).rev() {
            binary_representation.push(((i >> bit) & 1) as u8);
        }
        binary_representation
    })
}

fn operation_tern(n: usize) -> impl Iterator<Item = Vec<u8>> {
    (0..3usize.pow(n as u32)).map(move |i| {
        let mut representation = Vec::new();
        let mut value = i;

        for _ in 0..n {
            representation.push((value % 3) as u8);
            value /= 3;
        }
        representation
    })
}

fn part_1(data: &Vec<(u64, Vec<u64>)>) -> () {
    let res: u64 = data
        .into_iter()
        .filter_map(|(expected, seq)| {
            let n = seq.len() - 1;

            let operations = operations(n);
            let exist_ok = operations
                .into_iter()
                .map(|op| {
                    let toto = op
                        .into_iter()
                        .zip(&seq[1..])
                        .try_fold(seq[0], |acc, (op, val)| {
                            if acc > *expected {
                                return Err(false);
                            }
                            match op {
                                0 => Ok(acc + val),
                                1 => Ok(acc * val),
                                2 => Ok((acc.to_string() + &val.to_string()).parse().unwrap()),
                                _ => panic!("Unknown operation"),
                            }
                        });

                    match toto {
                        Ok(v) => v == *expected,
                        Err(_) => false,
                    }
                })
                .fold(false, |acc, val| acc || val);

            match exist_ok {
                true => Some(expected),
                false => None,
            }
        })
        .fold(0, |acc, val| acc + val);

    println!("Part 1: {}", res);
}

fn part_2(data: &Vec<(u64, Vec<u64>)>) -> () {
    let res: u64 = data
        .into_iter()
        .filter_map(|(expected, seq)| {
            let n = seq.len() - 1;

            let operations = operation_tern(n);
            let exist_ok = operations
                .into_iter()
                .map(|op| {
                    let toto = op
                        .into_iter()
                        .zip(&seq[1..])
                        .try_fold(seq[0], |acc, (op, val)| {
                            if acc > *expected {
                                return Err(false);
                            }
                            match op {
                                0 => Ok(acc + val),
                                1 => Ok(acc * val),
                                2 => Ok((acc.to_string() + &val.to_string()).parse().unwrap()),
                                _ => panic!("Unknown operation"),
                            }
                        });

                    match toto {
                        Ok(v) => v == *expected,
                        Err(_) => false,
                    }
                })
                .fold(false, |acc, val| acc || val);

            match exist_ok {
                true => Some(expected),
                false => None,
            }
        })
        .fold(0, |acc, val| acc + val);

    println!("Part 1: {}", res);
}
