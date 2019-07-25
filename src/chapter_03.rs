extern crate flate2;

use flate2::read::GzDecoder;
use regex::Regex;
use serde::Deserialize;
use serde_json::Deserializer;
use std::fs;

#[derive(Deserialize, Debug)]
struct Wiki {
    pub title: String,
    pub text: String,
}

pub fn sec_3_20() {
    let uk: Option<Wiki> = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        });

    uk.iter()
        .for_each(|s| println!("title: {}\ntext: {}", s.title, s.text));
}

pub fn sec_3_21() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"\[\[Category.*\]\]").unwrap();
    let categories: Vec<&str> = uk.text.lines().filter(|l| regex.is_match(l)).collect();

    categories.iter().for_each(|l| println!("{}", l));
}

pub fn sec_3_22() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"\[\[Category:([^|\n]*)\|?.*\]\]").unwrap();
    let categories: Vec<&str> = regex
        .captures_iter(&uk.text)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect();
    categories.iter().for_each(|l| println!("{:?}", l));
}

pub fn sec_3_23() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"(=+?).*(=+?)\n").unwrap();
    let sections: Vec<(&str, usize)> = regex
        .captures_iter(&uk.text)
        .map(|captures| captures.get(0).unwrap().as_str())
        .map(|l: &str| {
            let count = l.to_string().chars().filter(|c| c == &'=').count() / 2 - 1;
            (l, count)
        })
        .collect();

    sections
        .iter()
        .for_each(|(l, i)| println!("{} {}", i, l.to_string().trim_end_matches("\n")));
}

pub fn sec_3_24() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"\[\[(ファイル|File):([^|\n]*)\|?.*\]\]").unwrap();
    let media_files: Vec<&str> = regex
        .captures_iter(&uk.text)
        .map(|media| media.get(2).unwrap().as_str())
        .collect();

    media_files.iter().for_each(|l| println!("{}", l));
}

use std::collections::HashMap;

pub fn sec_3_25() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"\|(.+)(\s+)=(\s+)(.+)\n").unwrap();
    let basic_info: HashMap<&str, &str> = regex
        .captures_iter(&uk.text)
        .map(|info| {
            let k = info.get(1).unwrap().as_str();
            let v = info.get(4).unwrap().as_str();
            (k, v)
        })
        .collect::<HashMap<&str, &str>>();

    basic_info
        .iter()
        .for_each(|(k, v)| println!("{}: {}", k, v));
}

use regex::Captures;

pub fn sec_3_26() {
    let uk: Wiki = fs::File::open("jawiki-country.json.gz")
        .ok()
        .and_then(|file: fs::File| Some(GzDecoder::new(file)))
        .and_then(|gz: GzDecoder<fs::File>| {
            Deserializer::from_reader(gz)
                .into_iter::<Wiki>()
                .filter(|v| {
                    if let Ok(v) = v {
                        v.title == "イギリス"
                    } else {
                        false
                    }
                })
                .map(|wiki| wiki.unwrap())
                .next()
        })
        .unwrap();
    let regex = Regex::new(r"\|(.+)(\s+)=(\s+)(.+)\n").unwrap();
    let regex_highlight = Regex::new("(\'*).+(\'*)").unwrap();
    let basic_info: HashMap<&str, &str> = regex
        .captures_iter(&uk.text)
        .map(|info| {
            let k = info.get(1).unwrap().as_str();
            let v = info.get(4).unwrap().as_str();
            (k, v)
        })
        .collect();

    let info_without_highlight: HashMap<&str, String> = basic_info
        .iter()
        .map(|(k, v)| {
            let highlight = regex_highlight.replace(v, |cap: &Captures| format!("{}", &cap[1]));
            (*k, highlight.chars().collect::<String>())
        })
        .collect();

    info_without_highlight
        .iter()
        .for_each(|(t1, t2): (&&str, &String)| println!("{}   {}", t1, t2));
}
