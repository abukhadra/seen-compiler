use std::{
	process::{Command, Stdio, Child},
    io::{BufReader, BufRead, Read}
};

use crate::util::fmt::*;

//================
//   cargo()
//================

pub struct Cargo {}

impl Cargo {
    //---------------------
    //  new()
    //---------------------		    
    pub fn new() -> Self {
        Self::v(); // run `cargo -v` to make sure that cargo is installed
        Self {}
    }    

    //---------------------
    //  v()
    //---------------------		    
    pub fn v() {
        Command::new("cargo")
                .arg("-V")
                .spawn()
                .expect(
                    Color::red("could not find cargo, please make sure that it's installed.")
                    .as_str()
                );
        }	    

    //---------------------
    //  build()
    //---------------------		
    pub fn build(
        &self,
        work_dir: &String,
        redirect: bool
    ) -> Option<Child> {

        self.spawn(
            "cargo",
            &["build"],
            work_dir,
            redirect
        )
    }

    //---------------------
    //  run()
    //---------------------		
    pub fn run(
        &self,
        work_dir: &String,
        cli_args: &Vec<String>,
        redirect: bool
    ) -> Option<Child> {
        let mut args = vec!["run", "--"];
        for arg in cli_args {
            args.push(arg.as_str());
        }

        let args = ["run"];     // FIXME: try comment this line out and run the bootsrap-seen-compiler project, it will give an error
                                //    for now pass only "run" as an arg and fix the cargo run -- [cli_args]  later
        self.spawn(
            "cargo",
            &args,  
            work_dir,
            redirect
        )
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
                    eprintln!("{}", Color::red("something went wrong, the command `cargo run` failed!") );
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
                println!("{}", &err.to_string());
            }
        }                            
    }

}