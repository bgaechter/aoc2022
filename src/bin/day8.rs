use aoc2022::utils;

fn parse_map(input: &String) -> Vec<Vec<usize>> {
    let mut tree_map: Vec<Vec<usize>> = Vec::new();
    for line in input.trim().lines(){
        let mut tree_line: Vec<usize> = Vec::new();
        for tree in line.split("").filter(|x| !x.is_empty()) {
            tree_line.push(tree.parse::<usize>().unwrap());
        }
        tree_map.push(tree_line);
    }
    tree_map
}

fn part_a(raw: &String) {
    let tree_map = parse_map(raw);
    let mut count = 0;

    for row in 0..tree_map.len() {
        for col in 0..tree_map[row].len() {
            let height = tree_map[row][col];
            if tree_map[..row].iter().all(|x| x[col] < height)
                || tree_map[row][..col].iter().all(|x| x < &height)
                || tree_map[row + 1..].iter().all(|x| x[col] < height)
                || tree_map[row][col + 1..].iter().all(|x| x < &height)
            {
                count += 1;
            }
        }
    }
    println!{"visible trees {}", count};
}

fn part_b(raw: &String) {
    let tree_map = parse_map(raw);
    let mut score = 0;
    for row in 0..tree_map.len() {
        for col in 0..tree_map[row].len() {
            let mut local_east_score: i32 = 0;
            let mut local_west_score: i32 = 0;
            let mut local_north_score: i32 = 0;
            let mut local_south_sccore:i32 = 0;
            for i in tree_map[..row].iter().map(|x| x[col]).rev(){
                local_east_score += 1;
                if i >= tree_map[row][col] {
                    break;
                }
            }
            for i in tree_map[row + 1..].iter().map(|x| x[col]) {
                local_west_score += 1;
                if i >= tree_map[row][col] {
                    break;
                }
            }
            for i in tree_map[row][..col].iter().rev().copied() {
                local_north_score += 1;
                if i >= tree_map[row][col] {
                    break;
                }
            }
            for i in tree_map[row][col + 1..].iter().copied() {
                local_south_sccore += 1;
                if i >= tree_map[row][col] {
                    break;
                }
            }
            score = score.max(local_east_score*local_west_score*local_north_score*local_south_sccore);
        }
    }
    println!("highest scenic score: {}", score);
}

fn main(){
    let raw: String = utils::load(8);
    part_a(&raw);
    part_b(&raw);
}