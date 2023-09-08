use std::{
    fs,
    path::PathBuf,
    fmt::Write,
};

use crate::{
    util::indent::Indent, 
    lang::{syntax_tree::ast::{self, ModElement}, compiler::Data}, project::conf::{SeenDep, Conf, ConfElement}
};

use super::rs_crate::Crate;
use super::rs_features::CrateFeatures;


//================
//   Constants
//================
const RS_EXT: &'static str = "rs";


//================
//  BuildRs
//================  
pub struct BuildRs {     
    path: PathBuf,
    src: String,  // FIXME, for now just hardcode all build.rs files for the demo
}

impl BuildRs {     
    //---------------------
    //  new()
    //---------------------    
    pub fn new(
        path: &PathBuf,    
        src: String
    ) -> Self {
       
        Self {
            path: path.clone(),
            src,        
        }
    }
}

//================
//  generate()
//================  
impl BuildRs {
    pub fn generate(&mut self) {
        self.path.push("build");
        self.path.set_extension(RS_EXT);
    
        match fs::write(&self.path, &self.src){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }        
    }
}