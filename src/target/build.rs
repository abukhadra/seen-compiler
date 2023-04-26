use std::{
    fs,
    env,
    path::PathBuf
};

use crate::lang::Lang;

// const PROJECT_NAME: &str = "test";

const CONF_DIR_EN: &str = "conf";
const CONF_DIR_AR: &str = "هيئة";


const RES_DIR_EN: &str = "res";
const RES_DIR_AR: &str = "موارد";

const PAGES_DIR_EN: &str = "pages";
const PAGES_DIR_AR: &str = "صفحات";

const AUDIO_DIR_EN: &str = "audio";
const AUDIO_DIR_AR: &str = "صوتي";


const IMAGES_DIR_EN: &str = "images";
const IMAGES_DIR_AR: &str = "صور";

const VIDEO_DIR_EN: &str = "videos";
const VIDEO_DIR_AR: &str = "مرئي";

const SRC_DIR: &str = "src";
const MAIN_FILE: &str = "main.rs";


//================
//  BuildDir
//================
pub struct BuildDir {
    pub name: String, // FIXME hardcoded
    pub home: PathBuf,
    pub config: PathBuf,
    pub res: ResDir,    
    pub src: SrcDir,
    
}

impl BuildDir {
    //---------------------
    //  new()
    //---------------------      
    pub fn new(
        lang: &Lang,
        name: &String,
        out_dir: Option<PathBuf>
    ) -> Self {

        let home = match out_dir {
            Some(out_dir) => out_dir,
            None => {
                match env::current_dir() {
                    Ok(path) => path,
                    Err(err) => panic!("{:?}", err)
                }
            }
        };
    
        // let mut home = home.clone();
        // home.push(name);    

        let mut config = home.clone();
        config.push(
            match lang {
                Lang::Ar => CONF_DIR_AR,
                Lang::En => CONF_DIR_EN
            }
        );

        let mut res_path = home.clone();
        res_path.push(
            match lang {
                Lang::Ar => RES_DIR_AR,
                Lang::En => RES_DIR_EN
            }
        );        

        let mut pages = res_path.clone();
        pages.push(
            match lang {
                Lang::Ar => PAGES_DIR_AR,
                Lang::En => PAGES_DIR_EN
            }            
        );

        let res = ResDir {
            path: res_path,
            pages
        };


        let mut src_path = home.clone();
        src_path.push(SRC_DIR);

        let mut main = src_path.clone();
        main.push(MAIN_FILE);

        let src = SrcDir {
            path: src_path,
            main
        };
    
        Self {
            name: name.clone(),
            home,
            config,
            res,
            src
        }
    }

    //---------------------
    //  create_dir_all()
    //---------------------  
    pub fn create_dir_all(&mut self) {
        self.src.create_dir();
        self.res.create_dir_all();
    }
}

//================
//  SrcDir
//================
pub struct SrcDir {
    pub path: PathBuf,
    pub main: PathBuf,
}

impl SrcDir {
    //---------------------
    //  create_dir()
    //---------------------  
    pub fn create_dir(&mut self) {
        if let Err(err) = fs::create_dir_all(&self.path) {
            panic!("{}", err);
        }        
    }
}


//================
//  ResDir
//================
pub struct ResDir {
    pub path: PathBuf,
    pub pages: PathBuf,
}

impl ResDir {
    //---------------------
    //  create_dir_all()
    //---------------------  
    pub fn create_dir_all(&mut self) {
        if let Err(err) = fs::create_dir_all(&self.pages) {
            panic!("{}", err);
        }        
    }
}