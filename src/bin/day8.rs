use aoc2024::{input_file, Combs};
use std::{collections::HashMap, error::Error, io::BufRead};

#[derive(Debug)]
struct Grid {
    size: (i64, i64),
    antennas: HashMap<char, Vec<(i64, i64)>>,
    poles: Vec<(i64, i64)>,
}

impl Grid {
    fn new(file: impl std::io::Read) -> Grid {
        let mut antennas: HashMap<_, Vec<_>> = HashMap::new();
        let mut y = 0;
        let mut width = None;
        for line in std::io::BufReader::new(file).lines() {
            let line = line.unwrap();
            let mut x = 0;
            for ch in line.chars() {
                if ch != '.' {
                    antennas.entry(ch).or_default().push((x as i64, y as i64));
                }
                x += 1;
            }
            if width.is_none() {
                width = Some(x);
            }
            y += 1;
        }
        Grid {
            size: (width.unwrap() as i64, y as i64),
            antennas,
            poles: vec![],
        }
    }
    fn generate_poles(&mut self) {
        for v in self.antennas.values() {
            for ab in Combs::new(2, v) {
                let [a, b] = &ab[..] else { unreachable!() };
                if a == b {
                    continue;
                }
                let x = 2 * a.0 - b.0;
                let y = 2 * a.1 - b.1;
                self.poles.push((x, y));
            }
        }
        self.poles.sort();
        self.poles.dedup();
    }
    fn generate_polelines(&mut self) {
        let max_n = self.size.0.max(self.size.1);
        for v in self.antennas.values() {
            for ab in Combs::new(2, v) {
                let [a, b] = &ab[..] else { unreachable!() };
                if a == b {
                    continue;
                }
                // TODO: gcd (wtf, I got the correct answer without gcd???)
                let x_ = a.0 - b.0;
                let y_ = a.1 - b.1;
                for n in -max_n..max_n {
                    let x = n * x_ + a.0;
                    let y = n * y_ + a.1;
                    self.poles.push((x, y));
                }
            }
        }
        self.poles.sort();
        self.poles.dedup();
    }
    fn count(&self) -> usize {
        self.poles
            .iter()
            .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < self.size.0 && p.1 < self.size.1)
            .count()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut grid = Grid::new(std::fs::File::open(filename)?);
    // println!("{:?}", grid);
    grid.generate_poles();
    let num = grid.count();
    println!("Result {}", num);
    grid.poles.clear();
    grid.generate_polelines();
    println!("Resultb {}", grid.count());
    Ok(())
}
