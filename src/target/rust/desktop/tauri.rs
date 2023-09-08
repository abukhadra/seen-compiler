use std::{
    fs,
    path::PathBuf, 
    fmt::Write
};

use crate::{
    target::{
        build::BuildDir,
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
        rust::rs_features::custom_protocol::CustomProtocol
    }, 
    lang::syntax_tree::ast::StructLiteral
};

//================
//  tauri_desktop()
//================  
impl <'a> Rust<'a> {
    pub fn  tauri_desktop(
        &mut self,
        path: &PathBuf,
        data: &StructLiteral

    ) {

        self.tauri_default_icons(path);
        self.tauri_conf_json(path);
        self.tauri_cargo_toml();
        self.tauri_build_rs(path);
        self.tauri_main_rs();
    }        
}

//================
//  tauri_cargo_toml()
//================  
impl <'a> Rust<'a> {
    fn  tauri_cargo_toml(
        &mut self,
    ) {
        let tauri = Tauri::new();
        let serde = Serde::new();
        let serde_json = SerdeJson::new();
    
        self.cargo_toml.add_dep(tauri);
        self.cargo_toml.add_dep(serde);
        self.cargo_toml.add_dep(serde_json);

        let tauri_build = TauriBuild::new();
        self.cargo_toml.add_build_dep(tauri_build);

        let custom_protocol = CustomProtocol::new();
        self.cargo_toml.add_features(custom_protocol);
    }
}

//================
//  tauri_build_rs()
//================  
// FIXME: hardcoding
impl <'a> Rust<'a> {
    fn  tauri_build_rs(
        &mut self,
        path: &PathBuf,
    ) {
        let mut build_rs = BuildRs::new( 
            path,
r#"fn main() {
    tauri_build::build()
}"#.to_string()
        );     
        build_rs.generate();
    }
}

//================
//  tauri_main_rs()
//================  
// FIXME: hardcoding
impl <'a> Rust<'a> {
    fn  tauri_main_rs(
        &mut self,
    ) {
    let _ = write!(self.res, 
r#"// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {{
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}}"#);
        
        match fs::write(&self.proj_dir.src.main, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }
    }        
}


//================
//  tauri_default_icons()
//================  
// FIXME hardcoding
impl <'a> Rust<'a> {
    fn  tauri_default_icons(
        &mut self,
        path: &PathBuf
    ) {
        let _32x32_png = include_bytes!("tauri/icons/32x32.png");
        let _128x128_png = include_bytes!("tauri/icons/128x128.png");
        let _128x128_2x_png = include_bytes!("tauri/icons/128x128@2x.png");
        let icon_icns = include_bytes!("tauri/icons/icon.icns");
        let icon_ico = include_bytes!("tauri/icons/icon.ico");        
        let icon_png = include_bytes!("tauri/icons/icon.png");
        let square_30x30_logo_png = include_bytes!("tauri/icons/Square30x30Logo.png");
        let square_44x44_logo_png = include_bytes!("tauri/icons/Square44x44Logo.png");
        let square_71x71_logo_png = include_bytes!("tauri/icons/Square71x71Logo.png");
        let square_89x89_logo_png = include_bytes!("tauri/icons/Square89x89Logo.png");
        let square_107x107_logo_png = include_bytes!("tauri/icons/Square107x107Logo.png");
        let square_142x142_logo_png = include_bytes!("tauri/icons/Square142x142Logo.png");
        let square_150x150_logo_png = include_bytes!("tauri/icons/Square150x150Logo.png");
        let square_284x284_logo_png = include_bytes!("tauri/icons/Square284x284Logo.png");
        let square_310x310_logo_png = include_bytes!("tauri/icons/Square310x310Logo.png");
        let store_logo_png = include_bytes!("tauri/icons/StoreLogo.png");

        let mut build_icons_dir = path.clone();
        build_icons_dir.push("icons");
        fs::create_dir_all(&build_icons_dir).expect(format!("expecting icons dir to be created: {:?}",build_icons_dir ).as_str());		

        let mut _32x32_png_path = build_icons_dir.clone();                      _32x32_png_path.push("32x32.png");                              fs::write(_32x32_png_path, _32x32_png).unwrap();
        let mut _128x128_png_path = build_icons_dir.clone();                    _128x128_png_path.push("128x128.png");                          fs::write(_128x128_png_path, _128x128_png).unwrap();
        let mut _128x128_2x_png_path = build_icons_dir.clone();                 _128x128_2x_png_path.push("128x128@2x.png");                    fs::write(_128x128_2x_png_path, _128x128_2x_png).unwrap();
        let mut icon_icns_path = build_icons_dir.clone();                       icon_icns_path.push("icon.icns");                               fs::write(icon_icns_path, icon_icns).unwrap();
        let mut icon_ico_path = build_icons_dir.clone();                        icon_ico_path.push("icon.ico");                                 fs::write(icon_ico_path, icon_ico).unwrap();
        let mut icon_png_path = build_icons_dir.clone();                        icon_png_path.push("icon.png");                                 fs::write(icon_png_path, icon_png).unwrap();
        let mut square_30x30_logo_png_path = build_icons_dir.clone();           square_30x30_logo_png_path.push("Square30x30Logo.png");         fs::write(square_30x30_logo_png_path, square_30x30_logo_png).unwrap();
        let mut square_44x44_logo_png_path = build_icons_dir.clone();           square_44x44_logo_png_path.push("Square44x44Logo.png");         fs::write(square_44x44_logo_png_path, square_44x44_logo_png).unwrap();
        let mut square_71x71_logo_png_path = build_icons_dir.clone();           square_71x71_logo_png_path.push("Square71x71Logo.png");         fs::write(square_71x71_logo_png_path, square_71x71_logo_png).unwrap();
        let mut square_89x89_logo_png_path = build_icons_dir.clone();           square_89x89_logo_png_path.push("Square89x89Logo.png");         fs::write(square_89x89_logo_png_path, square_89x89_logo_png).unwrap();
        let mut square_107x107_logo_png_path = build_icons_dir.clone();         square_107x107_logo_png_path.push("Square107x107Logo.png");     fs::write(square_107x107_logo_png_path, square_107x107_logo_png).unwrap();
        let mut square_142x142_logo_png_path = build_icons_dir.clone();         square_142x142_logo_png_path.push("Square142x142Logo.png");     fs::write(square_142x142_logo_png_path, square_142x142_logo_png).unwrap();
        let mut square_150x150_logo_png_path = build_icons_dir.clone();         square_150x150_logo_png_path.push("Square150x150Logo.png");     fs::write(square_150x150_logo_png_path, square_150x150_logo_png).unwrap();
        let mut square_284x284_logo_png_path = build_icons_dir.clone();         square_284x284_logo_png_path.push("Square284x284Logo.png");     fs::write(square_284x284_logo_png_path, square_284x284_logo_png).unwrap();
        let mut square_310x310_logo_png_path = build_icons_dir.clone();         square_310x310_logo_png_path.push("Square310x310Logo.png");     fs::write(square_310x310_logo_png_path, square_310x310_logo_png).unwrap();
        let mut store_logo_png_path = build_icons_dir.clone();                  store_logo_png_path.push("StoreLogo.png");                      fs::write(store_logo_png_path, store_logo_png).unwrap();

    }
}

//================
//  tauri_conf_json()
//================  
// FIXME hardcoding
impl <'a> Rust<'a> {
    fn  tauri_conf_json(
        &mut self,
        path: &PathBuf,
    ) {
        let mut tauri_conf_json_path = path.clone();
        tauri_conf_json_path.push("tauri.conf.json");

        let title = self.proj_dir.name.clone();

        let tauri_conf_json = format!(
r#"{{
    "build": {{
        "beforeDevCommand": "",
        "beforeBuildCommand": "",
        "devPath": "./js",
        "distDir": "./js",
        "withGlobalTauri": true
    }},
    "package": {{
        "productName": "{}",
        "version": "0.0.0"
    }},
    "tauri": {{
        "allowlist": {{
        "all": false,
        "shell": {{
            "all": false,
            "open": true
        }}
        }},
        "bundle": {{
        "active": true,
        "icon": [
            "icons/32x32.png",
            "icons/128x128.png",
            "icons/128x128@2x.png",
            "icons/icon.icns",
            "icons/icon.ico"
        ],
        "identifier": "com.tauri.dev",
        "targets": "all"
        }},
        "security": {{
        "csp": null
        }},
        "updater": {{
        "active": false
        }},
        "windows": [
        {{
            "fullscreen": false,
            "resizable": true,
            "title": "{}",
            "width": 800,
            "height": 600
        }}
        ]
    }}
}}"#, 
            title,
            title
        );
                
          fs::write(tauri_conf_json_path, tauri_conf_json).unwrap();
    }
}
