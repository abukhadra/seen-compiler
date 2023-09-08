use std::{
    fs,
    fmt::{
        Write
    }, path::PathBuf,
};

use crate::target::rust::rs_gen::Rust;

//================
//   bevy()
//================
// FIX HARDCODED
impl <'a> Rust<'a> {
    pub fn  bevy(
        &mut self,
    ) {
        // FIXEME, hardcoding the example for demo
        let _ = write!(self.res, 
r#"use bevy::prelude::*;

fn main() {{
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}}"#);  
        match fs::write(&self.proj_dir.src.main, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }
    }
}
