use aoc2024::input_file;
use std::{error::Error, io::Read, mem::swap};

fn try_split_num(n: u64) -> Option<(u64, u64)> {
    let digits = n.ilog10() + 1;
    if digits % 2 == 1 {
        None
    } else {
        let half_length = 10u64.pow(digits / 2);
        Some((n / half_length, n % half_length))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_num() {
        assert_eq!(try_split_num(1), None);
        assert_eq!(try_split_num(12), Some((1, 2)));
        assert_eq!(try_split_num(123), None);
        assert_eq!(try_split_num(1234), Some((12, 34)));

        assert_eq!(try_split_num(1000), Some((10, 0)));
    }
}

/// Used for part 1
#[allow(dead_code)]
fn blink(nums: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for n in nums {
        if *n == 0 {
            out.push(1);
        } else if let Some((a, b)) = try_split_num(*n) {
            out.push(a);
            out.push(b);
        } else {
            out.push(*n * 2024);
        }
    }
    out
}

fn blink_one(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        (1, None)
    } else if let Some((a, b)) = try_split_num(num) {
        (a, Some(b))
    } else {
        (2024 * num, None)
    }
}

/// faster for part 1 but not fast enough for part 2
#[allow(dead_code)]
fn stonecount(num: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if num == 0 {
        return stonecount(1, blinks - 1);
    }
    if let Some((a, b)) = try_split_num(num) {
        return stonecount(a, blinks - 1) + stonecount(b, blinks - 1);
    }
    return stonecount(2024 * num, blinks - 1);
}

#[derive(Debug, Copy, Clone)]
struct Stone {
    mul: u64,
    num: u64,
    blinks: u64,
}

fn stonecount2(nums: &[u64], blinks: u64) -> u64 {
    let mut todo = nums
        .iter()
        .map(|num| Stone {
            mul: 1,
            num: *num,
            blinks,
        })
        .collect::<Vec<_>>();
    let mut todo_ = Vec::new();
    while !todo.iter().all(|s| s.blinks == 0) {
        todo.sort_by_key(|s| (s.blinks, s.num));
        todo_.clear();

        // dedup:
        let mut elem = todo.pop().unwrap();
        while let Some(other) = todo.pop() {
            if elem.num == other.num && elem.blinks == other.blinks {
                elem.mul += other.mul;
            } else {
                todo_.push(elem);
                elem = other;
            }
        }
        todo_.push(elem);

        swap(&mut todo, &mut todo_);
        todo.reverse();
        elem = todo.pop().unwrap();
        //println!("blinking {:?}", elem);
        assert_ne!(elem.blinks, 0);
        let (a, b) = blink_one(elem.num);
        todo.push(Stone {
            mul: elem.mul,
            num: a,
            blinks: elem.blinks - 1,
        });
        if let Some(a) = b {
            todo.push(Stone {
                mul: elem.mul,
                num: a,
                blinks: elem.blinks - 1,
            });
        }
    }

    let mut sum = 0;
    for stone in todo {
        sum += stone.mul;
        assert_eq!(stone.blinks, 0);
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut buf = String::new();
    std::fs::File::open(filename)?.read_to_string(&mut buf)?;
    let nums: Vec<u64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let count = stonecount2(&nums, 25);

    println!("Numbercount at 25: {}", count);

    let count2 = stonecount2(&nums, 75);
    println!("Numbercount at 75: {}", count2);

    Ok(())
}
