use aoc2024::input_file;
use regex::Regex;
use std::{error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut input = String::new();
    std::io::BufReader::new(std::fs::File::open(filename)?).read_to_string(&mut input)?;
    let mut suma: i32 = 0;
    let mut sumb: i32 = 0;
    let mut enabled = true;
    let re = Regex::new(r"(mul\(\d\d?\d?,\d\d?\d?\))|(do\(\))|(don't\(\))")?;

    for mul in re.find_iter(&input) {
        // println!("{:?}", mul);
        match &mul.as_str()[0..3] {
            "do(" => {
                enabled = true;
            }
            // don't:
            "don" => {
                enabled = false;
            }
            "mul" => {
                let mut nums = mul.as_str()[4..mul.len() - 1].split(',');
                let a: i32 = nums.next().unwrap().parse()?;
                let b: i32 = nums.next().unwrap().parse()?;
                suma += a * b;
                if enabled {
                    sumb += a * b;
                }
            }
            _ => {
                panic!("Unexpected command")
            }
        }
    }
    println!("Suma {} ; sumb {}", suma, sumb);
    Ok(())
}
