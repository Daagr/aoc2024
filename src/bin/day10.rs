use aoc2024::input_file;
use std::{error::Error, io::BufRead};

struct Map {
    ns: Vec<Vec<i8>>,
}
impl Map {
    fn new(file: impl std::io::Read) -> Map {
        let ns = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i8)
                    .collect()
            })
            .collect();
        Map { ns }
    }
    fn trailsum(&self) -> (usize, usize) {
        let mut to_check = Vec::new();
        for y in 0..self.ns.len() {
            for x in 0..self.ns[0].len() {
                if self.at(x, y).unwrap() == 0 {
                    to_check.push((x as isize, y as isize, 0, (x, y)));
                }
            }
        }
        //println!("to_check: {:?}", to_check);
        let mut routes = Vec::new();
        while !to_check.is_empty() {
            let (x, y, n, origin) = to_check.pop().unwrap();
            let directions = &[(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (next_x, next_y) in directions {
                if self.at(*next_x, *next_y) == Some(n + 1) {
                    //println!("Found {} at {} {}", n + 1, next_x, next_y);
                    if n + 1 == 9 {
                        routes.push((origin, (*next_x, *next_y)))
                    } else {
                        to_check.push((*next_x, *next_y, n + 1, origin));
                    }
                }
            }
        }
        //println!("routes: {:?}", routes);
        let distinct = routes.len();
        routes.sort();
        routes.dedup();
        (routes.len(), distinct)
    }
    fn at(&self, x: impl TryInto<usize>, y: impl TryInto<usize>) -> Option<i8> {
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        #[allow(unused_comparisons)] // Yeah, it's part of TryInto<usize> but I like to be explicit
        if x < 0 || x >= self.ns[0].len() {
            return None;
        }
        #[allow(unused_comparisons)]
        if y < 0 || y >= self.ns.len() {
            return None;
        }
        Some(self.ns[y][x])
    }
    fn print(&self) {
        for line in &self.ns {
            for digit in line {
                print!("{}", digit);
            }
            println!();
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let map = Map::new(std::fs::File::open(filename)?);
    map.print();
    println!("Trailsum: {:?}", map.trailsum());
    Ok(())
}
