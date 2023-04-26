use indoc::indoc;

use crate::util::print::eprintln_red;

use crate::lang::{
	Lang,
	script::Script,
	lexer::Lexer,
    parser::Parser,
};


//================
//   fn()
//================
#[test]
fn _fn() {
	parse_en( indoc!{"
        f(x:int, y:str) -> 
            a = x
            println(a)
        end
	"}); 
}

//================
//   import()
//================
#[test]
fn import() {
	parse_en( indoc!{"
        x = import(\"pkg\")
	"}); 
}

//================
//   fn_with_ret_type()
//================
#[test]
fn fn_with_ret_type() {
	parse_en( indoc!{"
        f(x: int, y: str): str -> 
            a = x
            println(a)
        end
	"}); 
}

//================
//   block()
//================
#[test]
fn block() {
	parse_en( indoc!{"
        () -> 
            f1(a b)
            f2(x, y)
        end
	"}); 
}

//================
//   list()
//================
#[test]
fn list() {
	parse_en( indoc!{"
        () -> [1, a, b, d, f()]
	"}); 
}

//================
//   list_no_comma()
//================
#[test]
fn list_no_comma() {
	parse_en( indoc!{"
        () -> [1 a b d f()]
	"}); 
}

//================
//   list_access()
//================
#[test]
fn list_access() {
	parse_en( indoc!{"
        () -> list[3][4]
	"}); 

}

//================
//   tuple()
//================
#[test]
fn tuple() {
	parse_en( indoc!{"
        () -> (1 a b d f())
	"}); 
}

//================
//   tuple_access()
//================
#[test]
fn tuple_access() {
	parse_en( indoc!{"
        () -> x.0
	"}); 
}


//================
//   lambda()
//================
#[test]
fn lambda() {
	parse_en( indoc!{"
        () -> () -> println()
	"}); 
}

//================
//   let_decl()
//================
#[test]
fn let_decl() {
	parse_en( indoc!{"
	let x = 10
	"}); 	
}

//================
//   short_decl()
//================
#[test]
fn short_decl() {
	parse_en( indoc!{"
	x := 10
	"}); 	
}


//================
//   destruct()
//================
#[test]
fn destruct() {
	parse_en( indoc!{"
        (x, y) := get_data()
	"}); 
}



//================
//   fn_call()
//================
#[test]
fn fn_call() {
	parse_en( indoc!{"
        () -> f(a b)
	"}); 

}

//================
//   _match()
//================
#[test]
fn _match() {
	parse_en( indoc!{"
	()-> 
		match 1
			| 1 => println(\"number 1\")
			| 2 => println(\"number 2\")
			| _ => println(\"unknown value\")
		end
	end		
	"});
}

//================
//   _match_curly()
//================
#[test]
fn _match_curly() {
	parse_en( indoc!{"
	()-> { 
		match 1 {
			| 1 =>  println(\"number 1\") 
			| 2 =>  println(\"number 2\") 
			| _ =>  println(\"unknown value\") 
		}
	}	
	"});
}

//================
//   pat_matching1()
//================
#[test]
fn pat_matching1() {
	parse_en( indoc!{"

	// fib(0) -> 0
	// fib(1) -> 1
	// fib(n) -> fib(n-1) + fib(n-2)


	fib(0) -> 
		0
	end
	fib(1) -> 
		1
	end
	fib(n) -> 
		fib(n-1) + fib(n-2)
	end

	"});
}


//================
//   _for()
//================
#[test]
fn _for() {
	parse_en( indoc!{"
	()-> 
		for x in [1 2 3]
			println(x)
		end
	end		
	"});
}


//================
//   _while()
//================
#[test]
fn _while() {
	parse_en( indoc!{"
	()-> 
		x := 2
		while 1 == x
			println(x)
			break
		end
	end		
	"});
}


//================
//   _if()
//================
#[test]
fn _if() {
	parse_en( indoc!{"
	()-> 
		if 1 == 2
			println(1)
		else if 2 == 3
			println(2)
		else 
			println(3)
		end
	end		
	"});
}


//================
//   unit()
//================
#[test]
fn unit() {
	parse_en( indoc!{"
        () -> ()
	"}); 

}


//================
//   chain()
//================
#[test]
fn chain() {
	parse_en( indoc!{"
        () -> f().b.c.d()
	"}); 

}

//================
//   op_precedence1()
//================
#[test]
fn op_precedence1() {
	parse_en( indoc!{"
        () -> a + b * 9
	"}); 

}


//================
//   op_precedence2()
//================
#[test]
fn op_precedence2() {
	parse_en( indoc!{"
		// () -> fib1(n-1) + fib2(n-2)
		() -> f() + g()
	"}); 

}



//================
//   op_lassoc()
//================
#[test]
fn op_lassoc() {	
	parse_en( indoc!{"
        // let x = 1 + 3 * 5
		// let x = -1 * 2
		// let x = -3 * -5 + -1 * 6
		// let x = f??
		// let x = -f()??
		// let x = f[4]
		// let x = f[4]?
		// let x = -f[4]
		// let x = -f[4]?
		// let x = -f * g
		// let x = -f[3]
		// let x = -f[3]??
		// let x = -f * g
		// let x = -f[1] * g
		// let x = -f[3]?? * -f[6]?()
		
		// let x = (1)
		// let x = (1 + 5 ) * 3 * ( (x) - (1 2))

		// let x = 1 - -1
		// let x = 1 - - 1 // error ,  space
		// let x = f[1]
		// let x = f [1]	// error , space
		// let x = f (1 2 3)	// error , space

		// let x = [1 f(3) (2 3) -2] 

		let x = -f()?.g(3 + 1)??.z[3]
		// let x = -f()?.g(3 + 1)??.z[]	// error because [] can't be empty



	"});    
}

//================
//   op_rassoc()
//================
#[test]
fn op_rassoc() {	
	parse_en( indoc!{"
        x = y = 1 + 3 + 5
	"});    
}



//================
//   hello_world()
//================
#[test]
fn hello_world() {	
	parse_en( indoc!{"
    ()-> println(\"hello world\");"});    
}

//================
//   hello_world_ar()
//================
#[test]
fn hello_world_ar() {	
	parse_ar( indoc!{"
        ()-> اطبع_سطر(\"السلام عليكم\")
    "});
}



//================
//   parse_ar()
//================
fn parse_ar(src: &str) { parse(Lang::Ar, src)}
//================
//   parse_en()
//================
fn parse_en(src: &str) { parse(Lang::En, src)}

//================
//   parse()
//================
fn parse(
    lang: Lang,
    src: &str
) {
	let script = Script::from_str(src);

	let (mut tokens, errs) = Lexer::new().tokens(&lang, &script);
    let mut parser = Parser::new();
    let (ast, symtab, errs) = parser.parse(&mut tokens);
	println!("{:#?}", ast);
    if !errs.is_empty() {
        eprintln_red(format!("{:#?}", errs).as_str());
        panic!();
    }
	// assert_eq!(errs.len(), 0 );
}


