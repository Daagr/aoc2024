use aoc2024::input_file;
use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let input: Vec<Vec<char>> = std::io::BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    let needle = "XMAS";
    // println!("{:?} {}", input, input[0][4]);
    let count = count_forward(&input, needle);
    // input.reverse();
    // input.iter_mut().for_each(|x| x.reverse());
    // count += count_forward(&input, needle);
    println!("Count: {}", count);
    Ok(())
}

fn count_forward(input: &Vec<Vec<char>>, needle: &'static str) -> i32 {
    let height = input.len();
    let width = input[0].len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            // search right
            if x <= width - needle.len() {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y][x + i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct right from {} {}", x, y);
                    count += 1;
                }
            }
            // search down
            if y <= height - needle.len() {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y + i][x] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct down from {} {}", x, y);
                    count += 1;
                }
            }
            // search diagonal downright
            if x <= width - needle.len() && y <= height - needle.len() {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y + i][x + i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct downright from {} {}", x, y);
                    count += 1;
                }
            }
            // search downleft
            if x >= needle.len() - 1 && y <= height - needle.len() {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y + i][x - i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct downleft from {} {}", x, y);
                    count += 1;
                }
            }
            // search upleft
            if x >= needle.len() - 1 && y >= needle.len() - 1 {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y - i][x - i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct upleft from {} {}", x, y);
                    count += 1;
                }
            }
            // search upright
            if y >= needle.len() - 1 && x <= height - needle.len() {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y - i][x + i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct upright from {} {}", x, y);
                    count += 1;
                }
            }
            // search up
            if y >= needle.len() - 1 {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y - i][x] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct up from {} {}", x, y);
                    count += 1;
                }
            }
            // search left
            if x >= needle.len() - 1 {
                let mut correct = true;
                for i in 0..needle.len() {
                    if input[y][x - i] != needle.chars().nth(i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if correct {
                    println!("Correct left from {} {}", x, y);
                    count += 1;
                }
            }
        }
    }
    count
}
