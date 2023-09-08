use std::{
    fs,
    env,
    fmt::Write, 
    path::PathBuf
};


use crate::transl::transl::Transl;

//================
//   src_path()
//================
pub fn src_path(
    transl: &Transl,
	home: &PathBuf
) -> String {
	let mut src_path = home.clone();
	src_path.push(transl.src());
	format!("{}", src_path.display())    
}


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
	main_path.push(transl.src());
	main_path.push(transl.main());
	main_path.set_extension(transl.seen_ext());
	format!("{}", main_path.display())    

}
