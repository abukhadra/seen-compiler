use indoc::indoc;

use crate::util::print::eprintln_red;

use crate::lang::{
	Lang,
	script::Script,
	lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    infenrence::Inference,
    type_checker::TypeChecker
};


//================
//   fib_ar()
//================
#[test]
fn fib_ar() {	
	type_check_en( indoc!{"
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
	type_check_en( indoc!{"
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
//   type_check_ar()
//================
fn type_check_ar(src: &str) { type_check(Lang::Ar, src)}
//================
//   type_check_en()
//================
fn type_check_en(src: &str) { type_check(Lang::En, src)}

//================
//   type_check()
//================
fn type_check(
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
    let mut type_check = TypeChecker::new();
    let (ast, restab, errs) = type_checker.check(ast, restab);
    if !errs.is_empty() {
        eprintln_red(format!("{:#?}", errs).as_str());
        panic!();
    }
	// assert_eq!(errs.len(), 0 );
}
