use aoc2022::utils;

fn read_assignments() {
    let raw = utils::load(4);
    let mut result: usize = 0;
    let mut result_b: usize = 0;
    for line in raw.trim().lines() {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let range_a = ranges[0].split("-").collect::<Vec<&str>>();
        let range_b = ranges[1].split("-").collect::<Vec<&str>>();

        if range_a[0].parse::<u32>().unwrap() <= range_b[0].parse::<u32>().unwrap()
            && range_a[1].parse::<u32>().unwrap() >= range_b[1].parse::<u32>().unwrap()
            || range_b[0].parse::<u32>().unwrap() <= range_a[0].parse::<u32>().unwrap()
                && range_b[1].parse::<u32>().unwrap() >= range_a[1].parse::<u32>().unwrap()
        {
            println!("Full overlap in {}", line);
            result += 1;
        }
        if range_a[0]
            .parse::<u32>()
            .unwrap()
            .max(range_b[0].parse::<u32>().unwrap())
            <= range_a[1]
                .parse::<u32>()
                .unwrap()
                .min(range_b[1].parse::<u32>().unwrap())
        {
            result_b += 1;
        }
    }
    println!("full overlaps: {}", result);
    println!("partial overlaps {}", result_b);
}
fn main() {
    read_assignments();
}
