use std::fs;

#[derive(Debug)]
struct Pairs {
    left_1: i32,
    left_2: i32,
    right_1: i32,
    right_2: i32,
}

#[derive(Debug)]
struct VectorOfPairs(Vec<Pairs>);

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut vector_of_pairs = vec![];

    for line in input.lines() {
        let mut split = line.split(",");

        let split_left: Vec<&str> = split.next().unwrap().split("-").collect();
        let split_right: Vec<&str> = split.next().unwrap().split("-").collect();

        vector_of_pairs.push(Pairs {
            left_1: split_left[0].parse().unwrap(),
            left_2: split_left[1].parse().unwrap(),
            right_1: split_right[0].parse().unwrap(),
            right_2: split_right[1].parse().unwrap(),
        });
    }

    let mut counter = 0;
    for pair in &vector_of_pairs {
        if is_subset(pair.left_1..=pair.left_2, pair.right_1..=pair.right_2) {
            counter += 1;
        }
    }
    println!("Part 1: {}", counter);

    let mut counter = 0;
    for pair in &vector_of_pairs {
        if is_subset_inclusive(pair.left_1..=pair.left_2, pair.right_1..=pair.right_2) {
            counter += 1;
        }
    }
    println!("Part 2: {}", counter);
}

fn is_subset_inclusive(
    set1: std::ops::RangeInclusive<i32>,
    set2: std::ops::RangeInclusive<i32>,
) -> bool {
    for num in set1.clone() {
        if set2.contains(&num) {
            return true;
        }
    }
    false
}

fn is_subset(
    set1: std::ops::RangeInclusive<i32>,
    set2: std::ops::RangeInclusive<i32>
) -> bool {
    if set1.clone().sum::<i32>() > set2.clone().sum::<i32>() {
        for i in set2 {
            if !set1.contains(&i) {
                return false;
            }
        }
    } else {
        for i in set1 {
            if !set2.contains(&i) {
                return false;
            }
        }
    }
    true
}
