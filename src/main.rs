#![allow(warnings)]
mod lang;
mod project;
mod target;
mod tool;
mod util;
mod transl;
mod test;
mod debug;

use project::ProjSettings;

use util::cli::*;

//================
//   main()
//================
fn main() {
    env_logger::init();	// RUST_LOG=debug
	let cli = Cli::new();	
	let settings = ProjSettings::new(None, false);
	match cli.command {
		// Some(Commands::Editor(Editor{ar, path})) => Editor::exec(ar, path),
		others => {
			match others {
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
				Some(Commands::Compile(Compile{})) => Compile::exec(&settings),
				Some(Commands::Build(Build{})) => { let _ = Build::exec(&settings); },
				Some(Commands::Run(Run{})) => { let _ = Run::exec(&settings); },			
				Some(Commands::Clean(Clean{})) => todo!(),
				Some(Commands::Update(Update{})) => todo!(),
				Some(Commands::Check(Check{})) => todo!(),
				None => Cli::print_help(),
				_ => panic!()
			}
		}
	}
}
