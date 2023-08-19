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

impl Package {
    //---------------------
    //  new()
    //---------------------        
    pub fn write(
        &mut self,
        res: &mut String
    ) {
        let _ = writeln!(res, "[package]");
        let _ = writeln!(res, "name = \"{}\"", self.name);
        let _ = writeln!(res, "version = \"{}\"", self.version);
        let _ = writeln!(res, "edition = \"{}\"", self.edition);
        let _ = writeln!(res, "\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n");        
    }
}


//================
//  bin
//================  
pub struct Bin {
    pub name: String, 
    pub path: String,
}

impl Bin {
    //---------------------
    //  new()
    //---------------------       
    pub fn write(
        &mut self,
        res: &mut String
    ) {
        let _ = writeln!(res, "[[bin]]");
        let _ = writeln!(res, "name = \"{}\"", self.name);
        let _ = writeln!(res, "path = \"{}\"", self.path);
        let _ = writeln!(res, "");        
    }
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
    //---------------------
    //  new()
    //---------------------        
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
//  Deps
//================  
#[derive(Debug)]
pub struct Deps {
    items: Vec<Dep>
}

impl Deps {
    //---------------------
    //  new()
    //---------------------        
    pub fn write(
        &mut self,
        res: &mut String
    ) {
        let _ = writeln!(res, "[dependencies]");
        for dep in self.items.iter_mut() {
            dep.write(res);
        }

    }
}

//================
//  Dep
//================  
#[derive(Debug)]
pub struct Dep {
    pub id: String, 
    pub version: String,
    pub features: Option<ast::List>
}

impl Dep {
    //---------------------
    //  new()
    //---------------------        
    pub fn write(
        &mut self,
        res: &mut String
    ) {
        if let Some(features) = &self.features {
            let _ = write!(res, "{} = {{ version = \"{}\", features = [", self.id, self.version);        
            for feature in features.items.iter() {
                let _ = write!(res, "\"{}\", ", feature);    
            }
            let _ = writeln!(res, "] }}");
        } else {
            let _ = writeln!(res, "{} = \"{}\"", self.id, self.version);        
        }
    }
}

//================
//  CargoToml
//================  
pub struct CargoToml {     
    path: PathBuf,
    pub package: Package,
    pub bin: Bin,
    pub profile_rls: ProfileRls,
    pub deps: Deps,
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
        seen_conf: &Conf
    ) -> Self {
       
        let mut deps = Deps { items: vec![]};
        for el in seen_conf.data.iter() {
            match el {
                ConfElement::Rust(rs) => {
                    for rs_dep in rs.deps.iter() {
                        let dep = Dep{ 
                            id: rs_dep.id.clone(),
                            version: rs_dep.ver.clone(),
                            features: rs_dep.features.clone()
                        };
                        deps.items.push(dep);
                    }
                },
                _ => ()
            }
        }

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
            deps,
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
        self.deps.items.push(
            Dep { 
                id: _crate.id().clone(), 
                version: _crate.version().clone(),
                features: _crate.features().clone()
            }
        );
    }

}

//================
//  generate()
//================  
impl CargoToml {
    pub fn generate(&mut self) {
        self.package.write(&mut self.res);

        self.bin.write(&mut self.res);

        self.profile_rls.write(&mut self.res);

        let _ = writeln!(self.res, "");
    
        self.deps.write(&mut self.res);

        self.path.push("Cargo");
        self.path.set_extension(TOML_EXT);
    
        match fs::write(&self.path, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }        
    }
}