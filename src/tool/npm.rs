use std::{
	process::{Command, Stdio, Child},
    io::{BufReader, BufRead, Read}
};

use crate::util::fmt::*;

//================
//   npx
//================
pub struct NPM {}

impl NPX {
    //---------------------
    //  new()
    //---------------------		    
    pub fn new() -> Self {
        Self::v(); // run `npm -v` to make sure that npx is installed
        Self {}
    }    

    //---------------------
    //  v()
    //---------------------		    
    pub fn v() {
        Command::new("npm")
                .arg("-v")
                .spawn()
                .expect(
                    Color::red("could not find npx, please make sure that it's installed.")
                    .as_str()
                );
        }	    
    //---------------------
    //  spawn()
    //---------------------		        
    fn spawn( 
        &self,
        cmd: &str,
        args: &[&str],
        work_dir: &String,
        redirect: bool
    ) -> Option<Child>{
        match Command::new(cmd)
            .args(args)
            .current_dir(&work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Err(err) => {
                    eprintln!("{}", Color::red(format!("something went wrong, cmd: `{}` , args: `{:?}` failed!", cmd, args).as_str() ));
                    None
                },
                Ok(mut cmd) => {
                    if redirect {   
                        Some(cmd)
                    } else { 
                        self.stdio(&mut cmd);
                        None
                    }
                }
        }        
    }

    //---------------------
    //  stdio()
    //---------------------		        
    fn stdio(
        &self,
        cmd: &mut Child
    ) {
        if let Some(stdout) = cmd.stdout.as_mut() {
            for line in BufReader::new(stdout).lines() {
                println!("{}", line.unwrap());
            }     
        }

        if let Some(stderr) = cmd.stderr.as_mut() {
            let mut err = String::new();
            let _ = stderr.read_to_string(&mut err);
            if err != "" {
                eprintln!("{}", &err.to_string());
            }
        }                            
    }
}

impl NPM {
    
    //---------------------
    //  npm_install()
    //---------------------		
    pub fn npm_install(
        &self,
        work_dir: &String,
        proj_name: &String,
        redirect: bool
    ) -> Option<Child> {
        let child = self.spawn(
            "npm",
            &["i"],
            work_dir,
            redirect
        );
        child
    }
}