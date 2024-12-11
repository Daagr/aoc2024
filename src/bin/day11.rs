use aoc2024::input_file;
use std::{error::Error, io::Read};

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

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(true);
    let mut buf = String::new();
    std::fs::File::open(filename)?.read_to_string(&mut buf)?;
    let mut nums: Vec<u64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", nums);
    for _ in 0..25 {
        nums = blink(&nums);
    }
    println!("Numbercount at 25: {}", nums.len());

    Ok(())
}
