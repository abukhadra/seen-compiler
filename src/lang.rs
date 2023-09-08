#![allow(warnings)]

pub mod script;
pub mod token;
pub mod operator;
pub mod lexer;
pub mod syntax_tree;
pub mod symtab;
pub mod parser;
pub mod resolver;
pub mod inference;
pub mod type_checker;
pub mod compiler;
pub mod error;

use std::fmt;

use serde::Serialize;

//================
//   Constants
//================
const SEEN_EN_EXT: &str = "seen";
const SEEN_AR_EXT: &str = "س";

//================
//  Lang
//================
#[derive(Clone, Debug, Serialize)]
pub enum Lang {
    Ar,
    En
}

impl Lang {
    //---------------------
    //  from_str()
    //---------------------    
    pub fn from_str(v: &str) -> Lang {
        match v {
            "ar" => Lang::Ar,
            "en" => Lang::En,
            _ => panic!("invalid language string!")
        }
    }


    //---------------------
    //  as_str()
    //---------------------    
    pub fn as_str(&self) -> &str {
        match &self {
            Lang::Ar => "ar" ,
            Lang::En => "en",
        }
    }

    //---------------------
    //  ext()
    //---------------------    
    pub fn ext(&self) -> &'static str {
        match self {
            Lang::Ar => SEEN_AR_EXT,
            Lang::En => SEEN_EN_EXT
        }
    }

    //---------------------
    //  lang_from_ext()
    //---------------------    
    pub fn lang_from_ext(path: &str) -> Self {
        if path.ends_with(SEEN_EN_EXT) {
            Self::En
        } else if path.ends_with(SEEN_AR_EXT) {
            Self::Ar
        } else {
            panic!("unknown seen file format!");
        }
    }

    //---------------------
    //  is_ar_ext()
    //---------------------        
    pub fn is_ar_ext() {

    }
}


//================
//   Display Lang
//================
impl fmt::Display for Lang {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ar => write!(f, "ar"), 
            Self::En => write!(f, "en"), 
        }
    }
}
