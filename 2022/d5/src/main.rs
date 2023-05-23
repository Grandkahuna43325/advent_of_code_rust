#![allow(dead_code)]
use std::fs;

#[derive(Debug)]
struct Stack {
    stack_number: i32,
    contents: Vec<char>,
}

impl Stack {
    fn pop(self: &mut Self) -> Option<char> {
        self.contents.pop()
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    how_much: i32,
    from: i32,
    to: i32,
}

#[derive(Debug)]
struct MoveVec {
    vector: Vec<Move>,
}

fn main() {
    let instructions = fs::read_to_string("./input.txt").unwrap();

    let mut instructions_vec = vec![];
    for line in instructions.lines() {
        let splited: Vec<&str> = line.split(" ").collect();
        instructions_vec.push(Move {
            how_much: splited[1].parse().unwrap(),
            from: splited[3].parse().unwrap(),
            to: splited[5].parse().unwrap(),
        });
    }
    let instructions_vec_part2 = instructions_vec.clone();

    let input_crates = fs::read_to_string("./input_crates.txt").unwrap();
    let mut lines = input_crates.lines();

    let mut stack_vec = vec![];
    let mut stack_vec_2 = vec![];
    for stack_number in 1..=9 {
        let mut contents = vec![];
        while let Some(ch) = is_a_number(lines.next().unwrap().to_string()) {
            contents.push(ch);
        }
        contents.reverse();

        stack_vec.push(Stack {
            stack_number,
            contents: contents.clone(),
        });
        stack_vec_2.push(Stack {
            stack_number,
            contents,
        });
    }

    let reverse = false;
    let mut stack_vec = execute_instructions(instructions_vec, stack_vec, reverse);
    let reverse = true;
    let mut stack_vec_2 = execute_instructions(instructions_vec_part2, stack_vec_2, reverse);

    let mut part_one = String::from("");

    for i in 0..9 {
        part_one.push(stack_vec[i].contents.pop().unwrap());
    }
    println!(r#"Part 1: "{part_one}" "#);

    let mut part_two = String::from("");

    for i in 0..9 {
        part_two.push(stack_vec_2[i].contents.pop().unwrap());
    }
    println!(r#"Part 2: "{part_two}" "#);
}

fn execute_instructions(
    instructions_vec: Vec<Move>,
    mut stackvec: Vec<Stack>,
    reverse: bool,
) -> Vec<Stack> {
    let mut char_vec = vec![];

    for instruction in instructions_vec {
        for _ in 1..=instruction.how_much {
            char_vec.push(
                stackvec[instruction.from as usize - 1]
                    .contents
                    .pop()
                    .unwrap(),
            );
        }
        if reverse == true {
            char_vec.reverse();
        }
        for ch in &char_vec {
            stackvec[instruction.to as usize - 1].contents.push(*ch);
        }
        char_vec.clear();
    }
    stackvec
}

fn is_a_number(line: String) -> Option<char> {
    match line.parse::<i32>() {
        Ok(_) => None,
        Err(_) => {
            let mut line = line.chars();
            line.next();
            line.next()
        }
    }
}
