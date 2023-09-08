use std::{
    fs,
    process
};

//================
//   Alias
//================
pub type ScriptPath = String;

//================
//   Script
//================
#[derive(Debug)]
pub struct Script {
    pub path: String,
    pub content: String,

}

impl Script {
    //---------------------
    //  from_str
    //---------------------        
    pub fn from_str(
        content: &str
    )  -> Self  {
        Self {
            path: String::new(),
            content: content.to_string()
        }

    }

    //---------------------
    //  from_file
    //---------------------        
    pub fn from_file(
        path: &String
    )  -> Self  {
        let content = fs::read_to_string(path);
        let content = match content {
            Ok(content) => content,
            Err(e) => {
                eprintln!("error : {} - {}", path, e);
                process::exit(1);   
            }
        };  

        Self {
            path: path.clone(),
            content
        }

    }

}
