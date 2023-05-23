#![allow(unused_variables)]

use std::fs;
use std::str::Chars;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let len = input.len();

    let input = input.chars();

    let marker_char_number = find_marker(input.clone(), len, 4);

    println!("Part 1: {}", marker_char_number);

    let marker_char_number = find_marker(input.clone(), len, 14);

    println!("Part 2: {}", marker_char_number);
}

fn find_marker(mut input: Chars, len: usize, length_of_marker: i32) -> i32 {
    let mut counter = length_of_marker;
    let mut shortvec = vec![];
    for _ in 0..length_of_marker {
        // Getting starting chars to check_for_dupe_in_vec
        shortvec.push(input.next().unwrap())
    }
    for _ in 0..len {
        if !check_for_dupe_in_vec(&shortvec) {
            break;
        }
        shortvec.reverse();
        shortvec.pop();
        shortvec.reverse();
        shortvec.push(input.next().unwrap());

        counter += 1;
    }
    counter
}

fn check_for_dupe_in_vec(vec: &[char]) -> bool {
    for (i, item) in vec.iter().enumerate() {
        if vec.iter().skip(i + 1).any(|x| x == item) {
            return true;
        }
    }
    false
}
