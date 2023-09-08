use indoc::indoc;

use crate::lang::{
	Lang,
	script::Script,
	compiler::compile,
	lexer::Lexer
};

use crate::util::print::eprintln_red;

//================
//   tatweel()
//================
#[test]
fn tatweel() {
	scan_ar( indoc!{r#"
		عرف
		عــــــــــرفــــــــ
	"#});

}


//================
//   valid_id()
//================
#[test]
fn valid_id() {
	scan_en( indoc!{r#"
		dsf
		_fت٩9
	"#});

}

//================
//   valid_ar_num()
//================
#[test]
fn valid_ar_num() {
	scan_ar( indoc!{r#"
		١٢٣	
		١٢٣,٨٣
		٠,٣٢٤	
		4334
		32.0
		9.34
	"#});

}


//================
//   invalid_ar_num()
//================
#[test]
fn invalid_ar_num() {
	scan_ar( indoc!{r#"
			234٤٣٤
			234.٤٣٤
			٤٣٤fdf
			٤٣٤,fdf
	"#});
}


//================
//   valid_en_num()
//================
#[test]
fn valid_en_num() {
	scan_en( indoc!{r#"
			234
			23.8
			0.434
	"#});

}


//================
//   invalid_en_num()
//================
#[test]
fn invalid_en_num() {
	scan_en( indoc!{r#"
			234٤٣٤
			٤٣٤fdf
			234f
			234.f
	"#});

}

//================
//   scan_ar()
//================
fn scan_ar(src: &str) { scan(Lang::Ar, src); }

//================
//   scan_en()
//================
fn scan_en(src: &str) { scan(Lang::En, src); }

//================
//   scan()
//================
fn scan(
	lang: Lang,
	src:&str
) {
	let script = Script::from_str(src);

	let (tokens, errs) = Lexer::new().tokens(&lang, &script);
	println!("{:#?}", errs);	
    if !errs.is_empty() {
        eprintln_red(format!("{:#?}", errs).as_str());
        panic!();
    } else {
		println!("{:?}", tokens);
	}	
 }