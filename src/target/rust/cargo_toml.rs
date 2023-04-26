use std::{
    fs,
    path::PathBuf,
    fmt::{
        Write,
    }
};

use crate::util::{
    indent::Indent    
};

use super::rs_crate::Crate;


//================
//   Constants
//================
const TOML_EXT: &'static str = "toml";

//================
//  Package
//================  
pub struct Package {
    pub name: String, 
    pub version: String,
    pub edition: String    
}


//================
//  bin
//================  
pub struct Bin {
    pub name: String, 
    pub path: String,
}

//================
//  ProfileRelease
//================  
pub struct ProfileRls {
    pub lto: String, 
    pub opt_level: String,
    pub strip: String
}

impl ProfileRls {
    pub fn write(
        &mut self,
        res: &mut String
    ) {
        let _ = writeln!(res, "[profile.release]");
        let _ = writeln!(res, "lto = {}", self.lto);
        let _ = writeln!(res, "opt-level = {}", self.opt_level);
        let _ = writeln!(res, "strip = {}", self.strip);        
    }
}


//================
//  Dep
//================  
pub struct Dep {
    pub id: String, 
    pub version: String    
}

//================
//  CargoToml
//================  
pub struct CargoToml {     
    path: PathBuf,
    pub package: Package,
    pub bin: Bin,
    pub profile_rls: ProfileRls,
    pub deps: Vec<Dep>,
    indent: Indent,
    res: String    
}

impl CargoToml {     
    //---------------------
    //  new()
    //---------------------    
    pub fn new(
        name: &str,
        path: &PathBuf,    
    ) -> Self {
        Self {
            path: path.clone(),
            package: Package {   // FIXME hardcoded
                name: name.to_string(),
                version: "0.1.0".to_string(),
                edition: "2021".to_string()
            },
            bin: Bin {
                name: name.to_string(),
                path: "src/main.rs".to_string()
            },
            profile_rls: ProfileRls { 
                lto: "true".to_string(), 
                opt_level: "1".to_string(), 
                strip: "true".to_string() 
            },
            deps: vec![],
            indent: Indent::new(),
            res: String::new()
        }
    }

    //---------------------
    //  add()
    //---------------------   
    pub fn add(
        &mut self,
        _crate: impl Crate
    )  {
        self.deps.push(
            Dep { 
                id: _crate.id().clone(), 
                version: _crate.version().clone()
            }
        );
    }

}

//================
//  generate()
//================  
impl CargoToml {
    pub fn generate(&mut self) {
        let _ = writeln!(self.res, "[package]");
        let _ = writeln!(self.res, "name = \"{}\"", self.package.name);
        let _ = writeln!(self.res, "version = \"{}\"", self.package.version);
        let _ = writeln!(self.res, "edition = \"{}\"", self.package.edition);
        let _ = writeln!(self.res, "\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n");

        let _ = writeln!(self.res, "[[bin]]");
        let _ = writeln!(self.res, "name = \"{}\"", self.bin.name);
        let _ = writeln!(self.res, "path = \"{}\"", self.bin.path);
        let _ = writeln!(self.res, "");

        self.profile_rls.write(&mut self.res);

        let _ = writeln!(self.res, "");
    

        let _ = writeln!(self.res, "[dependencies]");
        for dep in self.deps.iter() {
            let _ = writeln!(self.res, "{} = \"{}\"", dep.id, dep.version);        
        }

        self.path.push("Cargo");
        self.path.set_extension(TOML_EXT);
    
        match fs::write(&self.path, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }        
    }
}