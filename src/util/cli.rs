// FIXME : refactor to remove all the copy/pasted code
// FIXME : conditional compilation: transpile ss files only when they are updated.

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
	conf,
	src,
	build
};

use crate::transl::transl::Transl;

use crate::tool::cargo::*;

//================
//   Cli
//================
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    //---------------------
    //  new()
    //---------------------  	
    pub fn new() -> Self {
        Self::parse()        
    }
    //---------------------
    //  print_help()
    //---------------------    
    pub fn print_help() {
        let _ = Cli::command().print_help();
    }
}



//================
//   Commands
//================
#[derive(Debug)]
#[derive(Subcommand)]
pub enum Commands {
  /// New Seen project, creates the project directory.
  New(New),    	
  /// Initialize the Seen project in the current directory
  Init(Init),    
  /// Transpiles the Seen source to target langauges
  Compile(Compile),
  /// Compile the target source and generate any necessary files
  Build(Build),
  /// Run the program
  Run(Run),
  /// Clean 
  Clean(Clean),  
  /// Update
  Update(Update),
  /// Check
  Check(Check),
  // Launch the UI Seen Editor
  Editor(Editor),
}

//================
//   New
//================
#[derive(Parser,Debug)]
#[command()]
pub struct New {
    /// Set language to Arabic
    #[arg(long)]
    pub ar: bool,

    /// The project name:
    pub name: String,

    /// The project template:
    /// e.g:
    /// 	`web_server`
    pub template: Option<String>,
}

impl New {
    //---------------------
    //  exec()
    //---------------------
	pub fn exec(
		ar: bool,
		name: String,
		path: Option<String>,
		template: Option<String>
	) -> Result<(), String> {
		let mut proj_path = match path {
			None => std::env::current_dir().unwrap(),
			Some(path) => PathBuf::from(path)
		};
		 
		proj_path.push(name);
		check_dir_not_exists(&proj_path)?;

		fs::create_dir_all(&proj_path).expect("");
		let _ = Init::exec(ar, Some(proj_path), template); 

		Ok(())
	}
}

//================
//   Init
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Init {
    /// Set language to Arabic
    #[arg(long)]
    pub ar: bool,

    /// The project template:
    /// e.g:
    /// 	`web_server`
    pub template: Option<String>,
}

impl Init {
    //---------------------
    //  exec()
    //---------------------	
	pub fn exec(
		ar: bool,
		path: Option<PathBuf>,
		template: Option<String>
	) -> Result<(), String> {
		let home = match path {
			None => std::env::current_dir().unwrap(),
			Some(path) => path
		};

		check_dir_empty(&home)?;

		let lang = if ar { Lang::Ar} else { Lang::En};
		let mut proj = proj_dir::ProjDir::new(lang, Some(home));

		match template {
			Some(template) => {
				match template.as_str() {
					"webserver" => {
						proj.create_dir_all();
						proj.create_template(Some(&template));
					},
					_ => println!("unsupported : init {}", template)
				}
			},
			None => {
				proj.create_dir_all();
				proj.create_template(None);
			},
		}
		Ok(())	
	}
}

//================
//   Compile
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Compile {}

impl Compile {
    //---------------------
    //  exec()
    //---------------------		
	pub fn exec(path: Option<PathBuf>) {
		let home = match path {
			None => std::env::current_dir().unwrap(),
			Some(path) => path
		};

		let lang = conf::proj_lang(&home).expect("");
		let transl = Transl::new();
		let proj_name = conf::proj_name(&transl, &home);
		let build_path = build::build_path(&transl, &home, &proj_name);
		let out = Some(format!("{}", build_path.display()));
		let main_path = src::main_path(&transl, &home);
		let paths = vec![main_path];
		if let Err(err) = compiler::compile(lang, &proj_name, out, paths ) {
			panic!("{}", err);
		  }  	
	
		println!("{} built successfully.", proj_name);
	}
}

//================
//   Build
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Build {
/*       /*
          // FIXME arabic requires right to left support
          /// ملفات
          #[arg(name="ملفات")]
      */		
      /// Seen source files
      pub paths: Option<Vec<String>>,

      /// Destination directory
      #[arg(short, long)]
      pub output: Option<String>, */
}

impl Build {
    //---------------------
    //  exec()
    //---------------------			
	pub fn exec(
		path: Option<PathBuf>,
		redirect: bool
	) -> Option<Child> {
		let home = match path {
			None => std::env::current_dir().unwrap(),
			Some(path) => path
		};
		let lang = conf::proj_lang(&home).expect("");
		let transl = Transl::new();
		let proj_name = conf::proj_name(&transl, &home);
		let build_path = build::build_path(&transl, &home, &proj_name);
		let work_dir = format!("{}", build_path.display());
		Compile::exec(Some(home));
		Cargo::new().build(&work_dir, redirect)
	}	
}

//================
//   Run
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Run {}

impl Run {
    //---------------------
    //  exec()
    //---------------------		
	pub fn exec(
		path: Option<PathBuf>,
		redirect: bool
	) -> Option<Child> {
		let home = match path {
			None => std::env::current_dir().unwrap(),
			Some(path) => path
		};
		let lang = conf::proj_lang(&home).expect("");
		let transl = Transl::new();
		let proj_name = conf::proj_name(&transl, &home);
		let build_path = build::build_path(&transl, &home, &proj_name);
		let work_dir = format!("{}", build_path.display());
		Compile::exec(Some(home));
		Cargo::new().run(&work_dir, redirect)
	}	
}

//================
//   Clean
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Clean {}

//================
//   Update
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Update {}

//================
//   Check
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Check {}


//================
//   Editor
//================
#[derive(Parser,Debug)]
#[command()]
pub struct Editor {
	#[arg(long)]
	/// Set language to Arabic
	pub ar: bool,

	/// Launch the editor and open the project at the specified path
	///	e.g:
	///		`seen editor .` will open the project in the current directory
	pub path: Option<String>,
}

impl Editor {
    //---------------------
    //  exec()
    //---------------------		
	pub fn exec(
		ar: bool,
		path: Option<String>
	) {
		panic!()	// TODO Editor is handled in the tauri project, 
					// remove the main and cli from here, make this a lib only
					//	handle both cli (clap) and gui from the tauri project
	}
}

//================
//   check_dir_not_exists()
//================
pub fn check_dir_not_exists(path: &PathBuf) -> Result<(), String> {
	if path.exists() { 
		return Err( 
			format!(
				"Error: {} already exists!", 
				path.display()
			)
		)
	}
	Ok(())
}


//================
//   check_dir_empty()
//================
pub fn check_dir_empty(path: &PathBuf) -> Result<(), String> {
	match path.read_dir() {
		Ok(mut entries) => {
			if entries.next().is_some() {
				return Err( 
					format!(
						"Error: {} contains files, you can only run init inside an empty directory!", 
						path.display()
					)
				)			
			 } else {
				Ok(())
			 }
		},
		Err(err) => Err(err.to_string())
	}
}
