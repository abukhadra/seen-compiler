
use indoc::indoc;

use crate::util::print::eprintln_red;

use crate::tool::npx::NPX;

//================
//   npx_react_native_init()
//================
#[test]
fn npx_react_native_init() {	
    let npx = NPX::new();

    let work_dir = String::from("/temp");
    let proj_name = String::from("AwesomeProject");
    let redirect = false;

    let _ = npx.react_native_init(&work_dir, &proj_name, redirect);
}