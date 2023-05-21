use std::arch::x86_64::_mm_set1_ps;
use std::fs;

#[derive(Debug)]
struct Pairs {
    l1: i32,
    l2: i32,
    r1: i32,
    r2: i32 }

#[derive(Debug)]
struct VectorOfPairs(Vec<Pairs>);


// 17-99,18-24

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let inp = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let mut vector_of_pairs = vec![];

    for line in input.lines(){
        let split: Vec<&str> =  line.split(",").collect();
        let mut iter = split.clone().into_iter();
        let split_left: Vec<&str> = iter.next().unwrap().split("-").collect();
        let split_right: Vec<&str> = iter.next().unwrap().split("-").collect();

        let pair = Pairs{
            l1: split_left[0].parse().unwrap(),
            l2: split_left[1].parse().unwrap(),
            r1: split_right[0].parse().unwrap(),
            r2: split_right[1].parse().unwrap()
        };

        
        vector_of_pairs.push(pair);

    }

    let mut counter = 0;
    for pair in &vector_of_pairs{
        if is_subset(pair.l1..=pair.l2, pair.r1..=pair.r2) {
            counter += 1;

        }
    }
    println!("{}", counter);


}


fn is_subset(set1: std::ops::RangeInclusive<i32>, set2: std::ops::RangeInclusive<i32>) -> bool {

    let mut setl = vec![];

    for num in set1.clone() {
        setl.push(num);
    }

    let mut setr = vec![];

    for num in set2.clone() {
        setr.push(num);
    }

    if setl[0] <= setr[0] && setl[setl.len()-1] >= setr[setr.len()-1] {
        return true; 
    }
    if setr[0] <= setl[0] && setr[setr.len()-1] >= setl[setl.len()-1] {
        return true; 
    }

    for num in set1.clone() {
        if set2.contains(&num) {
            return true;
        }
    }

    for num in set2 {
        if set1.contains(&num) {
            return true;
        }
    }
    false

}
    



