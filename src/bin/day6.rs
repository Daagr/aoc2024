use aoc2024::input_file;
use std::{error::Error, io::BufRead};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Position {
    Obstruction,
    Empty,
    Visited,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone)]
struct Grid {
    size: (usize, usize),
    positions: Vec<Vec<Position>>,
    location: Option<(i32, i32)>,
    direction: Direction,
}

impl Grid {
    fn new(file: impl std::io::Read) -> Grid {
        let mut positions = Vec::new();
        let mut location = None;
        for line in std::io::BufReader::new(file).lines() {
            let mut row = Vec::new();
            for char in line.unwrap().chars() {
                match char {
                    '.' => row.push(Position::Empty),
                    '#' => row.push(Position::Obstruction),
                    '^' => {
                        assert_eq!(location, None);
                        location = Some((row.len() as i32, positions.len() as i32));
                        row.push(Position::Visited);
                    }
                    _ => unreachable!(),
                }
            }
            positions.push(row);
        }

        Grid {
            size: (positions.len(), positions[0].len()),
            positions,
            location,
            direction: Direction::Up,
        }
    }
    fn count(&self) -> usize {
        let mut n = 0;
        for row in &self.positions {
            for pos in row {
                if pos == &Position::Visited {
                    n += 1;
                }
            }
        }
        n
    }
    fn next_pos(&self) -> Option<(i32, i32)> {
        let mut x;
        let mut y;
        if let Some((x_, y_)) = self.location {
            x = x_;
            y = y_;
        } else {
            return None;
        }

        match self.direction {
            Direction::Up => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        }
        if x < 0 || y < 0 || x >= self.size.1 as i32 || y >= self.size.1 as i32 {
            return None;
        }
        Some((x, y))
    }
    fn turn(&mut self) {
        use Direction::*;
        let new_dir = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
        self.direction = new_dir;
    }
    fn at(&self, pos: Option<(i32, i32)>) -> Position {
        if let Some((x, y)) = pos {
            return self.positions[y as usize][x as usize];
        }
        Position::Empty
    }
    fn step(&mut self) {
        let mut i = 0;
        while self.at(self.next_pos()) == Position::Obstruction {
            self.turn();
            i += 1;
            if i > 10 {
                panic!("Infinite turning loop");
            }
        }
        self.location = self.next_pos();
        if let Some((x, y)) = self.location {
            self.positions[y as usize][x as usize] = Position::Visited;
        }
    }
    fn run(&mut self) {
        while self.location.is_some() {
            self.step();
        }
    }
    fn print(&self) {
        for line in &self.positions {
            for p in line {
                match p {
                    Position::Obstruction => print!("#"),
                    Position::Empty => print!("."),
                    Position::Visited => print!("X"),
                }
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut grid = Grid::new(std::fs::File::open(filename)?);
    //println!("{:?}", grid);
    grid.print();
    println!();
    grid.run();
    grid.print();
    println!("Visited {}", grid.count());
    Ok(())
}
