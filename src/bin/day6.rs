use aoc2024::input_file;
use std::{error::Error, io::BufRead};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Position {
    Obstruction,
    Empty,
    Visited(Direction),
}
impl Position {
    fn is_visited(&self) -> bool {
        match self {
            Position::Visited(_) => true,
            _ => false,
        }
    }
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
                        row.push(Position::Visited(Direction::Up));
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
                if pos.is_visited() {
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
    /// Returns true if looping
    fn step(&mut self) -> bool {
        let mut i = 0;
        while self.at(self.next_pos()) == Position::Obstruction {
            self.turn();
            i += 1;
            if i > 5 {
                return true;
            }
        }
        self.location = self.next_pos();
        if let Some((x, y)) = self.location {
            if let Position::Visited(dir) = self.positions[y as usize][x as usize] {
                if dir == self.direction {
                    return true;
                }
            } else {
                self.positions[y as usize][x as usize] = Position::Visited(self.direction);
            }
        }
        false
    }
    /// Returns true on loop
    fn run(&mut self) -> bool {
        while self.location.is_some() {
            if self.step() {
                return true;
            }
        }
        false
    }
    fn print(&self) {
        for line in &self.positions {
            for p in line {
                match p {
                    Position::Obstruction => print!("#"),
                    Position::Empty => print!("."),
                    Position::Visited(_) => print!("X"),
                }
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut grid = Grid::new(std::fs::File::open(filename)?);
    let orig_grid = grid.clone();
    //println!("{:?}", grid);
    grid.print();
    println!();
    grid.run();
    grid.print();
    println!("Visited {}", grid.count());
    let mut stucks = 0;
    for y in 0..grid.size.1 {
        for x in 0..grid.size.0 {
            if grid.at(Some((x as i32, y as i32))).is_visited() {
                if let Some((x_, y_)) = orig_grid.location {
                    if x_ as usize != x || y_ as usize != y {
                        let mut new_grid = orig_grid.clone();
                        new_grid.positions[y][x] = Position::Obstruction;
                        if new_grid.run() {
                            println!("Stuck at ({}, {})", x, y);
                            stucks += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Stucks {}", stucks);
    /*
    let mut new_grid = orig_grid.clone();
    new_grid.positions[6][3] = Position::Obstruction;
    if new_grid.run() {
        println!("Stuck???");
    }
    new_grid.print();
    */
    Ok(())
}
