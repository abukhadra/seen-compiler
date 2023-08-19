use std::{
    io,
    collections::HashMap, 
    path::{
        PathBuf, 
    }, 
};   

use crate::{
    target::{
        build::BuildDir,
        rust::{
            cargo_toml::CargoToml,
            rs_gen::Rust
        }, 

    }, 
    transl::transl::Transl, project::conf::Conf
};

use crate::debug::lang::compiler::*;

use super::{
    Lang,
    script::{
        ScriptPath,
        Script
    },
    token::{
        Token,
    },
    lexer::Lexer,
    syntax_tree::ast::ModElement,
    symtab::SymTab,
    parser::Parser,
    resolver::{
        ResTab,
        Resolver
    },
    inference::Inference,
    type_checker::TypeChecker,
    error::{
        Error
    }
};

//================
//  Alias
//================
pub type Modules = HashMap<ScriptPath, Data>;

//================
//  Data
//================
#[derive(Debug)]
pub struct Data {
    pub lang: Lang,
    pub script: Script,
    pub tokens: Option<Vec<Token>>,
    pub ast : Option<Vec<ModElement>>,
    pub symtab: Option<SymTab>,
    pub restab: Option<ResTab>,
    pub errors: Vec<Error>,

}

impl Data {
    //---------------------
    //  new()
    //---------------------         
    pub fn new(
        lang: Lang,
        script: Script
    ) -> Self {
        Self {
            lang,
            script,
            tokens: None,
            ast: None,
            symtab: None,
            restab: None,
            errors: vec![]
        }
    }
}

// FIXME, quick hack to get the project name from the conf.seen file 
//================
//  to_ast()
//================
pub fn to_ast(path: String) -> Vec<ModElement> {
    let mut modules = HashMap::from([]);
    let script = Script::from_file(&path);   
    let lang = Lang::lang_from_ext(&path);
    let data = Data::new(lang, script);
    modules.insert(path.clone(), data );    
    let modules = scan(modules);
    let mut modules = parse(modules);    
    for  data in modules.values_mut() {
        return data.ast.take().unwrap()
    }
    vec![]
}



//================
//  compile()
//================
pub fn compile(
    lang: Lang,
    transl: &Transl,
    home: &PathBuf,
    proj_name: &String,
    // proj_dir: &ProjDir,
    out_dir: Option<String>,
    paths: Vec<String>,  // FIXME, switch to PathBuf
    main_mods: Vec<String>
) -> Result<(), io::Error> {

	let mut modules = HashMap::from([]);
	for path in paths {
        let script = Script::from_file(&path);
        let lang = Lang::lang_from_ext(&path);

        let data = Data::new(lang, script);
		modules.insert(path.clone(), data );
	}

    let mut modules = scan(modules);
    let mut modules = parse(modules);
    // let modules = resolve(modules);  // FIXME, turned off, not planned for first release
    // let modules = type_infer(modules); 
    // let modules = type_check(modules); // FIXME, turned off, not planned for first release
    let out_dir = if let Some(out_dir) = out_dir {
        Some(PathBuf::from(out_dir))
    } else {
        None
    };
    generate(&lang, &transl, &home, &proj_name, out_dir, &mut modules, &main_mods);

    Ok(())
}

//================
//  scan()
//================
fn scan (
    mut modules: Modules,
) -> Modules {

    let mut lexer = Lexer::new();    

    for data in modules.values_mut() {
        let (tokens, errors)  = lexer.tokens(&data.lang, &data.script);
        data.tokens = Some(tokens);
        data.errors = errors;
    }

    log::debug!("\n{}", debug_tokens(&modules));    
    print_errors(&modules);

    modules    
}


//================
//  parse()
//================
fn parse (
    mut modules: Modules,
) -> Modules {
    let mut parser = Parser::new();

    for data in modules.values_mut() {
        let tokens = data.tokens.as_mut().unwrap();
        let (ast, symtab, errors) = parser.parse(tokens);
        data.ast = Some(ast);
        data.symtab = Some(symtab);
        data.errors = errors;
    }

    log::debug!("\n{}", debug_ast(&modules));    
    print_errors(&modules);

    modules
}

//================
//  resolve()
//================
fn resolve (
    mut modules: Modules,
) -> Modules {
    for data in modules.values_mut() {
        let mut resolver = Resolver::new();
        let symtab = data.symtab.take().unwrap();
        let (symtab, restab, errors) = resolver.resolve(symtab);
        data.symtab = Some(symtab);
        data.restab = Some(restab);
        data.errors = errors;
    }

    log::debug!("\n{}", debug_resolver(&modules));    
    print_errors(&modules);

    modules
}

//================
//  type_infer()
//================
fn type_infer (
    mut modules: Modules,
) -> Modules {
    
    for data in modules.values_mut() {
        let mut inference = Inference::new();
        let ast = data.ast.take().unwrap();
        let restab = data.restab.take().unwrap();
        let (ast, restab, errors) = inference.infer(ast, restab);
        data.ast = Some(ast);
        data.restab = Some(restab);
        data.errors = errors;
    }

    log::debug!("\n{}", debug_inference(&modules));    
    print_errors(&modules);

    modules
}

//================
//  type_check()
//================
fn type_check (
    mut modules: Modules,
) -> Modules {
    for data in modules.values_mut() {
        let mut type_checker = TypeChecker::new();
        let ast = data.ast.take().unwrap();
        let restab = data.restab.take().unwrap();
        let (ast, restab, errors) = type_checker.check(ast, restab);
        data.ast = Some(ast);
        data.restab = Some(restab);
        data.errors = errors;
    }

    // log::debug!("\n{}", debug_inference(&modules));    
    print_errors(&modules);

    modules
}

//================
//  generate()
//================
fn generate (
    lang: &Lang,
    transl: &Transl,
    home: &PathBuf,
    name: &String,
    out_dir: Option<PathBuf>,
    modules: &mut Modules,
    main_mods: &Vec<String>

) {
    let mut build_dir = BuildDir::new(
        lang,
        name,
        out_dir
    );

    build_dir.create_dir_all();

    // let seen_conf_path = home.join(transl.conf());
    // let seen_conf_path = format!("{}", seen_conf_path.display());
    // let seen_conf_ast = to_ast(seen_conf_path.clone()); // modules.get(&seen_conf_path).expect(format!("seen.conf is missing: looking for`{}`, available files: `{:?}`", seen_conf_path, modules.keys()).as_str());

    let seen_conf = Conf::new(home);

    let mut cargo_toml = CargoToml::new(
        &build_dir.name, 
        &build_dir.home,
        &seen_conf   
    );

    for (path,module) in modules {
        let file_name = std::path::PathBuf::from(path.clone());
        let file_name = file_name.file_stem().expect("expect file stem");
        let file_name = format!("{}", file_name.to_str().expect("expect str"));
        
        let ast= module.ast.as_mut().unwrap();

        let path = Rust::new(
            &mut build_dir, 
            &mut cargo_toml
        ).generate(
            file_name,
            path, 
            // module.lang.ext(), 
            &module.lang,
            ast,
            &main_mods
        );

        // log::debug!("\n{}", debug_generated_src(&path));        
    }
    cargo_toml.generate();
    
}


