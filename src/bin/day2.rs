use aoc2022::utils;

struct Round {
    opponent_move: String,
    player_move: String,
    points: i32,
    points_t2: i32,
}

impl Round {
    fn calculate_score(&mut self){
        if self.opponent_move == "A" && self.player_move == "X" {
            self.points += 1+3;
        }
        if self.opponent_move == "A" && self.player_move == "Y" {
            self.points += 2+6;
        }
        if self.opponent_move == "A" && self.player_move == "Z" {
            self.points += 3;
        }
        if self.opponent_move == "B" && self.player_move == "X" {
            self.points += 1;
        }
        if self.opponent_move == "B" && self.player_move == "Y" {
            self.points += 2+3;
        }
        if self.opponent_move == "B" && self.player_move == "Z" {
            self.points += 3+6;
        }
        if self.opponent_move == "C" && self.player_move == "X" {
            self.points += 1+6;
        }
        if self.opponent_move == "C" && self.player_move == "Y" {
            self.points += 2;
        }
        if self.opponent_move == "C" && self.player_move == "Z" {
            self.points += 3+3;
        }
    }

    fn calculate_move(&mut self) {

        if self.player_move == "X" {
            // loose
            if self.opponent_move == "A" {
                self.points_t2 += 3;
            }
            if self.opponent_move == "B" {
                self.points_t2 += 1;
            }
            if self.opponent_move == "C" {
                self.points_t2 += 2;
            }
        }
        if self.player_move == "Y" {
            // draw
            if self.opponent_move == "A" {
                self.points_t2 += 1+3;
            }
            if self.opponent_move == "B" {
                self.points_t2 += 2+3;
            }
            if self.opponent_move == "C" {
                self.points_t2 += 3+3;
            }
        }
        if self.player_move == "Z" {
            // win
            if self.opponent_move == "A" {
                self.points_t2 += 2+6;
            }
            if self.opponent_move == "B" {
                self.points_t2 += 3+6;
            }
            if self.opponent_move == "C" {
                self.points_t2 += 1+6;
            }
        }
    }
}

fn read_rounds(path: &str) {
    let lines = utils::read_lines(path).unwrap();
    let rounds: Vec<Round> = Vec::new();
    let mut total_score: i32 = 0;
    let mut total_score_2: i32 = 0;

    for line in lines {
        let l: String = line.unwrap();
        let v: Vec<&str> = l.split(' ').collect();
        if v.len() <= 1 {
            break;
        }
        let mut round: Round = Round{opponent_move: v[0].into(), player_move: v[1].into(), points: 0, points_t2: 0};
        round.calculate_score();
        round.calculate_move();
        total_score += round.points;
        total_score_2 += round.points_t2   
    }
    println!("Total Score: {}", total_score);
    println!("Total Score task 2: {}", total_score_2);
}

fn main() {
    read_rounds("input/day2.txt");
}
