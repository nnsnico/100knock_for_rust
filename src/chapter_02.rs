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

pub fn sec_2_13() {
    let mut col1 = fs::File::open("col1.txt").expect("file not found.");
    let mut col2 = fs::File::open("col2.txt").expect("file not found.");
    let mut col1_str = String::new();
    let mut col2_str = String::new();

    col1.read_to_string(&mut col1_str)
        .expect("failed reading the flie.");
    col2.read_to_string(&mut col2_str)
        .expect("failed reading the flie.");

    let col1_ls: Vec<String> = col1_str.split('\n').map(|s| s.to_string()).collect();
    let col2_ls: Vec<String> = col2_str.split('\n').map(|s| s.to_string()).collect();

    // TODO: 最後の改行がムダ
    let merge_str = col1_ls
        .iter()
        .zip(col2_ls.iter())
        .filter(|(s1, s2)| s1 != &"" && s2 != &"")
        .map(|(col1, col2): (&String, &String)| {
            [col1.to_string(), col2.to_string()].join("\t") + "\n"
        })
        .collect::<Vec<String>>()
        .concat();

    println!("{}", merge_str);
}

pub fn sec_2_14() {}
