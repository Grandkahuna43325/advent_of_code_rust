#![allow(unused_variables, dead_code)]
use std::fmt::Pointer;
use std::fs;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<*mut File>,
    dir: Vec<String>,
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    // let mut count = 0;
    let mut dirs = vec![];


    let mut name = String::new();
    let mut files: Vec<*mut File> = vec![];
    let mut dir: Vec<String> = vec![];

    for line in input.lines() {
        name = "".to_string();
        files = vec![];
        dir = vec![];
        match line {
            line if line.starts_with("$ cd ..") => {
                // println!("changing back");
            }
            line if line.starts_with("$ cd ") => {
                let name_ = line.split(" ").nth(2).unwrap();
                // println!("changing to: {name_}");
                name = name_.to_string();
            }
            line if line.starts_with("$ ls") => {
                // println!("")
            }
            line if line.starts_with("dir ") => {
                // println!("dir name: {line}");
                let dir_ = line.split(" ").nth(1).unwrap().to_string();
                dir.push(dir_);
            }
            line => {
                let size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                let name = line.split(" ").nth(1).unwrap().to_string();
                let mut file = File { name, size };
                let ptr = &mut file as *mut File;
                files.push(ptr)
            }
        }
        dirs.push(Directory { name: name.clone(), files: files.clone(), dir: dir.clone() });
        unsafe { 
        match files.iter().next() {
            Some(x) => {if !x.is_null(){ println!("{:#?}", (*(*x)))}},
            None => {}
        }}
        // println!("{:?}\nfiles: {:?},{:?}", name, files.iter().next().unwrap(), dir);
    }
    println!("{:?}", dirs.iter().nth(19).unwrap().name);
}
