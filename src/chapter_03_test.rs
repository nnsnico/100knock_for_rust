#[warn(unused_imports)]
use onig::*;

#[test]
pub fn test_sec26() {
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
    let matched_group: Vec<String> = regex
        .captures_iter(&s)
        .flat_map(|cap: Captures| {
            let match_group: Vec<String> = cap
                .iter()
                .map(|v: Option<&str>| v.unwrap().to_string())
                .collect();
            match_group
        })
        .collect();

    assert_eq!(matched_group, right_group)
}

#[test]
pub fn test_sec27() {
    let s = "[[ロンドン]]";
    let right_group: Vec<&str> = vec![s, "[[", "ロンドン", "]]"];
    let regex = Regex::new(r"(.*?)('+|\[\[)(.+?)('+|\]\])(.*)").unwrap();
    let matched_group: Vec<String> = regex
        .captures_iter(&s)
        .flat_map(|cap: Captures| {
            let match_group: Vec<String> = cap
                .iter()
                .map(|v: Option<&str>| v.unwrap().to_string())
                .filter(|s| s != &"")
                .collect();
            match_group
        })
        .collect();

    assert_eq!(matched_group, right_group)
}

#[test]
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

    let ans = vec![("公式国名", "{{lang|en|United Kingdom of Great Britain and Northern Ireland}}<ref>英語以外での正式国名:<br/>
*{{lang|gd|An Rìoghachd Aonaichte na Breatainn Mhòr agus Eirinn mu Thuath}}（[[スコットランド・ゲール語]]）<br/>
*{{lang|cy|Teyrnas Gyfunol Prydain Fawr a Gogledd Iwerddon}}（[[ウェールズ語]]）<br/>
*{{lang|ga|Ríocht Aontaithe na Breataine Móire agus Tuaisceart na hÉireann}}（[[アイルランド語]]）<br/>
*{{lang|kw|An Rywvaneth Unys a Vreten Veur hag Iwerdhon Glédh}}（[[コーンウォール語]]）<br/>
*{{lang|sco|Unitit Kinrick o Great Breetain an Northren Ireland}}（[[スコットランド語]]）<br/>
**{{lang|sco|Claught Kängrick o Docht Brätain an Norlin Airlann}}、{{lang|sco|Unitet Kängdom o Great Brittain an Norlin Airlann}}（アルスター・スコットランド語）</ref>"),
("国旗画像", "Flag of the United Kingdom.svg"),
("国章画像", "[[ファイル:Royal Coat of Arms of the United Kingdom.svg|85px|イギリスの国章]]")];

    let regex_kv: Regex = Regex::new(r"(?m)^\|(.+?)\s*=\s*(.+?)(?:(?=\n\|)|(?=\n$))").unwrap();

    let target: Vec<(&str, &str)> = regex_kv
        .captures_iter(&s)
        .map(|c: Captures| {
            let k = c.at(1).unwrap();
            let v = c.at(2).unwrap();
            (k, v)
        })
        .collect();

    assert_eq!(target, ans)
}

#[test]
pub fn test_sec27_3() {
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
