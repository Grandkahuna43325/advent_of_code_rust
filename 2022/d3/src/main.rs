use std::{fs, vec};

#[derive(Debug)]
struct char_and_bool {
    bool: bool,
    char: Option<char>,
}

#[derive(Debug)]
struct triple {
    s1: String,
    s2: String,
    s3: String
}

fn contain_same_chars(s1: &str, s2: &str) -> char_and_bool {
    for c in s1.chars() {
        if s2.contains(c) {
            return char_and_bool {
                bool: true,
                char: Some(c),
            };
        }
    }
    return char_and_bool {
        bool: true,
        char: None,
    };
}

fn find_char_in_triline(triple: triple) -> char {
    let s1_chars = triple.s1.chars();
    let s2_chars = triple.s2;
    let s3_chars = triple.s3;
    for c in s1_chars {
        if s2_chars.contains(c) && s3_chars.contains(c) {
            return c;
        }
    }
    's'
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let inp = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let mut trlines = vec![];

    let mut i = input.lines().into_iter();

    for _ in 0..100{
        trlines.push(triple{
            s1: i.next().unwrap().to_string(),
            s2: i.next().unwrap().to_string(),
            s3: i.next().unwrap().to_string()
        }) ;
    }





    // let f_half = &inp[0..14];
    // let s_half = &inp[14..];
    let mut all_chars = vec![];
    // for line in input.lines() {
    //     if contain_same_chars(&line[0..line.len() / 2], &line[line.len() / 2..]).bool {
    //         all_chars
    //             .push(contain_same_chars(&line[0..line.len() / 2], &line[line.len() / 2..]).char);
    //     }
    // }


    for triple in trlines {
        all_chars.push(find_char_in_triline(triple));
    }


    let mut count = 0;

    for c in all_chars.iter() {
                match c {
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
            _ => count += 9000000
                }
        };

    println!("{count}");

    // let a = 1;
    // let b = 2;
    // let c = 3;
    // let d = 4;
    // let e = 5;
    // let f = 6;
    // let g = 7;
    // let h = 8;
    // let i = 9;
    // let j = 10;
    // let k = 11;
    // let l = 12;
    // let m = 13;
    // let n = 14;
    // let o = 15;
    // let p = 16;
    // let q = 17;
    // let r = 18;
    // let s = 19;
    // let t = 20;
    // let u = 21;
    // let v = 22;
    // let w = 23;
    // let x = 24;
    // let y = 25;
    // let z = 26;
    // let A = 27;
    // let B = 28;
    // let C = 29;
    // let D = 30;
    // let E = 31;
    // let F = 32;
    // let G = 33;
    // let H = 34;
    // let I = 35;
    // let J = 36;
    // let K = 37;
    // let L = 38;
    // let M = 39;
    // let N = 40;
    // let O = 41;
    // let P = 42;
    // let Q = 43;
    // let R = 44;
    // let S = 45;
    // let T = 46;
    // let U = 47;
    // let V = 48;
    // let W = 49;
    // let X = 50;
    // let Y = 51;
    // let Z = 52;
}
