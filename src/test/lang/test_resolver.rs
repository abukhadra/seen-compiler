use indoc::indoc;

use crate::util::print::eprintln_red;

use crate::lang::{
	Lang,
	script::Script,
	lexer::Lexer,
    parser::Parser,
    resolver::Resolver
};

//================
//   fib_ar()
//================
#[test]
fn fib_ar() {	
	resolve_en( indoc!{"
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
	resolve_en( indoc!{"
    fib(n) ->
        match n {
            0 => 0
            1 => 1
            n => fib(n-1) + fib(n-2) 
        }
    
    fib(n)->
        println(\"test\")
    
    () -> println(fib(3))   
    "});
}



//================
//   resolve_ar()
//================
fn resolve_ar(src: &str) { resolve(Lang::Ar, src)}
//================
//   parse_en()
//================
fn resolve_en(src: &str) { resolve(Lang::En, src)}

//================
//   resolve()
//================
fn resolve(
    lang: Lang,
    src: &str
) {
	let script = Script::from_str(src);

	let (mut tokens, errs) = Lexer::new().tokens(&lang, &script);
    let mut parser = Parser::new();
    let (ast, symtab, errs) = parser.parse(&mut tokens);
    
    let mut resolver = Resolver::new();
    let (symtab, restab, errs) = resolver.resolve(symtab);
	
    if !errs.is_empty() {
        eprintln_red(format!("{:#?}", errs).as_str());
        panic!();
    }
	// assert_eq!(errs.len(), 0 );
}
