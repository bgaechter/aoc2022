use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_ints(path: &str) -> Vec<i32> {
    let mut elves: Vec<i32> = vec![0];
    let mut index: i32 = 0;
    let lines = read_lines(path).unwrap();

    for line in lines {
        match line.unwrap().parse::<i32>() {
            Ok(num) => {
                elves[index as usize] += num;
            }
            _ => {
                println!(
                    "Elve #{} is carrying {} calories",
                    index, elves[index as usize]
                );
                index += 1;
                elves.push(0);
            }
        };
    }
    elves
}

fn main() {
    let mut elves = read_ints("input/day1.txt");

    // Task 1 Top Elve
    match elves.iter().max() {
        Some(max) => println!("Max value: {}", max),
        None => println!("Vector is empty"),
    }
    // Task 2 Top three Elves
    elves.sort_by(|a, b| b.cmp(a));
    let sum: i32 = elves.iter().take(3).sum();
    println!("Top Three: {}", sum);
}
