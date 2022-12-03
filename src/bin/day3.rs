use std::collections::HashSet;

use aoc2022::utils;

fn score_item(char_: char) -> u8 {
    match char_ as u8 {
        97..=122 => char_ as u8 - 96,
        65..=90 => char_ as u8 - 38,
        _ => unreachable!(),
    }
}

fn read_rucksacks() {
    let raw = utils::load(3);
    let mut priority: usize = 0;
    for line in raw.trim().lines() {
        let mut rucksack = line.chars().collect::<Vec<_>>();
        let compartments = line.split_at(rucksack.len() / 2);
        let compartment_b = compartments.1.chars().collect::<Vec<_>>();
        
        rucksack.retain(|x| compartment_b.contains(x));
        rucksack.dedup();

        priority += score_item(rucksack[0]) as usize;
    }
    println!("Priority sum: {}", priority);
}

fn find_badges() {
    let raw = utils::load(3);
    let mut priority: usize = 0;
    for group in raw.trim().lines().collect::<Vec<_>>().chunks(3) {
        let mut group_set = HashSet::new();
        group.iter().for_each(|x| group_set.extend(x.chars())); 
        group.iter().for_each(|x| group_set.retain(|y| x.contains(*y)));
        
        priority += score_item(*group_set.iter().next().unwrap()) as usize;
    }
    println!("Priority sum: {}", priority);

}

fn main() {
    read_rucksacks();
    find_badges();
}
