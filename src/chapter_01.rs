use std::collections::HashSet;

pub fn sec_1_0() {
    let s: &str = "stressed";
    let v = s.chars().rev().collect::<String>();
    println!("{}", v);
}

pub fn sec_1_1() {
    let s: &str = "パタトクカシーー";
    let v = s.chars().step_by(2).collect::<String>();
    println!("{}", v);
}

pub fn sec_1_2() {
    let p = "パトカー";
    let t = "タクシー";
    let mut result = String::new();
    let zipped = p.chars().zip(t.chars());
    zipped.for_each(|(p, t)| {
        result.push(p);
        result.push(t);
    });
    println!("{}", result);
}

pub fn sec_1_3() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let splited = s.split_whitespace();
    let v: Vec<usize> = splited
        .map(|s: &str| s.trim_matches(|c| c == ',' || c == '.').len())
        .collect();
    v.iter().for_each(|s| print!("{}", s));
}

pub fn sec_1_4() {
    let s: &str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let target: [usize; 9] = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    let parsed: Vec<String> = s
        .split_whitespace()
        .map(|st: &str| {
            st.chars()
                .filter(|c: &char| c.is_alphabetic())
                .collect::<String>()
        })
        .collect();
    let result: Vec<&str> = parsed
        .iter()
        .enumerate()
        .map(|(i, s): (usize, &String)| {
            if target.contains(&(i + 1)) {
                s.get(..1).unwrap()
            } else {
                s.get(..2).unwrap()
            }
        })
        .collect();
    result.iter().for_each(|s| {
        println!("{}", s);;
    });
}

pub fn sec_1_5_word() {
    let s = "I am an NLPer";
    let splited: Vec<&str> = s.split_whitespace().collect();
    // ここでは2分割
    let n = 2;
    let max = splited.len() - n + 1;
    let result: Vec<String> = (0..max)
        .map(|i| splited.get(i..(i + n)).unwrap().concat())
        .collect();
    result.iter().for_each(|s| {
        println!("{}", s);
    });
}

pub fn sec_1_5_char() {
    let s = "I am an NLPer";
    // ここでは2分割
    let n = 2;
    let max = s.len() - n + 1;
    let result: Vec<&str> = (0..max).map(|i| s.get(i..(i + n)).unwrap()).collect();
    result.iter().for_each(|s| {
        println!("{}", s);
    });
}

pub fn sec_1_6() {
    let pa = "paraparaparadise";
    let p = "paragraph";
    // ここでは2分割
    let n = 2;
    let max_pa = pa.len() - n + 1;
    let max_p = p.len() - n + 1;
    let x: HashSet<String> = (0..max_pa)
        .map(|i| pa.get(i..(i + n)).unwrap().to_string())
        .into_iter()
        .collect();
    let y: HashSet<String> = (0..max_p)
        .map(|i| p.get(i..(i + n)).unwrap().to_string())
        .into_iter()
        .collect();

    println!("<和集合>");
    let or = &x | &y;
    or.iter().for_each(|s| println!("{}", s));
    println!("<積集合>");
    let and = &x & &y;
    and.iter().for_each(|s| println!("{}", s));
    println!("<差集合>");
    let sub = &x - &y;
    sub.iter().for_each(|s| println!("{}", s));
}

pub fn sec_1_7() {
    let x = 12;
    let y = "気温";
    let z = 22.4;
    println!("{}時の{}は{}", x, y, z);
}

use std::char;

pub fn sec_1_8() {
    let s = "yuji toyama";
    let cipher = |s: &str| {
        s.chars()
            .map(|c: char| {
                if c.is_alphabetic() && c.is_lowercase() {
                    char::from_u32(219 - (c as u32)).unwrap()
                } else {
                    c
                }
            })
            .collect::<String>()
    };
    let decipher = |s: &str| {
        s.chars()
            .map(|c: char| {
                if c.is_alphabetic() && c.is_lowercase() {
                    char::from_u32(219 - (c as u32)).unwrap()
                } else {
                    c
                }
            })
            .collect::<String>()
    };

    let ciphered = cipher(s);
    let deciphered = decipher(&ciphered);
    println!("s: {}", s);
    println!("cihpered: {}", ciphered);
    println!("deciphered: {}", deciphered);
}

use rand::prelude::*;

pub fn sec_1_9() {
    let s = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind";
    let v: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    let end = v.len() - 1;
    let result: Vec<String> = v
        .iter()
        .enumerate()
        .map(|(i, s): (usize, &String)| match (i, s) {
            (i, s) if i == 0 => s.to_string(),
            (i, s) if i == end => s.to_string(),
            (_, s) if s.len() <= 4 => s.to_string(),
            (_, s) => {
                let mut chars: Vec<char> = s.to_string().chars().collect();
                let mut rand = rand::thread_rng();
                chars.shuffle(&mut rand);
                return chars.iter().collect::<String>();
            }
        })
        .collect();

    result
        .iter()
        .enumerate()
        .for_each(|(i, s)| println!("{}: {}", i, s));
    println!("{}", s);
}
