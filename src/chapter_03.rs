extern crate flate2;

use flate2::read::GzDecoder;
use onig::{Captures, Regex};
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
        .map(|captures| captures.at(1).unwrap())
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
        .map(|captures| captures.at(0).unwrap())
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
        .map(|media| media.at(2).unwrap())
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
    let regex_basic_info: Regex = Regex::new(r"(?m)^\{\{基礎情報.*?\n(.*?)\n\}\}$").unwrap();
    let regex_kv: Regex = Regex::new(r"(?m)^\|(.+?)\s*=\s*(.+?)(?:(?=\n\|)|(?=\n$))").unwrap();
    let basic_info: String = regex_basic_info
        .captures_iter(&uk.text)
        .map(|cap: Captures| cap.at(1).unwrap())
        .collect();
    let parsed_kv: HashMap<&str, &str> = regex_kv
        .captures_iter(&basic_info)
        .map(|cap| {
            let k = cap.at(1).unwrap();
            let v = cap.at(2).unwrap();
            (k, v)
        })
        .collect();
    parsed_kv.iter().for_each(|(k, v)| println!("{}: {}", k, v));
}

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
    let regex_highlight = Regex::new(r"(.+?)('+)(.+?)('+)(.+)").unwrap();
    let basic_info: HashMap<&str, &str> = regex
        .captures_iter(&uk.text)
        .map(|info| {
            let k = info.at(1).unwrap();
            let v = info.at(4).unwrap();
            (k, v)
        })
        .collect();

    let info_without_highlight: HashMap<&str, String> = basic_info
        .iter()
        .map(|(k, v)| {
            let highlight = regex_highlight.replace(v, |cap: &Captures| {
                format!(
                    "{}{}{}",
                    &cap.at(1).unwrap(),
                    &cap.at(3).unwrap(),
                    &cap.at(5).unwrap()
                )
            });
            (*k, highlight.chars().collect::<String>())
        })
        .collect();

    info_without_highlight
        .iter()
        .for_each(|s| println!("{} {}", s.0, s.1));
}

pub fn sec_3_27() {
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
    let regex = Regex::new(r"\|(.+)(\s+)=(\s+)([^(?=\n\|)])]").unwrap();
    let regex_highlight_or_link = Regex::new(r"(.*?)('+|\[+)(.+?)('+|\]+)(.*)").unwrap();
    let basic_info: HashMap<&str, &str> = regex
        .captures_iter(&uk.text)
        .map(|info| {
            println!("{:?}", info);
            let k = info.at(1).unwrap();
            let v = info.at(4).unwrap();
            (k, v)
        })
        .collect();

    let info_without_highlight: HashMap<&str, String> = basic_info
        .iter()
        .map(|(k, v)| {
            let highlight = regex_highlight_or_link.replace(v, |cap: &Captures| {
                format!(
                    "{}{}{}",
                    &cap.at(1).unwrap(),
                    &cap.at(3).unwrap(),
                    &cap.at(5).unwrap()
                )
            });
            (*k, highlight.chars().collect::<String>())
        })
        .collect();

    info_without_highlight
        .iter()
        .for_each(|s| println!("{} {:?}", s.0, s.1));
}

#[test]
pub fn test_sec26() {
    use regex::Match;

    let s =
        "現在の国号「'''グレートブリテン及び北アイルランド連合王国'''」に変更";
    let right_group: Vec<&str> = vec![
        s,
        "現在の国号「",
        "'''",
        "グレートブリテン及び北アイルランド連合王国",
        "'''",
        "」に変更",
    ];
    let regex = Regex::new(r"(.*?)('+)(.+?)('+)(.*)").unwrap();
    let matched_group: Vec<&str> = regex
        .captures_iter(&s)
        .flat_map(|cap: Captures| {
            let match_group: Vec<&str> = cap
                .iter()
                .map(|v: Option<Match>| v.unwrap().as_str())
                .collect();
            match_group
        })
        .collect();

    assert_eq!(matched_group, right_group)
}

#[test]
pub fn test_sec27() {
    use regex::Match;

    let s = "[[ロンドン]]";
    let right_group: Vec<&str> = vec![s, "[[", "ロンドン", "]]"];
    let regex = Regex::new(r"(.*?)('+|\[\[)(.+?)('+|\]\])(.*)").unwrap();
    let matched_group: Vec<&str> = regex
        .captures_iter(&s)
        .flat_map(|cap: Captures| {
            let match_group: Vec<&str> = cap
                .iter()
                .map(|v: Option<Match>| v.unwrap().as_str())
                .filter(|s| s != &"")
                .collect();
            match_group
        })
        .collect();

    assert_eq!(matched_group, right_group)
}

pub fn test_sec27_2() {
    let s = r"
|公式国名 = {{lang|en|United Kingdom of Great Britain and Northern Ireland}}<ref>英語以外での正式国名:<br/>
*{{lang|gd|An Rìoghachd Aonaichte na Breatainn Mhòr agus Eirinn mu Thuath}}（[[スコットランド・ゲール語]]）<br/>
*{{lang|cy|Teyrnas Gyfunol Prydain Fawr a Gogledd Iwerddon}}（[[ウェールズ語]]）<br/>
*{{lang|ga|Ríocht Aontaithe na Breataine Móire agus Tuaisceart na hÉireann}}（[[アイルランド語]]）<br/>
*{{lang|kw|An Rywvaneth Unys a Vreten Veur hag Iwerdhon Glédh}}（[[コーンウォール語]]）<br/>
*{{lang|sco|Unitit Kinrick o Great Breetain an Northren Ireland}}（[[スコットランド語]]）<br/>
**{{lang|sco|Claught Kängrick o Docht Brätain an Norlin Airlann}}、{{lang|sco|Unitet Kängdom o Great Brittain an Norlin Airlann}}（アルスター・スコットランド語）</ref>
|国旗画像 = Flag of the United Kingdom.svg
|国章画像 = [[ファイル:Royal Coat of Arms of the United Kingdom.svg|85px|イギリスの国章]]
";

    let regex_kv: Regex = Regex::new(r"(?m)^\|(.+?)\s*=\s*(.+?)(?:(?=\n\|)|(?=\n$))").unwrap();

    regex_kv.captures_iter(&s).for_each(|c: Captures| {
        let k = c.at(1).unwrap();
        let v = c.at(2).unwrap();
        println!("{}: {}", k, v);
    });
}
