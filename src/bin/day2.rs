use aoc2024::input_file;
use std::{error::Error, io::BufRead};

fn is_safe(nums: &[i32]) -> bool {
    let mut nums = nums.iter();
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
    let mut dampextra = 0;
    for line in lines {
        let nums: Vec<i32> = line?
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let safe = is_safe(&nums);
        if safe {
            count += 1;
        } else {
            for i in 0..nums.len() {
                let mut dnums = nums.clone();
                dnums.remove(i);
                if is_safe(&dnums) {
                    dampextra += 1;
                    break;
                }
            }
        }
    }
    println!("Safe count: {}", count);
    println!("Damp safe: {}", count + dampextra);
    Ok(())
}
