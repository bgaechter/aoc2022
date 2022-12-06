use std::collections::HashSet;

use aoc2022::utils;

fn get_start_of_packet(input: &String) {
    for (index,window) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        let mut unique_chars = HashSet::new();
        window.iter().for_each(|x| {unique_chars.insert(x);});
        if unique_chars.len() == window.len() {
            println!("Packet starts at {}", index+4);
            return;
        }
    }
}

fn get_start_of_message(input: &String) {
    for (index,window) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        let mut unique_chars = HashSet::new();
        window.iter().for_each(|x| {unique_chars.insert(x);});
        if unique_chars.len() == window.len() {
            println!("Message starts at {}", index+14);
            return;
        }
    }
}

fn read_input() {
    let raw: String = utils::load(6);
    get_start_of_packet(&raw);
    get_start_of_message(&raw);
}

fn main() {
    read_input();
}