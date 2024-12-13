use aoc2024::{input_file, IsExample};
use std::{error::Error, io::BufRead};

struct Map {
    areas: Vec<Vec<u8>>,
}
impl Map {
    fn new(file: impl std::io::Read) -> Map {
        let areas = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().bytes().collect())
            .collect();
        Map { areas }
    }
    fn at(&self, x: impl TryInto<usize>, y: impl TryInto<usize>) -> Option<u8> {
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        #[allow(unused_comparisons)] // Yeah, it's part of TryInto<usize> but I like to be explicit
        if x < 0 || x >= self.areas[0].len() {
            return None;
        }
        #[allow(unused_comparisons)]
        if y < 0 || y >= self.areas.len() {
            return None;
        }
        Some(self.areas[y][x])
    }

    fn count_area_and_perimeter(
        &self,
        (x, y): (isize, isize),
    ) -> (usize, usize, usize, Vec<(isize, isize)>) {
        //let (x, y) = (x as isize, y as isize);
        let plant = self.at(x, y).unwrap();
        let mut inside = Vec::new();
        let mut outside = Vec::new();
        let mut todo = vec![(x, y)];
        let mut walls = 0;
        while let Some((x, y)) = todo.pop() {
            if inside.contains(&(x, y)) {
                continue;
            }
            if outside.contains(&(x, y)) {
                walls += 1;
                continue;
            }
            if let Some(plant_) = self.at(x, y) {
                if plant == plant_ {
                    // first time finding this spot
                    inside.push((x, y));
                    for neighbor in &[(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        todo.push(*neighbor);
                    }
                    continue;
                }
            }
            // not the correct plant
            outside.push((x, y));
            walls += 1;
        }

        let mut vertical_walls = Vec::new();
        let mut horizontal_walls = Vec::new();
        for (ox, oy) in &outside {
            for (ix, iy) in &inside {
                if (ox - ix).abs() == 1 && (oy - iy) == 0 {
                    vertical_walls.push((ox.min(ix)..ox.max(ix), oy, ix < ox));
                }
                if (ox - ix) == 0 && (oy - iy).abs() == 1 {
                    horizontal_walls.push((ox, oy.min(iy)..oy.max(iy), iy < oy));
                }
            }
        }
        //println!("vert: {:?}\nhori: {:?}", vertical_walls, horizontal_walls);
        vertical_walls.sort_by_key(|(x, y, _)| (*x.start, *y));
        horizontal_walls.sort_by_key(|(x, y, _)| (*y.start, *x));
        let mut sides = 2;
        for (last, next) in vertical_walls.iter().zip(vertical_walls[1..].iter()) {
            if last.0 != next.0 || last.1 + 1 != *next.1 || last.2 != next.2 {
                sides += 1;
            }
        }
        for (last, next) in horizontal_walls.iter().zip(horizontal_walls[1..].iter()) {
            if last.1 != next.1 || last.0 + 1 != *next.0 || last.2 != next.2 {
                sides += 1;
            }
        }

        // println!(
        //     "checked {:?}  and found area of {} with perimeter of {} letter {}",
        //     (x, y),
        //     inside.len(),
        //     walls,
        //     char::from(plant),
        // );

        assert_eq!(walls, horizontal_walls.len() + vertical_walls.len());
        (inside.len(), walls, sides, inside)
    }

    fn result(&self) -> (u64, u64) {
        let mut marked = vec![vec![false; self.areas[0].len()]; self.areas.len()];
        let mut n: isize = 0;
        let width = self.areas[0].len() as isize;
        let mut sum = 0;
        let mut sum2 = 0;
        while !marked.iter().flatten().all(|x| *x) {
            let (x, y) = (n % width, n / width);
            n += 1;
            // marked[y as usize][x as usize] = true;
            // if self.at(x, y) == self.at(x, y - 1) || self.at(x, y) == self.at(x - 1, y) {
            //     continue;
            // }
            if marked[y as usize][x as usize] {
                continue;
            }
            let (area, walls, sides, inside) = self.count_area_and_perimeter((x, y));
            for (x_, y_) in inside {
                marked[y_ as usize][x_ as usize] = true;
            }
            sum += area * walls;
            sum2 += area * sides;
            //println!("area, sides, * : {} {} {}", area, sides, area * sides);
        }
        (sum as u64, sum2 as u64)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(IsExample::No);
    let map = Map::new(std::fs::File::open(filename)?);
    // println!(
    //     "0,0 area and perimeter {:?}",
    //     map.count_area_and_perimeter((0, 0))
    // );
    println!("Trailsum: {:?}", map.result());
    Ok(())
}
