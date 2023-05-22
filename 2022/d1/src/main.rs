#![allow(dead_code)]
use std::fs;

#[derive(Debug)]
struct Elf {
    elf_name: usize,
    counter: usize
}

#[derive(Debug)]
struct AllElfs(Vec<Elf>);


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut elfs = AllElfs(vec![]);
    let mut name = 0;
    let mut counter = 0;

    for line in input.lines()  {
        if line == "" {
            elfs.0.push(Elf{elf_name: name, counter});
            name += 1;
            counter = 0;
        }else {
            let number_in_line: usize = line.parse::<usize>().unwrap();
            counter += number_in_line;
        }
    }

    elfs.0.sort_by(|a, b| b.counter.cmp(&a.counter));

    let mut x = elfs.0.iter();
    let best = x.next().unwrap().counter;
    let second = x.next().unwrap().counter;
    let third = x.next().unwrap().counter;

    println!("Best: {}\nSecond: {}\nThird:{}\nCombined: {}",best, second, third, best + second + third);
}
