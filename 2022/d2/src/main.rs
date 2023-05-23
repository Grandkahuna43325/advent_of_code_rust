use std::fs;

const LOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

const ROCK_: char = 'X';
const PAPER_: char = 'Y';
const SCISSORS_: char = 'Z';

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    // part 1
    let mut score = 0;

    for line in input.lines() {
        match line {
            line if line.contains(ROCK_) => score += 1,
            line if line.contains(PAPER_) => score += 2,
            line if line.contains(SCISSORS_) => score += 3,
            _ => {
                println!("Error");
            }
        }

        match line {
            line if line.contains(ROCK) && line.contains(ROCK_) => score += 3,
            line if line.contains(PAPER) && line.contains(PAPER_) => score += 3,
            line if line.contains(SCISSORS) && line.contains(SCISSORS_) => score += 3,
            line if line.contains(ROCK) && line.contains(PAPER_) => score += 6,
            line if line.contains(PAPER) && line.contains(SCISSORS_) => score += 6,
            line if line.contains(SCISSORS) && line.contains(ROCK_) => score += 6,
            _ => {}
        }
    }

    println!("Part 1 score: {}", score);

    //part 2
    let mut score = 0;

    for line in input.lines() {
        match line {
            line if line.contains(LOSE) => score += 0,
            line if line.contains(DRAW) => score += 3,
            line if line.contains(WIN) => score += 6,
            _ => {}
        }

        match line {
            line if line.contains(ROCK) && line.contains(WIN) => score += 2,
            line if line.contains(PAPER) && line.contains(LOSE) => score += 1,
            line if line.contains(SCISSORS) && line.contains(DRAW) => score += 3,
            line if line.contains(SCISSORS) && line.contains(WIN) => score += 1,
            line if line.contains(ROCK) && line.contains(LOSE) => score += 3,
            line if line.contains(PAPER) && line.contains(DRAW) => score += 2,
            line if line.contains(PAPER) && line.contains(WIN) => score += 3,
            line if line.contains(SCISSORS) && line.contains(LOSE) => score += 2,
            line if line.contains(ROCK) && line.contains(DRAW) => score += 1,
            _ => {}
        }
    }
    println!("part 2 score: {}", score);
}
