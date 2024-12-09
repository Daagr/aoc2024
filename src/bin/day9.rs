use aoc2024::input_file;
use std::{collections::VecDeque, error::Error, io::Read};

#[derive(Debug, Clone, Copy)]
enum Usage {
    /// Free(length)
    Free(usize),
    /// Used(id, length)
    Used(usize, usize),
}
use Usage::*;

#[derive(Debug, Clone)]
struct Disk {
    map: Vec<Option<usize>>,
    usages: Vec<Usage>,
    ids: usize,
}

impl Disk {
    fn new(file: impl std::io::Read) -> Disk {
        let mut map = Vec::new();
        let mut usages = Vec::new();
        let mut free = false;
        let mut n = 0;
        for ch in std::io::BufReader::new(file).bytes() {
            let ch = ch.unwrap();
            if ch == b'\n' {
                break;
            }
            let length = ch - b'0';
            assert!(length < 10);
            let val = if free { None } else { Some(n) };
            map.extend(std::iter::repeat(val).take(length as usize));
            usages.push(if free {
                Free(length as usize)
            } else {
                Used(n, length as usize)
            });
            if !free {
                assert_ne!(length, 0);
                n += 1;
            }
            free = !free;
        }
        Disk {
            map,
            usages,
            ids: n,
        }
    }
    fn compact(&mut self) {
        let mut map: Vec<_> = std::iter::repeat(None).take(self.map.len()).collect();
        std::mem::swap(&mut map, &mut self.map);
        let mut old_map = VecDeque::from(map);
        for block in self.map.iter_mut() {
            if old_map.is_empty() {
                break;
            }
            let b = old_map.pop_front().unwrap();
            if b.is_some() {
                *block = b;
                continue;
            }
            while !old_map.is_empty() {
                let b = old_map.pop_back().unwrap();
                if b.is_some() {
                    *block = b;
                    break;
                }
            }
        }
    }
    fn compact2(&mut self) {
        for i in (0..self.ids).rev() {
            let (old_index, Used(_, size)) = self
                .usages
                .iter()
                .enumerate()
                .find(|(_, x)| if let Used(n, _) = **x { i == n } else { false })
                .unwrap()
            else {
                unreachable!()
            };
            if old_index == 0 {
                continue;
            }
            let size = *size;
            let mut extra_space = None;
            let mut new_index = None;
            for (new_index_, usage) in self.usages.iter_mut().enumerate() {
                match *usage {
                    Used(n, _) if n == i => break,
                    Free(free_size) if free_size >= size => {
                        // println!(
                        //     "Moving {} of size {} from {} to {}",
                        //     i, size, old_index, new_index_
                        // );
                        *usage = Used(i, size);
                        new_index = Some(new_index_);
                        if free_size > size {
                            extra_space = Some(free_size - size);
                        }
                        break;
                    }
                    _ => continue,
                }
            }
            if let Some(new_index) = new_index {
                assert!(new_index < old_index);
                // println!("Removing old {} from {}", i, old_index);
                self.usages[old_index] = Free(size);
            }
            if self.usages.len() > old_index {
                if let Free(f1) = self.usages[old_index - 1] {
                    if let Free(f2) = self.usages[old_index] {
                        // println!("Combining free space at {}(-1)", old_index);
                        self.usages.remove(old_index);
                        self.usages[old_index - 1] = Free(f1 + f2);
                    }
                }
            }
            if self.usages.len() > old_index {
                if let Free(f1) = self.usages[old_index - 1] {
                    if let Free(f2) = self.usages[old_index] {
                        // println!("Combining free space at {}(-1) (mark 2)", old_index);
                        self.usages.remove(old_index);
                        self.usages[old_index - 1] = Free(f1 + f2);
                    }
                }
            }
            if let Some(new_index) = new_index {
                if let Some(size) = extra_space {
                    // println!("Adding free space at {}", new_index + 1);
                    self.usages.insert(new_index + 1, Free(size));
                }
            }
        }
    }
    #[allow(dead_code)]
    fn is_compact(&self) -> bool {
        self.map.is_sorted_by(|a, b| {
            if a.is_none() && b.is_some() {
                false
            } else {
                true
            }
        })
    }
    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (i, b) in self.map.iter().enumerate() {
            if let Some(n) = b {
                sum += i * n;
                // println!("sum += i * n || {} += {} * {} || {}", sum, i, n, i * n);
            }
        }
        sum
    }
    fn checksum2(&self) -> usize {
        let mut i = 0;
        let mut sum = 0;
        for usage in self.usages.iter() {
            match usage {
                Free(length) => i += length,
                Used(id, length) => {
                    // there is a formula for this
                    for n in i..i + length {
                        sum += id * n;
                    }
                    i += length;
                }
            }
        }
        sum
    }
    fn usages_to_map(&mut self) {
        self.map.fill(None);
        let mut i = 0;
        for usage in &self.usages {
            match usage {
                Free(len) => i += len,
                Used(id, len) => {
                    for n in i..i + len {
                        self.map[n] = Some(*id);
                    }
                    i += len;
                }
            }
        }
    }
    #[allow(dead_code)]
    fn usage(&self) -> (usize, usize) {
        (
            self.map.iter().filter(|x| x.is_some()).count(),
            self.map.len(),
        )
    }
}
impl ToString for Disk {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for block in &self.map {
            match block {
                None => out.push('.'),
                Some(n @ 0..36) => out.push(char::from_digit(*n as u32, 36).unwrap()),
                _ => out.push('#'),
            }
        }
        out
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut disk = Disk::new(std::fs::File::open(filename)?);
    let mut diskb = disk.clone();
    // println!(
    //     "{}, {}, {:?}",
    //     disk.to_string(),
    //     disk.is_compact(),
    //     disk.usage()
    // );
    disk.compact();
    println!("ChecksumA {}", disk.checksum());
    diskb.compact2();
    println!("ChecksumB {}", diskb.checksum2());
    diskb.usages_to_map();
    // println!(
    //     "{}, {}, {:?}",
    //     diskb.to_string(),
    //     diskb.is_compact(),
    //     diskb.usage()
    // );
    // println!("{:?}", disk.
    println!("ChecksumBalt {}", diskb.checksum());
    Ok(())
}
