#![allow(warnings)]
mod lang;
mod project;
mod target;
mod tool;
mod util;
mod transl;
mod test;
mod debug;

use util::cli::*;

//================
//   main()
//================
fn main() {

    env_logger::init();	// RUST_LOG=debug

	let cli = Cli::new();
	
	match cli.command {
		Some(Commands::New(New{ar, name, template})) => {  
			if let Err(err) = New::exec(ar, name, None, template){ 
				eprintln!("{}", err);
			} 
		},
		Some(Commands::Init(Init{ar, template})) => { 
			if let Err(err) = Init::exec(ar, None, template) {
				eprintln!("{}", err);
			} 
		},
		Some(Commands::Compile(Compile{})) => Compile::exec(None),
		Some(Commands::Build(Build{})) => { let _ = Build::exec(None, false); },
		Some(Commands::Run(Run{})) => { let _ = Run::exec(None, false); },	
		Some(Commands::Clean(Clean{})) => todo!(),
		Some(Commands::Update(Update{})) => todo!(),
		Some(Commands::Check(Check{})) => todo!(),
		Some(Commands::Editor(Editor{ar, path})) => Editor::exec(ar, path),

		None => Cli::print_help()
	}
}
