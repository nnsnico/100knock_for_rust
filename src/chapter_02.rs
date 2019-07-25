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

    let merge_str = col1_ls
        .iter()
        .zip(col2_ls.iter())
        .filter(|(s1, s2)| s1 != &"" && s2 != &"")
        .map(|(col1, col2): (&String, &String)| format!("{}\t{}\n", col1, col2))
        .collect::<Vec<String>>()
        .concat();

    let mut merge_file = fs::File::create("merge.txt").expect("failed to create file.");
    write!(merge_file, "{}", merge_str).expect("failed to write file");

    println!("Success!");
}

pub fn sec_2_14() {
    let n = 5;

    let mut file = fs::File::open("hightemp.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let vec = s
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let taked_vec: Vec<&String> = vec.iter().take(n).collect::<Vec<&String>>();

    taked_vec.iter().for_each(|s| println!("{}", s));
}

pub fn sec_2_15() {
    let n = 1;

    let mut file = fs::File::open("hightemp.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let vec = s
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let taked_vec: Vec<&String> = vec.iter().rev().take(n).collect::<Vec<&String>>();

    taked_vec.iter().rev().for_each(|s| println!("{}", s));
}

pub fn sec_2_16() {
    let n = 1;

    let mut file = fs::File::open("hightemp.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let vec = s
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let splited_vec = vec
        .chunks(n)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<String>>>();

    splited_vec.iter().enumerate().for_each(|(i, v)| {
        let cmp_str = v.iter().map(|s| format!("{}\n", s)).collect::<String>();
        let mut f = fs::File::create(format!("out{}.txt", i)).expect("failed to create file.");
        write!(f, "{}", cmp_str).expect("failed to write file.");
    });

    println!("Success!");
}

use std::collections::HashSet;

pub fn sec_2_17() {
    let mut file = fs::File::open("col1.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let set = s
        .split("\n")
        .filter(|s| !s.to_string().is_empty())
        .collect::<HashSet<&str>>();

    set.iter().for_each(|s| println!("{}", s));
}

pub fn sec_2_18() {
    let mut file = fs::File::open("hightemp.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let cols: Vec<&str> = s.split("\n").filter(|s| s.to_string() != "").collect();
    let mut matrix: Vec<Vec<&str>> = cols
        .iter()
        .map(|col| col.split("\t").collect::<Vec<&str>>())
        .collect();
    matrix.sort_unstable_by(|_col: &Vec<&str>, col: &Vec<&str>| {
        let parsed_col: f32 = _col[2].parse().unwrap();
        let parsed_col2: f32 = col[2].parse().unwrap();
        parsed_col.partial_cmp(&parsed_col2).unwrap()
    });

    let merge: Vec<String> = matrix
        .iter()
        .cloned()
        .map(|v: Vec<&str>| {
            let str_vec: Vec<String> = v.iter().map(|s| s.to_string()).collect();
            str_vec.join("\t")
        })
        .collect();

    merge.iter().for_each(|s| println!("{}", s));
}

pub fn sec_2_19() {
    let mut file = fs::File::open("col1.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let col1: Vec<&str> = s
        .split("\n")
        .filter(|s| s.to_string() != "")
        .map(|s: &str| s.split("\t").nth(0).unwrap())
        .collect();

    let count: HashMap<&str, usize> =
        col1.iter()
            .cloned()
            .fold(HashMap::new(), |mut acc: HashMap<&str, usize>, s: &str| {
                if acc.contains_key(s) {
                    if let Some(x) = acc.get_mut(s) {
                        *x += 1;
                    };
                    acc
                } else {
                    acc.insert(s, 1);
                    acc
                }
            });
    let mut count_vec: Vec<(&str, usize)> = count.iter().map(|(a, b)| (*a, *b)).collect();
    count_vec.sort_by(|(_, b1), (_, b2)| b2.partial_cmp(b1).unwrap());

    count_vec
        .iter()
        .for_each(|(s, i)| println!("   {} {}", i, s));
}

use std::collections::HashMap;

pub fn sec_ext_map() {
    let mut file = fs::File::open("hightemp.txt").expect("failed to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file.");

    let cols = s
        .split("\n")
        .filter(|s| s.to_string() != "")
        .collect::<Vec<&str>>();
    let keys = vec!["pref", "city", "temp", "date"];
    let map = cols
        .iter()
        .cloned()
        .map(|col: &str| {
            let values = col.split("\t").collect::<Vec<&str>>();
            keys.iter()
                .cloned()
                .zip(values.iter().cloned())
                .collect::<HashMap<&str, &str>>()
        })
        .collect::<Vec<HashMap<&str, &str>>>();

    map.iter().for_each(|s| println!("{:?}", s));
}
