use std::path::PathBuf;

use crate::transl::transl::Transl;

use super::ProjSettings;

//================
//   build_path()
//================
pub fn build_path(
	settings: &ProjSettings
    // transl: &Transl,
	// home: &PathBuf,
	// proj_name: &String
) -> PathBuf{
	let mut build = settings.home.clone();
    build.push(settings.transl.build());
	build.push(settings.proj_name.clone());
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
