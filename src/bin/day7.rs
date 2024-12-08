use aoc2024::{concat_nums, input_file, Combs};
use std::{error::Error, io::BufRead};

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    nums: Vec<u64>,
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
                match op {
                    Ops::Plus => sum += n,
                    Ops::Mult => sum *= n,
                }
            }
            // println!("{}: {:?} {:?} => {}", self.result, ops, self.nums, sum);
            if sum == self.result {
                return true;
            }
        }
        false
    }
    fn correct2(&self) -> bool {
        #[derive(Debug, Copy, Clone)]
        enum Ops {
            Plus,
            Mult,
            Concat,
        }
        for ops in Combs::new(self.nums.len() - 1, &[Ops::Plus, Ops::Mult, Ops::Concat]) {
            let mut ns = self.nums.iter();
            let mut sum = ns.next().unwrap().to_owned();
            for (op, &n) in ops.iter().zip(ns) {
                match op {
                    Ops::Plus => sum += n,
                    Ops::Mult => sum *= n,
                    Ops::Concat => sum = concat_nums(sum, n),
                }
            }
            if sum == self.result {
                println!("{}: {:?} {:?} => {}", self.result, ops, self.nums, sum);

                return true;
            }
        }
        println!("Failed {}: {:?}", self.result, self.nums);
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
    for eq in &eqs {
        if eq.correct() {
            sum += eq.result;
        }
    }
    println!("Sum {}", sum);
    let mut sum2 = 0;
    for eq in eqs {
        if eq.correct2() {
            sum2 += eq.result;
        }
    }
    println!("Sum2 {}", sum2);
    // 425283565583384 is too low?
    Ok(())
}
