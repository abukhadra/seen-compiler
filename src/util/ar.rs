use std::{
    fmt::{
        Write,
    }
};

//================
//   is_western_num()
//================
pub fn is_western_num(num : &String) -> bool {
    match num.chars().next().unwrap() {
        '0'..='9' => true,
        _ => false
    }
}

//================
//   to_western_num()
//================
pub fn to_western_num(num : &String) -> String {
    
    if !is_western_num(&num) {     
        let mut res = String::new();
        for c in num.chars() {
            let _ = match c {
                '٠' => write!(res, "0"),
                '١' => write!(res, "1"),
                '٢' => write!(res, "2"),
                '٣' => write!(res, "3"),
                '٤' => write!(res, "4"),
                '٥' => write!(res, "5"),
                '٦' => write!(res, "6"),
                '٧' => write!(res, "7"),
                '٨' => write!(res, "8"),
                '٩' => write!(res, "9"),
                '٫' => write!(res, "."),
                _ => panic!()
            };
        }
        res
    } else {
        num.clone()
    }
}

//================
//   to_western_digit()
//================
pub fn to_western_digit(c : char) -> char {
    match c {
        '٠' => '0',
        '١' => '1',
        '٢' => '2',
        '٣' => '3',
        '٤' => '4',
        '٥' => '5',
        '٦' => '6',
        '٧' => '7',
        '٨' => '8',
        '٩' => '9',
        _ => panic!()
    }
}



