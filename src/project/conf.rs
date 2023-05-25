// FIXME: the conf formats should be externally configured using data files,
//			otherwise, new enums, structs and a rebuild is required everytime a new data element or language binding is supported

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
		List,
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
	pub proj_lang: Lang,
	pub transl: Transl,
    // pub proj_name: String,
    // pub pre_build: PreBuild
	pub data: Vec<ConfElement>
}

impl Conf {
    //---------------------
    //  new()
    //---------------------  	
     pub fn new(home: &PathBuf) -> Self {
		let proj_lang = match Self::proj_lang(home) {
			Ok(lang) => lang,
			Err(err) => panic!("{}", err) // FIXME panic!
		};

		let transl = Transl::new(proj_lang);

		let data = Self::elements(&transl, &home);

        Self {
			proj_lang,
			transl,
			data
            
        }
    }

	//---------------------
	//   proj_lang()
	//---------------------
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

	//---------------------
	//   conf_ar()
	//---------------------
	pub fn conf_ar(home: &PathBuf) -> PathBuf {
		let mut conf_ar = home.clone();
		conf_ar.push(Transl::conf_ar());
		conf_ar.set_extension(Transl::seen_ext_ar());
		conf_ar
	}

	//---------------------
	//   conf_en()
	//---------------------
	pub fn conf_en(home: &PathBuf) -> PathBuf {
		let mut conf_en = home.clone();
		conf_en.push(Transl::conf_en());
		conf_en.set_extension(Transl::seen_ext_en());	
		conf_en
	}

	//---------------------
	//   elements()
	//---------------------
	pub fn elements(
		&mut self,
		transl: &Transl,
		home: &PathBuf,
	) -> Vec<ConfElement> {
		// let data = vec![];
		let mut conf = home.clone();
		conf.push(transl.conf());
		conf.set_extension(transl.seen_ext());	
		let ast = compiler::to_ast( format!("{}", conf.display()));
		for el in ast {
			match el {
				ModElement::MainFn(Fn{block: stmts,..}) => {
					let mut main = Main::new();
					for stmt in stmts {
						match stmt {
							BlockElement::Expr(Expr::Ret(_box)) => {
								match *_box {
									Expr::StructLiteral(StructLiteral{items}) => {
										for item in items {
											// match items.get(0).unwrap() {
											match item {
												(t, expr ) => {
													match expr.as_ref().unwrap() {
														Expr::Str(v) => {
															if t.to_string() == transl.name() {
																main.proj_name = v.to_string();
															} else {
																panic!("unknown item: {:?}", t)   
															}
														},
														x => panic!("unknown item: {:?}", x)
													}
												},
											}
										}
									},
									_ => panic!("unexpected statement: {:?}", el) 										
								}
							},
							_ => panic!("unexpected statement: {:?}", el) 
						}								
					}
					self.data.push( ConfElement::Main(main) );
				},
				ModElement::Fn(Fn{name: Some(t), block: stmts,..}) => {
					// FIXME : !!!!!  NEED TO  EXECUTE THE BLOCK :
					//					[] for checks, 
					//					[] file reads, 
					//					[] exceutions
					//					[] create external shell for arabic, 
					//					    [] I would say english too for multi os)
					let name = t.to_string();
					if name == transl.deps() {
						todo!("conf.seen ::  deps");	// TODO
					} else if name == transl.python() {
						let mut python = Python::new();
						 for stmt in stmts {
							match stmt {
								BlockElement::Expr(Expr::Ret(_box)) => {
									match *_box {
										Expr::StructLiteral(StructLiteral{items}) => {
											for item in items {
												match item {
													(t, expr ) => {
														if t.to_string() == transl.deps() {
															match expr.as_ref().unwrap() { // FIXME unwrap()
																Expr::List(List{items}) => {	
																	for item in items {
																		if let Expr::StructLiteral(StructLiteral{items}) = item {
																			for item in items {
																				match item {
																					(t, expr ) => {
																						match expr.as_ref().unwrap() {
																							Expr::Str(v) => {
																								if t.to_string() == transl.id() {
																									let dep = Dep::new(v.to_string());	// FIXME: currently supporting 
																																		//			simple dependency format using name only
																																		//			need to add struct literals to, for 
																																		//			version, path and other properties
																									python.deps.push(dep);
																								} else {
																									panic!("unknown item: {:?}", t)   
																								}
																							},
																							x => panic!("unknown item: {:?}", x)
																						}
																					},
																				}
																			}
																		} else {
																			panic!("expecting a a dependency");
																		}
																	}
																},
																_ => panic!("expecting a list of dependencies")
															}
														} else {
															match expr.as_ref().unwrap() {	// FIXME unwrap()
																Expr::Str(v) => {
																	if t.to_string() == transl.py_path() {
																		python.py_path = v.to_string();
																		
																	} else if t.to_string() == transl.py_path() {
																		python.py_path = v.to_string();
																	} else if t.to_string() == transl.pkg_man() {
																		python.pkg_man = v.to_string();
																	} else if t.to_string() == transl.install() {
																		python.install = v.to_string();
																	} else {																	
																		panic!("unknown item: {:?}", t)   
																	}
																},
																x => panic!("unknown item: {:?}", x)
															}
														}											
													},
												}
											}
											self.data.push( ConfElement::Python(python) );
										},
										_ => panic!("unexpected statement: {:?}", el) 										
									}
								},
								_ => panic!("unexpected statement: {:?}", el) 								
							}
						 }
					} else {
						panic!("unexpected conf function: {:?}", t)	
					}
				},					
				_ => panic!("unexpected element in the seen conf file: {:?}", el) 
			}
		}
		self.data
	}
 
	//---------------------
	//   proj_name()
	//---------------------
	pub fn proj_name(
		transl: &Transl,
		home: &PathBuf
	) -> String {

	}

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

impl ConfData {
	pub fn new() -> Self {
		Self {
			els: vec![]
		}
	}
}


//================
//   ConfElement
//================
#[derive(Debug)]
pub enum ConfElement {
	Main(Main),
	Deps(Vec<Dep>),
	Python(Python),
	Prebuild(Fn)
}

//================
//   Dep
//================
#[derive(Debug)]
pub struct Dep {
	pub name: String,
	pub ver: String,
	pub path: String
}

impl Dep {
	pub fn new(name: String) -> Self {
		Self {
			name: String::from(""),
			ver: String::from(""),
			path: String::from(""),
		}
	}
}


//================
//   Python
//================
#[derive(Debug)]
pub struct Python {
	pub py_path: String,
	pub pkg_man: String,
	pub install: String,
	pub deps: Vec<Dep>
}

impl Python {
    //---------------------
    //  new()
    //---------------------  		
	pub fn new() -> Self {
		Self {
			py_path: String::from(""),
			pkg_man: String::from(""),
			install: String::from(""),
			deps: vec![]

		}
	}
}


//================
//   Main
//================
#[derive(Debug)]
pub struct Main {
	pub proj_name: String
} 

impl Main {
    //---------------------
    //  new()
    //---------------------  		
	pub fn new() -> Self {
		Self {
			proj_name: String::from("")
		}

	}
}
