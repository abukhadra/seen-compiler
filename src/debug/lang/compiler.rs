use std::{
    fmt::{
        Write,
    }, 
};   

use crate::lang::{
    compiler::{
        Modules
    },
    error::{
        self,
    }        
    
};

//================
//  debug_tokens()
//================
#[cfg(debug_assertions)]
pub fn debug_tokens(modules: &Modules) -> String {
    let mut result = String::from("");
    
    for (path, data) in modules {
        if data.errors.is_empty() {
            let _ = writeln!(
                result, 
                "tokens in file  `{}` : \n", 
                path, 
            );
            data.tokens
                .as_ref()
                .unwrap()
                .iter()
                .for_each(
                    |t| {
                        let _ = writeln!(result, "{:?}", t);
                    }
                )
        } 
    }
    result
}

//================
//  debug_ast()
//================
#[cfg(debug_assertions)]
pub fn debug_ast(modules: &Modules) -> String {
    let mut result = String::from("");
    
    for (path, data) in modules {
        if data.errors.is_empty() {
            let _ = writeln!(
                result, 
                "ast for file `{}`: \n", 
                path, 
            );

            data.ast
                .as_ref()
                .unwrap()
                .iter()
                .for_each(
                    |t| {
                        let _ = writeln!(result, "{:?}", t);
                    }
                )
        } 
    }
    result
}

//================
//  debug_resolver()
//================
#[cfg(debug_assertions)]
pub fn debug_resolver(modules: &Modules) -> String {
    let mut result = String::from("");
    for (path, data) in modules {
        if data.errors.is_empty() {
            let _ = writeln!(
                result, 
                "symtab for file `{}`: \n", 
                path, 
            );

            let _ = writeln!(
                result, 
                "{:?}", 

                data.symtab
                    .as_ref()
                    .unwrap()
            );
        } 
    }
    result
}


//================
//  debug_inference()
//================
#[cfg(debug_assertions)]
pub fn debug_inference(modules: &Modules) -> String {
    let mut result = String::from("");
    for (path, data) in modules {
        if data.errors.is_empty() {
            let _ = writeln!(
                result, 
                "ast after inference for file `{}`: \n", 
                path, 
            );

            let _ = writeln!(
                result, 
                "{:?}", 

                data.ast
                    .as_ref()
                    .unwrap()
            );
        } 
    }
    result
}

//================
//  debug_generated_src()
//================
#[cfg(debug_assertions)]
pub fn debug_generated_src(path: &String) -> String {
    match std::fs::read_to_string(path){
        Err(err) => panic!("{}", err),
        Ok(src) => format!("{} :\n{}", path, src)
    }
    
}

//================
//  print_errors()
//================
pub fn print_errors(modules: &Modules) { 
    for data in modules.values() {
        if !data.errors.is_empty() {
            error::print_errors(
                &data.errors, 
                &data.script
            );
        }    
    }
}