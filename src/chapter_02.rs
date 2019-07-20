use std::fs;
use std::io::prelude::*;

pub fn sec_2_10() {
    let mut f = fs::File::open("hightemp.txt").expect("file not found.");
    let mut s = String::new();

    f.read_to_string(&mut s).expect("failed reading the flie.");

    let line_count: u32 = s
        .split("\n")
        .map(|s| if s == "" { 0 } else { 1 })
        .fold(0, |sum, i| sum + i);
    println!("{}", line_count);
}

pub fn sec_2_11() {
    let mut f = fs::File::open("hightemp.txt").expect("file not found.");
    let mut s = String::new();

    f.read_to_string(&mut s).expect("failed reading the flie.");

    println!("{}", s.replace("\t", " "));
}

pub fn sec_2_12() {
    let mut f = fs::File::open("hightemp.txt").expect("file not found.");
    let mut s = String::new();

    f.read_to_string(&mut s).expect("failed reading the flie.");

    let splited: Vec<String> = s
        .split(|s| s == '\t' || s == '\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let col1: Vec<&String> = splited
        .iter()
        .enumerate()
        .filter(|(i, _)| (i % 4) == 0)
        .map(|(_, s)| s)
        .collect();
    let col2: Vec<&String> = splited
        .iter()
        .enumerate()
        .filter(|(i, _)| (i % 4) == 1)
        .map(|(_, s)| s)
        .collect();

    let mut col1_file = fs::File::create("col1.txt").expect("failed to create file.");
    let mut col2_file = fs::File::create("col2.txt").expect("failed to create file.");
    col1.iter()
        .for_each(|s| write!(col1_file, "{}\n", s).expect("failed to write file!"));
    col2.iter()
        .for_each(|s| write!(col2_file, "{}\n", s).expect("failed to write file!"));

    println!("Success!!");
}
