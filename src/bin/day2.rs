use aoc2024::input_file;
use std::{error::Error, io::BufRead};

fn is_safe(line: &str) -> bool {
    let mut nums = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap());
    let mut last = nums.next().unwrap();
    let mut increasing = None;
    for n in nums {
        match increasing {
            None => increasing = Some(n > last),
            Some(true) => {
                if n < last {
                    return false;
                }
            }
            Some(false) => {
                if n > last {
                    return false;
                }
            }
        }
        if n == last || (n - last).abs() > 3 {
            return false;
        }
        last = n;
    }
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let lines = std::io::BufReader::new(std::fs::File::open(filename)?).lines();
    let mut count = 0;
    for line in lines {
        if is_safe(&line?) {
            count += 1;
        }
    }
    println!("Safe count: {}", count);
    Ok(())
}
