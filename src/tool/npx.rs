use std::{
	process::{Command, Stdio, Child},
    io::{BufReader, BufRead, Read}
};

use crate::util::fmt::*;

//================
//   npx
//================
pub struct NPX {}

impl NPX {
    //---------------------
    //  new()
    //---------------------		    
    pub fn new() -> Self {
        Self::v(); // run `npx -v` to make sure that npx is installed
        Self {}
    }    

    //---------------------
    //  v()
    //---------------------		    
    pub fn v() {
        Command::new("npx")
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
                println!("{}", &err.to_string());
            }
        }                            
    }
}

//================
//   npx react-native
//================
impl NPX {
    
    //---------------------
    //  react_native_init()
    //---------------------		
    pub fn react_native_init(
        &self,
        work_dir: &String,
        proj_name: &String,
        redirect: bool
    // ) -> Option<Child> {
    ) {
        // // println!("workdir: {}", work_dir);
        // let child = self.spawn(
        //     "npx",
        //     &["react-native@latest", "init", proj_name],
        //     // &["--help" ],
        //     // &["react-native@latest", "init", proj_name ],
        //     // &[format!("react-native@latest init {}", proj_name).as_str()],
        //     work_dir,
        //     redirect
        // );
        // child


        let cmd = "npx";
        let args = &["react-native@latest", "init", proj_name];
        match Command::new(cmd)
            .args(args)
            .current_dir(&work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            // .spawn()
            .status() {
                Err(err) => {
                    eprintln!("{}", Color::red(format!("something went wrong, cmd: `{}` , args: `{:?}` failed!", cmd, args).as_str() ));
                },
                _ => ()

            }

    }

    //---------------------
    //  react_native_run_ios()
    //---------------------		
    pub fn react_native_run_ios(
        &self,
        work_dir: &String,
        redirect: bool
    ) -> Option<Child> {
        self.spawn(
            "npx",
            &["react-native", "run-ios" ],
            work_dir,
            redirect
        )
    }

    //---------------------
    //  react_native_run_android()
    //---------------------		
    pub fn react_native_run_android(
        &self,
        work_dir: &String,
        redirect: bool
    ) -> Option<Child> {
        self.spawn(
            "npx",
            &["react-native", "run-android" ],
            work_dir,
            redirect
        )
    }

}