use std::{
    fs,
    env,
    fmt::Write, 
    path::PathBuf
};

use crate::transl::transl::Transl;


//================
//   main_src()
//================
pub fn main_src(
	transl: &Transl,
	home: &PathBuf
) -> Result<String, String> {
	// let lang = proj_lang(home)?;
	// let path = main_path(&lang, &home);
	let path = main_path(&transl, &home);
	match fs::read_to_string(path) {
		Err(err) => Err(err.to_string()),
		Ok(src) => Ok(src)
	}
}

//================
//   main_path()
//================
pub fn main_path(
    transl: &Transl,
	home: &PathBuf
) -> String {
	let mut main_path = home.clone();
	main_path.push(transl.proj.src());
	main_path.push(transl.proj.main());
	main_path.set_extension(transl.seen_ext());
	format!("{}", main_path.display())    

}
