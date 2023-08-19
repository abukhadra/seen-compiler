
use indoc::indoc;

use crate::util::print::eprintln_red;

use crate::lang::{
	Lang,
	script::Script,
	lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    infenrence::Inference,
};


//================
//   fib_ar()
//================
#[test]
fn fib_ar() {	
	infer_en( indoc!{"
    ق(ن) ->
	طابق ن {
		٠ => ٠
		١ => ١
		ن => ق(ن-١) + ق(ن-٢)
    }

()-> اطبع_سطر(ق(٣))   
    "});
}

//================
//   fib_en()
//================
#[test]
fn fib_en() {	
	infer_en( indoc!{"
    fib(n) ->
        match n {
            0 => 0
            1 => 1
            n => fib(n-1) + fib(n-2) 
        }
    () -> println(fib(3))   
    "});
}



//================
//   infer_ar()
//================
fn infer_ar(src: &str) { infer(Lang::Ar, src)}
//================
//   infer_en()
//================
fn infer_en(src: &str) { infer(Lang::En, src)}

//================
//   infer()
//================
fn infer(
    lang: Lang,
    src: &str
) {
	let script = Script::from_str(src);

	let (mut tokens, errs) = Lexer::new().tokens(&lang, &script);
    let mut parser = Parser::new();
    let (ast, symtab, errs) = parser.parse(&mut tokens);
    let mut resolver = Resolver::new();
    let (symtab, restab, errs) = resolver.resolve(symtab);
    let mut inference = Inference::new();
    let (ast, restab, errs) = inferences.infer(ast, restab);
    println!("{:#?}", ast);
    if !errs.is_empty() {
        eprintln_red(format!("{:#?}", errs).as_str());
        panic!();
    }
	// assert_eq!(errs.len(), 0 );
}
