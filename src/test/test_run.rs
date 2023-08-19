
use std::path::PathBuf;

use indoc::indoc;

use crate::project::build;
use crate::tool::cargo::Cargo;
use crate::transl::transl::Transl;
use crate::util::cli::Compile;
use crate::util::print::eprintln_red;

use crate::lang::{
	Lang,
	script::Script,
	lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    // infenrence::Inference,
};


//================
//   struct_ar()
//================
#[test]
fn struct_ar() {	
	run_en( indoc!{r#"
    نقطة {
        س: صحيح،
        ص: صحيح 
    }
    
    ()-> {
        ا:= نفطة { س: ١، ص: ٢ }  
        ب:= نفطة { س: ٣، ص: ٣ }  
        اطبع(ا + ب)
    }
    "#});
}

//================
//   struct_en()
//================
#[test]
fn struct_en() {
	run_en( indoc!{r#"
    Point {
        x: int,
        y: int
    }
    
    ()-> {
        p1 := Point { x: 1, y: 2 } 
        p2 := Point { x: 1, y: 2 } 
    
        println(p1 + p2)
    
    }
    "#});
}

//================
//   run_ar()
//================
fn run_ar(src: &str) { run(Lang::Ar, src)}
//================
//  run_en()
//================
fn run_en(src: &str) { run(Lang::En, src)}

//================
//   run()
//================
fn run(
    lang: Lang,
    src: &str
) {


    todo!();    // FIXME: Compile.exec() and Cargo.new().run() , requires src files and output to the filesystem
                //              need to create another version to read the src from a string and output to the console.



    // let home = match path {
    //     None => std::env::current_dir().unwrap(),
    //     Some(path) => path
    // };
    // let lang = conf::proj_lang(&home).expect("");
    let redirect = true;
    let transl = Transl::new(&lang);
    // let proj_name = conf::proj_name(&transl, &home);
    let home = PathBuf::from("");
    let proj_name = String::from("test_run.rs");
    let build_path = build::build_path(&transl, &home, &proj_name);
    let work_dir = format!("{}", build_path.display());
    Compile::exec(Some(home));
    Cargo::new().run(&work_dir, &vec![], redirect);
}


