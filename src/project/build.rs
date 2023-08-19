use std::path::PathBuf;

use crate::transl::transl::Transl;

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

//================
//   build_src_path()
//================
pub fn build_src_path(
    transl: &Transl,
	home: &PathBuf,
	proj_name: &String
) -> PathBuf{
	let mut src = home.clone();
    src.push(transl.build());
	src.push(proj_name.clone());
	src.push("src");
	src
}
