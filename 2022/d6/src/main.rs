#![allow(unused_variables)]

use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let len = input.len() / 4;
    // let mut input = "jplbgvbhsrlpgdmjqwftvncz".to_string();
    let mut input = input.chars();

    let mut shortvec = vec![];
    let mut counter = 0;
    for i in 0..len  {
        for ch in 1..=4 {
            shortvec.push(input.next().unwrap());
            counter += 1;
        }
        if !check_for_dupe(shortvec.clone()) {
            break;
        }
        shortvec.clear()
    }
    println!("{counter}");
}

fn check_for_dupe(vector: Vec<char>) -> bool {
    let mut shortvec = vector;
    shortvec.reverse();
    if has_duplicates(&shortvec) {
        return true;
    }
    false
}

fn has_duplicates<T: Eq>(vec: &[T]) -> bool {
    for (i, item) in vec.iter().enumerate() {
        if vec.iter().skip(i + 1).any(|x| x == item) {
            return true;
        }
    }
    false
}
