use std::{
    fs,
    path::PathBuf, 
    fmt::Write
};

use crate::{
    target::{
        build::{
            BuildDir
        },
        html::html_gen::Html,
        rust::{cargo_toml::CargoToml, rs_gen::Rust},
        rust::build_rs::BuildRs,
        rust::rs_crate::{
            actix_files::ActixFiles,
            actix_web::ActixWeb,
            serde_json::SerdeJson,
            serde::Serde,
            tauri_build::TauriBuild,
            tauri::Tauri
        },
        rust::rs_features::{
            custom_protocol::CustomProtocol
        }
    }, 
    lang::{
        syntax_tree::ast::{
            StructLiteral, 
            Expr
        },
        Lang
    }, 
    util::{
        ar::to_western_num,
        data::data_iter
    }
};

//================
//  actix()
//================  
impl <'a> Rust<'a> {
    pub fn  actix(
        &mut self,
        data: &'a StructLiteral

    ) {
        let settings = self.settings(data);

        self.actix_cargo_toml();
        let res_dir = match self.src_lang {
            Lang::Ar => "موارد",
            Lang::En => "res"
        };

        let pages_dir = match self.src_lang {
            Lang::Ar => "صفحات",
            Lang::En => "pages"
        };    

        let server_start_msg = match self.src_lang {
            Lang::Ar => "لقد تم تشغيل المخدم , العنوان : ",
            Lang::En => "server started: "
        };

        let hostname = match self.src_lang {
            Lang::Ar => {
                match settings.hostname.as_str() {
                    "المضيف_المحلي" => "localhost".to_string(),
                    x => to_western_num(&x.to_string())
                }
            },
            Lang::En => settings.hostname
        };

        let port = match self.src_lang {
            Lang::Ar => to_western_num(&settings.port.to_string()),
            Lang::En => settings.port
        };

        
        self.actix_main_rs(
            &hostname,
            &port,
            res_dir,
            pages_dir,
            server_start_msg
        );
    }
}

//================
//  actix_cargo_toml()
//================  
impl <'a> Rust<'a> {
    pub fn  actix_cargo_toml(
        &mut self
    ) {
        let actix_web = ActixWeb::new();
        let actix_file = ActixFiles::new();

        self.cargo_toml.add_dep(actix_file);
        self.cargo_toml.add_dep(actix_web);

    }
}

//================
//  actix_main_rs()
//================  
// FIXME, hardcoding the example for demo
impl <'a> Rust<'a> {
    pub fn  actix_main_rs(
        &mut self,
        hostname: &String,
        port: &String,
        res_dir: &str,
        pages_dir: &str,
        server_start_msg: &str,
    ) {
        let _ = write!(self.res, 
            r#"use actix_web::{{App, HttpServer}};
            use actix_files::Files;
            
            const HOSTNAME: &str = "{hostname}";
            const PORT: u32 = {port};
            
            #[actix_web::main]
            async fn main() -> std::io::Result<()> {{
                let addr = format!("{{}}:{{}}", HOSTNAME, PORT);
                let server = HttpServer::new(move || {{
                    App::new()            
                        .service(Files::new("/", "./{res_dir}/{pages_dir}").index_file("index.html"))  
            
                }});
            
                println!("{server_start_msg}\n\t\thttp://{{}}", addr);
                server
                .bind(addr)?
                .run()
                .await
            }}"#);
                    
                    match fs::write(&self.proj_dir.src.main, &self.res){
                        Err(err) => panic!("{:?}", err),
                        Ok(_) => ()
                    }
    }
}


//================
//  settings()
//================  
impl <'a> Rust<'a> {
    pub fn  settings(
        &mut self,
        data: &'a StructLiteral
    ) -> ServerSettings {
        let mut settings = None;
        
        let iter = data_iter(&data);

        for (k,v) in iter {
            match k.to_string().as_str() {
                "settings" | "اعدادات" => {
                    settings = self.server_settings(v);
                    break;
                },
                _ => panic!("expecting server settings")
            }
        };

        let settings = settings.expect("expecting server settings");
        settings

    }
}

//================
//   ServerSettings()
//================
pub struct ServerSettings {
    pub hostname: String,
    pub port: String
}

impl ServerSettings {
    pub fn new() -> Self {
        Self {
            hostname : String::new(),
            port: String::new()
        }
    }
}

//================
//   server_settings()
//================
impl <'a> Rust<'a> {
    fn server_settings (
        &mut self,
        data: &Option<Expr>
    ) -> Option<ServerSettings> {
        let mut settings = ServerSettings::new();

        let data = match data {
            Some(Expr::StructLiteral(sruct_literal)) => sruct_literal,
            _ => return None
        };

        for (k,v) in data.items.iter() {
            match k.to_string().as_str() {
                "hostname" | "اسم_المضيف"=> {
                    
                    if let Some(v) = v { 
                        match v {
                            Expr::Str(v) => settings.hostname = v.value.to_string(),
                            _ => panic!("unexpected hostname value")
                        }
                    }
                    
                },
                "port" | "منفذ"=> {
                    if let Some(v) = v { 
                        match v {
                            Expr::Int(v) => settings.port = v.value.to_string().parse().expect("port should be a number"),
                            _ => panic!("unexpected port value")
                        }
                    }                    
                },
                _ => panic!("unsupported: {:?}", k)
            }
        }
        Some(settings)
    }
}

