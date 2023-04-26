use std::{
    fs,
    env,
    fmt::Write, 
    path::PathBuf
};
use crate::lang::Lang;

use super::templates::{ar, en};

// FIXME create a dictionary struct and store the translations in a data file instead of hardcoding and manual lookups
//================
//  Constants
//================
const CONF_DIR_EN: &str = "conf";
const CONF_DIR_AR: &str = "هيئة";

const PACKAGE_CFG_EN: &str = "pkg";
const PACKAGE_CFG_AR: &str = "حزمة";


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



const SRC_DIR_EN: &str = "src";
const SRC_DIR_AR: &str = "مصدر";

const MAIN_FILE_EN: &str = "main";
const MAIN_FILE_AR: &str = "رئيسي";

const BUILD_DIR_EN: &str = "build";
const BUILD_DIR_AR: &str = "بنية";


//================
//  ProjDir
//================
pub struct ProjDir {    
    pub name: String, 
    pub lang: Lang,
    pub home: PathBuf,
    pub conf: PathBuf,
    pub res: ResDir,    
    pub src: SrcDir,
    pub build: PathBuf
}

impl ProjDir {
    //---------------------
    //  new()
    //---------------------      
    pub fn new(
        lang: Lang,
        home: Option<PathBuf>,
    ) -> Self {

        let home = match home {
            Some(path) => path,
            None => {
                match env::current_dir() {
                    Ok(path) => path,
                    Err(err) => panic!("{:?}", err)
                }
            }
        };

        let name = home.file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap();

        // FIXME switched from conf dir to conf.seen
        let mut conf = home.clone();
        conf.push(
            match lang { 
                Lang::Ar => CONF_DIR_AR, 
                Lang::En => CONF_DIR_EN
            }
        );


        let mut path = home.clone();
        path.push(
            match lang {
                Lang::Ar => RES_DIR_AR,
                Lang::En => RES_DIR_EN
            }
        );        

        let mut pages = path.clone();
        pages.push(
            match lang {
                Lang::Ar => PAGES_DIR_AR,
                Lang::En => PAGES_DIR_EN
            }
        );

        let res = ResDir {
            path: path,
            pages
        };


        let mut src_path = home.clone();
        src_path.push(
            match lang {
                Lang::Ar => SRC_DIR_AR,
                Lang::En => SRC_DIR_EN
            }
        );

        let mut main_file = src_path.clone();
        main_file.push(
            match lang {
                Lang::Ar=> MAIN_FILE_AR,
                Lang::En=> MAIN_FILE_EN
            }
        );
        main_file.set_extension(lang.ext());


        let src = SrcDir {
            path: src_path,
            main: main_file
        };
    
        let mut build = home.clone();
        build.push(
            match lang {
                Lang::Ar => BUILD_DIR_AR,
                Lang::En => BUILD_DIR_EN
            }
        );

        Self {
            name,
            lang,
            home,
            conf,
            res,
            src,
            build
        }
    }

    //---------------------
    //  conf_file()
    //---------------------  
    pub fn conf_file(&self) -> PathBuf {
        let mut app_cfg_file = self.conf.clone();
        app_cfg_file.set_extension(self.lang.ext());
        app_cfg_file        
    }

    //---------------------
    //  create_dir_all()
    //---------------------  
    pub fn create_dir_all(&mut self) {
        let app_cfg_file = self.conf_file();
        let mut app_cfg = String::new();

        let code = match self.lang {
            Lang::Ar =>  
format!(r#"() -> {{
    الاسم: «{}»
}}

\\ تبعيات -> [
\\ 
\\ ]
"#, self.name),
            Lang::En =>  
format!(r#"() -> {{
    name: "{}"
}}

// deps -> [
//
// ]
"#, self.name)
        };


        let _ = write!(app_cfg, 
            "{}",
            code,
        );

        match fs::write(app_cfg_file, app_cfg){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }        


        self.src.create_dir();

        // FIXME , create build if build is requested
        // println!("creating {} ...", &self.build.display());
        // if let Err(err) = fs::create_dir_all(&self.build) {
        //     panic!("{}", err);
        // }        

        // FIXME, create res dir as needed
        // self.res.create_dir_all();

        println!("Created project `{}`", self.name);
    }

    //---------------------
    //  create_template()
    //---------------------  
    pub fn create_template(
        &mut self, 
        name: Option<&String>
    ) {
        let code = match name {
            Some(name) => {
                match name.as_str() {
                    "webserver" => match self.lang{
                        Lang::Ar => ar::web_server::code(),
                        Lang::En => en::web_server::code()
                    },
                    _ => panic!("unsupported template")
                }
            },
            None => {
                match self.lang {
                    Lang::Ar => ar::cli::code(),
                    Lang::En => en::cli::code()
                }
            }
        };

        match fs::write(&self.src.main, code){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }        

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