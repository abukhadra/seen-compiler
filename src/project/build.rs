use std::path::PathBuf;

use crate::transl::Transl;

//================
//   build_path()
//================
pub fn build_path(
    transl: &Transl,
	home: &PathBuf,
	proj_name: &String
) -> PathBuf{
	let mut build = home.clone();
    build.push(transl.build());
	build.push(proj_name.clone());
	build
}
