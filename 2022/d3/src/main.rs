#![allow(dead_code)]
use std::{fs, vec};

#[derive(Debug)]
struct CharAndBool {
    bool: bool,
    char: Option<char>,
}

#[derive(Debug)]
struct Triple {
    s1: String,
    s2: String,
    s3: String,
}

fn contain_same_chars(s1: &str, s2: &str) -> CharAndBool {
    for c in s1.chars() {
        if s2.contains(c) {
            return CharAndBool {
                bool: true,
                char: Some(c),
            };
        }
    }
    return CharAndBool {
        bool: true,
        char: None,
    };
}

fn find_char_in_triline(triple: Triple) -> char {
    for char in triple.s1.chars() {
        if triple.s2.contains(char) && triple.s3.contains(char) {
            return char;
        }
    }
    'e' // this 'e' is for no errors because of return type
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut triples_vec = vec![];

    let mut input_for_tripple = input.lines().into_iter();

    for _ in 0..100 {
        triples_vec.push(Triple {
            s1: input_for_tripple.next().unwrap().to_string(),
            s2: input_for_tripple.next().unwrap().to_string(),
            s3: input_for_tripple.next().unwrap().to_string(),
        });
    }

    let mut all_chars = vec![];
    for line in input.lines() {
        let s = contain_same_chars(&line[0..line.len() / 2], &line[line.len() / 2..]);

        match s {
            s if s.bool => {
                all_chars.push(s.char.unwrap());
            }
            _ => {}
        }
    }

    let count = letter_counter(all_chars);

    println!("Part 1: {count}");

    let mut triple_all_chars = vec![];

    for triple in triples_vec {
        triple_all_chars.push(find_char_in_triline(triple));
    }

    let count = letter_counter(triple_all_chars);
    println!("Part 2: {count}");
}

fn letter_counter(all_chars: Vec<char>) -> i32 {
    let mut count = 0;
    for ch in all_chars.iter() {
        match ch {
            c if c == &'a' => count += 1,
            c if c == &'b' => count += 2,
            c if c == &'c' => count += 3,
            c if c == &'d' => count += 4,
            c if c == &'e' => count += 5,
            c if c == &'f' => count += 6,
            c if c == &'g' => count += 7,
            c if c == &'h' => count += 8,
            c if c == &'i' => count += 9,
            c if c == &'j' => count += 10,
            c if c == &'k' => count += 11,
            c if c == &'l' => count += 12,
            c if c == &'m' => count += 13,
            c if c == &'n' => count += 14,
            c if c == &'o' => count += 15,
            c if c == &'p' => count += 16,
            c if c == &'q' => count += 17,
            c if c == &'r' => count += 18,
            c if c == &'s' => count += 19,
            c if c == &'t' => count += 20,
            c if c == &'u' => count += 21,
            c if c == &'v' => count += 22,
            c if c == &'w' => count += 23,
            c if c == &'x' => count += 24,
            c if c == &'y' => count += 25,
            c if c == &'z' => count += 26,
            c if c == &'A' => count += 27,
            c if c == &'B' => count += 28,
            c if c == &'C' => count += 29,
            c if c == &'D' => count += 30,
            c if c == &'E' => count += 31,
            c if c == &'F' => count += 32,
            c if c == &'G' => count += 33,
            c if c == &'H' => count += 34,
            c if c == &'I' => count += 35,
            c if c == &'J' => count += 36,
            c if c == &'K' => count += 37,
            c if c == &'L' => count += 38,
            c if c == &'M' => count += 39,
            c if c == &'N' => count += 40,
            c if c == &'O' => count += 41,
            c if c == &'P' => count += 42,
            c if c == &'Q' => count += 43,
            c if c == &'R' => count += 44,
            c if c == &'S' => count += 45,
            c if c == &'T' => count += 46,
            c if c == &'U' => count += 47,
            c if c == &'V' => count += 48,
            c if c == &'W' => count += 49,
            c if c == &'X' => count += 50,
            c if c == &'Y' => count += 51,
            c if c == &'Z' => count += 52,
            _ => (),
        }
    }
    count
}
