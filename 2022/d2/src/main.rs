use std::fs;

fn main() {

    let lose = 'X';
    let draw = 'Y';
    let win = 'Z';


    let rock = 'A';
    let paper = 'B';
    let scissors = 'C';
    let input = fs::read_to_string("./input.txt").unwrap();
    // let input = "A Y
// B X
// C Z";
    let mut score = 0;
    for line in input.lines() {
        for _ in 0..1 {
        if line.contains(lose){
            score += 0;
        }
        if line.contains(draw){
            score += 3;
        }
        if line.contains(win){
            score += 6;
        }

        if line.contains(rock) && line.contains(win){
            score += 2;
            break;
        }
        if line.contains(paper) && line.contains(lose){
            score += 1;
            break;
        }
        if line.contains(scissors) && line.contains(draw){
            score += 3;
            break;
        }

        if line.contains(rock) && line.contains(lose){
            score += 3;
            break;
        }
        if line.contains(paper) && line.contains(draw){
            score += 2;
            break;
        }
        if line.contains(scissors) && line.contains(win){
            score += 1;
            break;
        }

        if line.contains(rock) && line.contains(draw){
            score += 1;
            break;
        }
        if line.contains(paper) && line.contains(win){
            score += 3;
            break;
        }
        if line.contains(scissors) && line.contains(lose){
            score += 2;
            break;
        }
    }
        }
    println!("{}", score);
}
