use std::{
	fs, 
	path::PathBuf,
	process::Child,
};

use clap::{
	Parser,
	Subcommand,
  	CommandFactory
};

use crate::lang::{
	Lang,
	compiler,
	syntax_tree::ast::{
		ModElement,
		Fn,
		StructLiteral,
		Expr,
		BlockElement
	}
};

use crate::project::{
    proj_dir,
};

use crate::transl::transl::Transl;

use crate::tool::cargo::*;

//================
//   Conf
//================
#[derive(Debug)]
pub struct Conf {
    pub proj_name: String,
    pub pre_build: PreBuild
}

impl Conf {
    //---------------------
    //  new()
    //---------------------  	
/*     pub fn new() -> Self {

        Self {
            
        }
    }
 */
}

//================
//   proj_lang()
//================
pub fn proj_lang(home: &PathBuf) -> Result<Lang, String> {
	if conf_ar(home).exists() {
		Ok(Lang::Ar)
	} else if conf_en(home).exists() {
		Ok(Lang::En)
	} else {
		Err(Transl::missing_conf_err_en())     		// FIXME, if conf.seen is not available , check if any other files with an extension 
                                                    //          is available to determine which language should be used to output the error message
                                                    //          if language cannot be determinded then output a bilingual message or have 2 separate seen tools
                                                    //          if it's the english one then default messages to english
                                                    //          finally we need --ar option in all commands to enforce the language 
                                                    //          and a separate seen arabic utiltiy that defaults to arabic
	}
}


//================
//   conf_ar()
//================
pub fn conf_ar(home: &PathBuf) -> PathBuf {
	let mut conf_ar = home.clone();
	conf_ar.push(Transl::conf_ar());
	conf_ar.set_extension(Transl::seen_ext_ar());
	conf_ar
}

//================
//   conf_en()
//================
pub fn conf_en(home: &PathBuf) -> PathBuf {
	let mut conf_en = home.clone();
	conf_en.push(Transl::conf_en());
	conf_en.set_extension(Transl::seen_ext_en());	
	conf_en
}

//================
//   proj_name()
//================
pub fn proj_name(
	transl: &Transl,
	home: &PathBuf
) -> String {

	// let conf_ar = conf_ar(&home);
	// pkg_ar.push("حزمة");			
	let mut conf = home.clone();
	conf.push(transl.conf());
	conf.set_extension(transl.seen_ext());	
	let ast = compiler::to_ast( format!("{}", conf.display()));
	match ast.get(0).unwrap() {
		ModElement::MainFn(Fn{block: els,..}) => {
			match els.get(0).unwrap() {	// assuming that the name is provided in the config file
				BlockElement::Expr(Expr::StructLiteral(StructLiteral{items})) => {
					match items.get(0).unwrap() {
						(t, expr ) => {
							match expr.as_ref().unwrap() {
								Expr::Str(v) => {
                                    if t.to_string() == transl.name() {
                                        v.to_string()
                                    } else {
                                        panic!("{:?}", t)   
                                    }
                                    
                                },
								x => panic!("{:?}", x)
							}
						},
					}
				},
				x => panic!("{:?}", x)
			}
		}
		x => panic!("{:?}", x)
	}	
}

// //================
// //   proj_name_ar()
// //================
// pub fn proj_name_ar(home: &PathBuf) -> String {
// 	let conf_ar = conf_ar(&home);
// 	// pkg_ar.push("حزمة");				
// 	let ast = compiler::to_ast( format!("{}", conf_ar.display()));
// 	match ast.get(0).unwrap() {
// 		ModElement::MainFn(Fn{block: els,..}) => {
// 			match els.get(0).unwrap() {	// assuming that the name is provided in the config file
// 				BlockElement::Expr(Expr::StructLiteral(StructLiteral{items})) => {
// 					match items.get(0).unwrap() {
// 						(t, expr ) => {
// 							match (t.to_string().as_str(), expr.as_ref().unwrap()) {
// 								("الاسم", Expr::Str(v)) => v.to_string(),
// 								x => panic!("{:?}", x)
// 							}
// 						},
// 					}
// 				},
// 				x => panic!("{:?}", x)
// 			}
// 		}
// 		x => panic!("{:?}", x)
// 	}
// }

// //================
// //   proj_name_en()
// //================
// pub fn proj_name_en(home: &PathBuf) -> String {
// 	let conf_en = conf_en(&home);
// 	// conf_en.push("pkg");				
// 	let ast = compiler::to_ast( format!("{}", conf_en.display()));
// 	match ast.get(0).unwrap() {
// 		ModElement::MainFn(Fn{block: els,..}) => {
// 			match els.get(0).unwrap() {	// assuming that the name is provided in the config file
// 				BlockElement::Expr(Expr::StructLiteral(StructLiteral{items})) => {
// 					match items.get(0).unwrap() {
// 						(t, expr ) => {
// 							match (t.to_string().as_str(), expr.as_ref().unwrap()) {
// 								("name", Expr::Str(v)) => v.to_string(),
// 								x => panic!("{:?}", x)
// 							}
// 						}
// 					}
// 				},
// 				x => panic!("{:?}", x)
// 			}
// 		},
// 		x => panic!("{:?}", x)
// 	}
// }

// //================
// //   proj_name()
// //================
// pub fn proj_name(
// 	lang: &Lang,
// 	home: &PathBuf
// ) -> String {
// 	match lang {
// 		Lang::Ar => proj_name_ar(&home),
// 		Lang::En => proj_name_en(&home)
// 	}
// }


//================
//   PreBuild
//================
#[derive(Debug)]
pub struct PreBuild {

}