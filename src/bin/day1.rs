use aoc2024::input_file;
use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let lines = std::io::BufReader::new(std::fs::File::open(filename)?).lines();
    let mut av = Vec::new();
    let mut bv = Vec::new();
    for line in lines {
        let line = line?;
        let mut nums = line.split_ascii_whitespace();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();
        // println!("{a} {b}");
        av.push(a);
        bv.push(b);
    }
    av.sort();
    bv.sort();
    let mut sum = 0;
    for (a, b) in av.iter().zip(&bv) {
        sum += (a - b).abs();
    }
    println!("Part 1: {}", sum);
    let mut similarity = 0;
    for a in av {
        let count = bv.iter().filter(|x| **x == a).count() as i32;
        similarity += count * a;
    }
    println!("Part 2: {}", similarity);
    Ok(())
}
