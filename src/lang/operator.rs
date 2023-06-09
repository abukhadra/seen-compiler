// separate ar / en symbols 
pub fn prec_uni(v: &String) -> u32 {
    match v.as_str() {
        "!" | "?" | "؟" => 16,
        "-"             => 15,
        "ret" | "⏎"     => 0,
        _ => panic!()
    }
}


pub fn prec_bin(v: &String) -> u32 {
    match v.as_str() {
        "["                                 => 20,
        "("                                 => 19,
        "."                                 => 18,
        "*"  | "×" | "/" | "÷"              => 13, 
        "+"  | "-"                          => 12, 
        "<"  | "<=" | "≤" | ">" | ">=" | "≥"=> 11, 
        "==" | "!=" | "≠"                   => 10,
        "<<" | ">>"                         => 8,
        "&"  | "ࢱ"                          => 7,
        "^"  | "⊕"                          => 6,
        "|"                                 => 5,
        "&&" | "ࢱࢱ"                         => 4,
        "||"                                => 3,
        "|>"                                => 2,
        "="  | ":=" | "+=" | "-=" | "*=" | 
        "×=" | "/=" | "÷=" | "&=" | "ࢱ=" | 
        "|=" | "^=" | ">>=" | "<<="         => 1,
        _ => panic!()
    }
}

pub fn is_bin_rassoc(v: &str) -> bool {
    match v {
        "=" 
        | ":="
        | "+=" 
        | "-=" 
        | "*=" 
        | "×=" 
        | "/=" 
        | "÷=" 
        | "&=" 
        | "ࢱ=" 
        | "|=" 
        | "^=" 
        | ">>=" 
        | "<<=" => true,
        _ => false
    }
}