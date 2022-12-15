use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use derive_more::{Add, AddAssign, Mul, Sub};
use num_traits::{abs, signum, Num, Signed};


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load(day: u32) -> String {
    let file = format!("input/day{}.txt", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
pub struct Point<T: Num = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Signed + Copy> Point<T> {
    pub fn normalize(&self) -> Self {
        Self {
            x: signum(self.x),
            y: signum(self.y),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: abs(self.x),
            y: abs(self.y),
        }
    }
}

impl<T: Num + Ord + Copy> Point<T> {
    pub fn max(&self) -> T {
        self.x.max(self.y)
    }
}