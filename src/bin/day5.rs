use aoc2022::utils;

fn part_a (crates: &mut Vec<Vec<char>>, orders: &Vec<Vec<usize>>) {
    for order in orders {
        for _ in 0..order[0] {
            let old = crates[order[1]].remove(0);
            crates[order[2]].insert(0, old);
        }
    }

    for crate_ in crates.iter().filter(|x| !x.is_empty()) {
        print!("{}", crate_[0]);
    }
    println!("");
}

fn part_b (crates: &mut Vec<Vec<char>>, orders: &Vec<Vec<usize>>) {
    for order in orders {
        let mut buffer:Vec<char> = Vec::new();
        for _ in 0..order[0] {
            buffer.push(crates[order[1]].remove(0));
        }
        for crate_ in buffer.iter().rev() {
            crates[order[2]].insert(0, *crate_);
        }
    }

    for crate_ in crates.iter().filter(|x| !x.is_empty()) {
        print!("{}", crate_[0]);
    }
    println!("");
}

fn read_input() {
    let raw: String = utils::load(5);
    let (raw_crates, raw_orders) = raw.split_once("\n\n").unwrap();
    let crates = parse_crates(raw_crates);
    let orders = parse_orders(raw_orders);

    part_a(&mut crates.clone(), &orders);
    part_b(&mut crates.clone(), &orders);

}

fn parse_orders(raw_orders: &str) -> Vec<Vec<usize>> {
    let mut orders: Vec<Vec<usize>> = vec![];

    for order in raw_orders.trim().lines(){
        let parts = order.split_whitespace().collect::<Vec<_>>();
        let mut order_v: Vec<usize> = vec![];
        order_v.push(parts[1].parse::<usize>().unwrap()); // count
        order_v.push(parts[3].parse::<usize>().unwrap()-1); // from
        order_v.push(parts[5].parse::<usize>().unwrap()-1); // to
        orders.push(order_v);
    }
    orders
}

fn parse_crates(raw_crates: &str) -> Vec<Vec<char>> {
    let mut crates = vec![Vec::new(); 9];
    for line in raw_crates.lines().filter(|x| !x.starts_with(" 1")) {
        for stack in 0..9 {
            if let Some(crate_) = line.chars().nth(1+stack*4) {
                if crate_.is_whitespace() {
                    continue;
                }
                crates[stack].push(crate_);
            }
        }
    }
    crates
}

fn main() {
    read_input();
}