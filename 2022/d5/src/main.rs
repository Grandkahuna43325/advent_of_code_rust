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
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut instructions = vec![];
    let mut instructions_iter = vec![];

    for line in input.lines() {
        let splited: Vec<&str> = line.split(" ").collect();
        let instruction = Move {
            how_much: splited[1].parse().unwrap(),
            from: splited[3].parse().unwrap(),
            to: splited[5].parse().unwrap(),
        };
        instructions.push(instruction);
        instructions_iter.push(instruction);
    }

    let input_crates = fs::read_to_string("./input_crates.txt").unwrap();

    let mut lines = input_crates.lines();

    let mut stackvec = vec![];
    for i in 1..=9 {
        let mut contents = vec![];
        while let Some(ch) = is_anumber(lines.next().unwrap().to_string()) {
            contents.push(ch);
        }
        contents.reverse();

        let stack = Stack {
            stack_number: i,
            contents,
        };
        stackvec.push(stack);
    }
    // [stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9] 



    let mut tempinstructions = vec![];
    let temp = Move{from:1, to: 2, how_much:2};
    tempinstructions.push(temp);
    // let temp = Move{from:3, to: 4, how_much:3};
    // tempinstructions.push(temp);




let mut char = 'x';
let mut charvec = vec![];

    for instruction in instructions {
        for i in 1..=instruction.how_much {
            char = match instruction.from {
                x if x == 1 => {
                    stackvec[0].contents.pop().unwrap()
                },
                x if x == 2 => {
                    stackvec[1].contents.pop().unwrap()
                },
                x if x == 3 => {
                    stackvec[2].contents.pop().unwrap()
                },
                x if x == 4 => {
                    stackvec[3].contents.pop().unwrap()
                },
                x if x == 5 => {
                    stackvec[4].contents.pop().unwrap()
                },
                x if x == 6 => {
                    stackvec[5].contents.pop().unwrap()
                },
                x if x == 7 => {
                    stackvec[6].contents.pop().unwrap()
                },
                x if x == 8 => {
                    stackvec[7].contents.pop().unwrap()
                },
                x if x == 9 => {
                    stackvec[8].contents.pop().unwrap()
                },
                _ => {'x'}
            };
            charvec.push(char);

        }
        charvec.reverse();
        for ch in &charvec {
            stackvec[instruction.to as usize -1].contents.push(*ch);
        }
        charvec.clear();
        char = 'x'
    }
    for i in 0..9 {
        println!("{:?}", stackvec[i].contents);
    }
}

fn is_anumber(line: String) -> Option<char> {
    match line.parse::<i32>() {
        Ok(_) => None,
        Err(_) => {
            let mut chars = line.chars();
            chars.next();
            chars.next()
        }
    }
}
