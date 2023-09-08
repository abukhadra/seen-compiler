#![allow(warnings)]
pub mod proj_dir;
pub mod conf;
pub mod src;
pub mod build;
pub mod templates;

use std::path::PathBuf;

use crate::transl::transl::Transl;
use crate::lang::Lang;


use conf::{
    Conf, 
    ConfElement, 
    Target
};




//================
//   ProjSettings()
//================
pub struct ProjSettings {
    pub home: PathBuf, 
    pub seen_conf: Conf, 
    pub lang: Lang, 
    pub transl: Transl, 
    pub proj_name: String, 
    pub target: String, 
    pub redirect: bool
}

impl ProjSettings {
    pub fn new(
        path: Option<PathBuf>,
        redirect: bool
    ) -> Self {
        let home = match path {
            None => std::env::current_dir().unwrap(),
            Some(path) => path
        };
    
        let seen_conf = Conf::new(&home);
        let lang = conf::proj_lang(&home).expect("");
        let transl = Transl::new(&lang);
        let proj_name = conf::proj_name(&transl, &home);
    
        let mut target = String::new();
        for el in seen_conf.data.iter() {
            if let ConfElement::Target(Target(_target))= el {
                target = _target.to_owned();
                break;
            } 
        };
    
        Self {
            home,
            seen_conf,
            lang,
            transl,
            proj_name,
            target,
            redirect

        }
    }
}