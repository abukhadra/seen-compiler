// FIXME: the conf formats should be externally configured using data files ( or decorators ),
//			otherwise, new enums, structs and a rebuild is required everytime a new data element or language binding is supported
// TODO: split to a crate, seen / rust / python ...etc should be in their own modules

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

		let transl = Transl::new(&proj_lang);

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
		// &mut self,
		transl: &Transl,
		home: &PathBuf,
	) -> Vec<ConfElement> {
		let mut data = vec![];
		let mut conf = home.clone();
		conf.push(transl.conf());
		conf.set_extension(transl.seen_ext());	
		let ast = compiler::to_ast( format!("{}", conf.display()));
		for el in ast {
			match &el {
				ModElement::MainFn(Fn{block: stmts,..}) => {
					pkg(&el, &transl, &stmts, &mut data);
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
						deps(&el, &transl, &stmts, &mut data);
					} else if name == transl.rust() || name == transl.rs() {
						rust(&el, &transl, &stmts, &mut data);
					} else if name == transl.python() || name == transl.py() {
						python(&el, &transl, &stmts, &mut data);
					} else {
						panic!("unexpected conf function: {:?}", t)	
					}
				},					
				_ => panic!("unexpected element in the seen conf file: {:?}", el) 
			}
		}
		data
	}
 
	//---------------------
	//   proj_name()
	//---------------------
	pub fn proj_name(
		&mut self,
		transl: &Transl,
		home: &PathBuf
	) -> String {
		String::from("INVALID NAME")	// FIXME
	}

}

//================
//   pkg()
//================
fn pkg(
	el: &ModElement,
	transl: &Transl,
	stmts: &Vec<BlockElement>,
	data: &mut Vec<ConfElement>
) {
	let mut main = Main::new();
	for stmt in stmts {
		match stmt {
			BlockElement::Expr(Expr::Ret(_box)) => {
				match &**_box {
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
					_ => panic!("unexpected expression: {:?}", el) 										
				}
			},
			_ => panic!("unexpected statement: {:?}", stmt) 
		}								
	}
	data.push( ConfElement::Main(main) );	
}

//================
//   deps()
//================
fn deps(
	el: &ModElement,
	transl: &Transl,
	stmts: &Vec<BlockElement>,
	data: &mut Vec<ConfElement>
) {
	todo!("conf.seen ::  deps");	// TODO
}

//================
//   rust()
//================
// TODO: rewrite, code is not clean
fn rust(
	el: &ModElement,
	transl: &Transl,
	stmts: &Vec<BlockElement>,
	data: &mut Vec<ConfElement>
) {
	let mut rust = Rust::new();
	for stmt in stmts {
	   match stmt {
		   BlockElement::Expr(Expr::Ret(_box)) => {
			   match &**_box {
				   Expr::StructLiteral(StructLiteral{items}) => {
					   for item in items {
						   match item {
							   (t, expr ) => {
								   if t.to_string() == transl.deps() {
										rust_deps(expr, &transl, &mut rust);
								   } else {
									   match expr.as_ref().unwrap() {	// FIXME unwrap()
										   x => panic!("unknown item: {:?}", x)
									   }
								   }											
							   },
						   }
					   }
					   data.push( ConfElement::Rust(rust.clone()) );
				   },
				   _ => panic!("unexpected statement: {:?}", el) 										
			   }
		   },
		   _ => panic!("unexpected statement: {:?}", el) 								
	   }
	}	
}

//================
//   rust_deps()
//================
fn rust_deps(
	expr: &Option<Expr>,
	transl: &Transl,
	rust: &mut Rust
) {
	match expr.as_ref().unwrap() { // FIXME unwrap()
		Expr::List(List{items}) => {	
			for item in items {
				if let Expr::StructLiteral(StructLiteral{items}) = item {
					let mut dep = RustDep::new();
					for struct_item in items {						
						match struct_item {							
							(t, expr ) => {
								match expr.as_ref().unwrap() {
									Expr::Str(v) => {
										if t.to_string() == transl.id() {
											dep.id = v.to_string();	
										} else if t.to_string() == transl.v() || t.to_string() == transl.version() {
											dep.ver = v.to_string();	
										} else  {
											panic!("unknown item: {:?}", t)   
										}
									},
									Expr::List(list) => {
										if t.to_string() == transl.f() || t.to_string() == transl.features() {
											dep.features = Some(list.to_owned());	
										} else  {
											panic!("unknown item: {:?}", t)   
										}
									},
									x => panic!("unknown item: {:?}", x)
								}
							},
						}
					}
					rust.deps.push(dep);
				} else {
					panic!("expecting a a dependency");
				}
			}
		},
		_ => panic!("expecting a list of dependencies")
	}
	
}


//================
//   python()
//================
// TODO: rewrite, code is not clean and a copy of rust()
fn python(
	el: &ModElement,
	transl: &Transl,
	stmts: &Vec<BlockElement>,
	data: &mut Vec<ConfElement>
) {
	let mut python = Python::new();
	for stmt in stmts {
	   match stmt {
		   BlockElement::Expr(Expr::Ret(_box)) => {
			   match &**_box {
				   Expr::StructLiteral(StructLiteral{items}) => {
					   for item in items {
						   match item {
							   (t, expr ) => {
								   if t.to_string() == transl.deps() {
									python_deps(expr, &transl, &mut python);
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
					   data.push( ConfElement::Python(python.clone()) );
				   },
				   _ => panic!("unexpected statement: {:?}", el) 										
			   }
		   },
		   _ => panic!("unexpected statement: {:?}", el) 								
	   }
	}	
}

//================
//   python_deps()
//================
fn python_deps(
	expr: &Option<Expr>,
	transl: &Transl,
	python: &mut Python	
) {
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
											let dep = PythonDep::new(v.to_string());	// FIXME: currently supporting 
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
				BlockElement::Expr(Expr::Ret(_box)) => {
					let expr = *_box.clone();
						match expr {
							Expr::StructLiteral(StructLiteral{items})  => {
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
									x => panic!("{:?}", x)
								}
							},
							x => panic!("{:?}", x)
						}
				},
				x => panic!("{:?}", x)
			}
		}
		x => panic!("{:?}", x)
	}	
}

// impl ConfData {
// 	pub fn new() -> Self {
// 		Self {
// 			els: vec![]
// 		}
// 	}
// }


//================
//   ConfElement
//================
#[derive(Debug)]
pub enum ConfElement {
	Main(Main),
	Deps(Vec<SeenDep>),

	Rust(Rust),
	Python(Python),
	Prebuild(Fn)
}

//================
//   SeenDep
//================
#[derive(Clone, Debug)]
pub struct SeenDep {
	pub name: String,
	pub ver: String,
	pub path: String
}

//================
//   Rust
//================
#[derive(Clone, Debug)]
pub struct Rust {
	pub deps: Vec<RustDep>
}

impl Rust {
    //---------------------
    //  new()
    //---------------------  		
	pub fn new() -> Self {
		Self {
			deps: vec![]
		}
	}
}

//================
//   RustDep
//================
#[derive(Clone, Debug)]
pub struct RustDep {
	pub id: String,
	pub ver: String,	// TODO, we could make version optional, if its not present then we downloaded the latest version.
	pub features: Option<List>
}

impl RustDep {
	pub fn new() -> Self {
		Self {
			id: String::from(""),
			ver: String::from(""),	
			features: None,
		}
	}
}


//================
//   Python
//================
#[derive(Clone, Debug)]
pub struct Python {
	pub py_path: String,
	pub pkg_man: String,
	pub install: String,
	pub deps: Vec<PythonDep>
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
//   PythonDep
//================
#[derive(Clone, Debug)]
pub struct PythonDep {
	pub name: String,
	pub ver: String,
	pub path: String
}

impl PythonDep {
	pub fn new(name: String) -> Self {
		Self {
			name: String::from(""),
			ver: String::from(""),
			path: String::from(""),
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
