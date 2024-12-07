use aoc2024::input_file;
use std::{error::Error, io::BufRead};

#[derive(Debug)]
struct Combs<'a, Elem> {
    len: usize,
    n: usize,
    elems: &'a [Elem],
}
impl<'a, Elem: Copy> Iterator for Combs<'a, Elem> {
    type Item = Vec<Elem>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.elems.len().pow(self.len as u32) {
            return None;
        }
        let mut it = Vec::with_capacity(self.len);
        let mut n = self.n;
        for _ in 0..self.len {
            it.push(self.elems[n % self.elems.len()]);
            n /= self.elems.len();
        }
        self.n += 1;
        Some(it)
    }
}
impl<'a, Elem> Combs<'a, Elem> {
    fn new(len: usize, elems: &'a [Elem]) -> Self {
        Combs { len, n: 0, elems }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combs() {
        assert_eq!(
            Combs::new(1, &[1, 2]).collect::<Vec<_>>(),
            vec![vec![1], vec![2]]
        );
        assert_eq!(
            Combs::new(2, &[1, 2]).collect::<Vec<_>>(),
            vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]
        );
    }
}
#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    nums: Vec<i64>,
}
impl Equation {
    fn new(line: &str) -> Equation {
        let mut colon = line.split(':');
        let result = colon.next().unwrap().parse().unwrap();
        let nums = colon
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Equation { result, nums }
    }
    fn correct(&self) -> bool {
        #[derive(Debug, Copy, Clone)]
        enum Ops {
            Plus,
            Mult,
        }
        for ops in Combs::new(self.nums.len() - 1, &[Ops::Plus, Ops::Mult]) {
            let mut ns = self.nums.iter();
            let mut sum = ns.next().unwrap().to_owned();
            //let mut prod = 0;
            for (op, &n) in ops.iter().zip(ns) {
                /*
                match op {
                    Ops::Plus => {
                        sum += n;
                        prod = n;
                    }
                    Ops::Mult => {
                        if prod == 0 {
                            prod = sum;
                        }
                        sum -= prod;
                        prod *= n;
                        sum += prod;
                    }
                }
                */
                match op {
                    Ops::Plus => sum += n,
                    Ops::Mult => sum *= n,
                }
            }
            println!("{}: {:?} {:?} => {}", self.result, ops, self.nums, sum);
            if sum == self.result {
                return true;
            }
        }
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let eqs: Vec<Equation> = std::io::BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|x| Equation::new(&x.unwrap()))
        .collect();
    let mut sum = 0;
    for eq in eqs {
        if eq.correct() {
            sum += eq.result;
        }
    }
    println!("Sum {}", sum);
    Ok(())
}
