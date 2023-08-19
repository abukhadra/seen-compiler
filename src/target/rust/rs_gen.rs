use std::{
    fs,
    fmt::{
        Write
    }, path::PathBuf,
};

use crate::lang::{
    Lang,
    token::{
        Token,
        Location,
        TokenValue
    },
    syntax_tree::ast::*, 
};

use crate::util::{
    ar::to_western_num,
    indent::Indent 

};

use crate::target::{
    build::{
        BuildDir
    },
    html::html_gen::Html,
    rust::cargo_toml::CargoToml,
    rust::rs_crate::{
        actix_files::ActixFiles,
        actix_web::ActixWeb
    }
};

//================
//   Constants
//================
const RS_EXT: &'static str = "rs";
const INDEX_HTML: &'static str = "index.html";


//================
//   Rust
//================
pub struct Rust<'a> {
    src_lang: Lang,
    path: String,
    indent: Indent,
    res: String,
    html: Option<Html>,
    proj_dir : &'a mut BuildDir,
    cargo_toml : &'a mut CargoToml,
    imports: Vec<String>         // FIXME: vector of imported modules... used as a workaround for not having a resolver / semantic analyzer
                                //          for now , we are supporint  module name ( single token ) imports
                                //          during code generation, a reference will be checked if it exists in this vector, if it does , then we will use :: rather than . to access elements
                                //          later on, the resolver / semantic analyzer should eliminate the need for this workaround
}

impl <'a> Rust<'a> {
    //---------------------
    //  new()
    //---------------------
    pub fn new (
        project_struct: &'a mut BuildDir,
        cargo_toml: &'a mut CargoToml
    ) -> Self {
        Self {
            src_lang: Lang::Ar,
            path: String::new(),
            indent: Indent::new(),
            res: String::new(),
            html: None,
            proj_dir: project_struct,
            cargo_toml,
            imports: vec![]
        }
    }

    //---------------------
    //  generate()
    //---------------------
    pub fn generate(
        &mut self,
        mut file_name: String,
        path: &String,
        src_lang: &Lang, //&str,
        ast: &mut Vec<ModElement>,
        main_mods: &Vec<String>
    ) {

        let _ = writeln!(self.res, "#![allow(warnings)]\n");


        self.src_lang = src_lang.clone();
        self.path = path.clone();
        self.html = Some(Html::new(
            &self.src_lang,
            &self.path, 
            self.src_lang.ext()
        ));

        for el in ast.iter() {
            match el {
                // ModElement::Decl(el) => self.asgmt(el),
                ModElement::Decl(el) => self.decl(&el),
                ModElement::MainFn(el) => self.main_fn(el, main_mods),
                ModElement::Fn(el) => self._fn(el),
                ModElement::Struct(el) =>  self._struct(el),
                ModElement::StructImpl(el) => self.struct_impl(el),
                ModElement::Trait(el) => self._trait(el),
                ModElement::Enum(el) => self._enum(el),
                ModElement::EnumImpl(el) => self.enum_impl(el),                
            }
        }
        // match fs::write(&self.proj_dir.src.main, &self.res){
        let mut path_buf = self.proj_dir.src.path.clone();
        if file_name == "رئيسي" { file_name = "main".to_string() }  // FIXME use enums instead of strings
        
        path_buf.push(file_name);
        path_buf.set_extension("rs");
        match fs::write(&path_buf, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }            

    }
}

// //================
// //   asgmt()
// //================
// impl <'a> Rust<'a> {
//     fn asgmt(
//         &mut self, 
//         asgmt: Decl 
//     ) {
//         todo!();    // TODO
//     }
// }
    
//================
//   main_fn()
//================
impl <'a> Rust<'a> {
    fn main_fn(
        &mut self,
        mut _fn: &Fn,
        main_mods: &Vec<String>
    ) {
        if self.is_attr("web_server", &_fn.attrs) || self.is_attr("مخدم_شع", &_fn.attrs) {     // FIXME hardcoding @web_server for the demo
            self.web_server_main(&_fn.block, &_fn.attrs);
         } else {
            let _ = writeln!(self.res);
            for _mod in main_mods {
                if let Lang::Ar = self.src_lang  {  // FIXME this should be applied to any nonascii mod name, not just arabic
                    let _ = writeln!(self.res, "#[path = \"{}.rs\"]", _mod);
                }
                let _ = writeln!(self.res, "mod {};", _mod);    
            }
            let _ = writeln!(self.res);
            let _ = write!(self.res, "fn main()");
            if self.fn_main_has_params(&_fn.params) {    // TODO: main with params is not tested
                // _fn.block.insert(0, BlockElement::MainArgs); // FIXME: if args are passed and used, then should add args at the beginning of the main function 
            }
            self.fn_body(&_fn.block, &_fn.attrs);
            let _ = writeln!(self.res);
        }
    }
}

//================
//   fn_main_has_params()
//================
impl <'a> Rust<'a> {
    fn fn_main_has_params(
        &mut self,
        params: &Vec<Param>,
    ) -> bool {
        // FIXME: currently, supporting only (args: [str]) -> ... , should be able to deconstruct using patterns
        for param in params {
            if is_main_param_pat_id(&param.pat) == is_main_param_list_str(&param._type) {
                return true
            } else {
                return false
            }
        }
        false
    }

}

//================
//   is_main_param_pat_id()
//================
fn is_main_param_pat_id(pat: &Pattern) -> bool {
    match pat  {
        Pattern::Id(_) => true,
        _ => false
    }
}

//================
//   is_main_param_list_str()
//================
fn is_main_param_list_str(_type: &Option<Type>) -> bool {
    match _type  {
        Some(
            Type::ListType(
                ListType{ 
                    els_type: els_type
                }
            )
        ) => {
            let els_type = *els_type.clone();
            match els_type {
                Type::PrimitiveType(
                    PrimitiveType{
                        id: Token { 
                            value: TokenValue::Id(v), 
                            .. 
                        }
                    }
                ) if v == "str "=> true,
                _ => false
            }    
        },
        _ => false
    }
}


//================
//   _fn()
//================
impl <'a> Rust<'a> {  
    fn _fn(
        &mut self,
        _fn: &Fn,  
    ) {
        
        let name = if let Some(name) = &_fn.name {
            match &name.value {
                TokenValue::Id(id) => id.to_owned(),
                _ => panic!()
            }
        } else {
            String::from("")
        }; 

        let _ = write!(self.res, "{}pub fn {}",self.indent , name);     // FIXME: for now, all impl block members are going to be public, change code to make them public as needed
        self.fn_params(&_fn.params, _fn.is_method);
        self.fn_ret_type(&_fn.ret_type);
        self.fn_body(&_fn.block, &_fn.attrs);
        let _ = writeln!(self.res);
    }
}


//================
//   fn_params()
//================
impl <'a> Rust<'a> {
    fn fn_params(
        &mut self,
        params: &Vec<Param>,
        is_method: bool
    ) {
        let _ = write!(self.res, "(");  
        if is_method {
            let _ = write!(self.res, "&mut self, ");        // FIXME assuming everything is a &mut , fix to accomdate all cases incuding &self, self 
                                                            //          also , do not print comma if we have 0 params
        }   
        for (i, param) in params.iter().enumerate() {
            match &param._type {
                None => todo!("type inference"),
                Some(_type) => {
                    match &param.pat {
                        Pattern::Id(pat) => {
                            let _ = write!(self.res, "{}", pat.id);
                        },
                        _ => todo!("only id pattern is implemented")
                        
                    }
                    self.type_annotation(&_type);
                    if i < params.len() - 1 {
                        let _ = write!(self.res, ", ");
                    }
                }
            }
        }
        let _ = write!(self.res, ") ");  
    }
}

//================
//   fn_ret_type()
//================
impl <'a> Rust<'a> {
    fn fn_ret_type(
        &mut self,
        ret_type: &Option<Type>,
    ) {
        match ret_type {
            // None => panic!("bug, should not pass the type checker"),
            None => (),
            Some(Type::UnitType) => (),
            Some(_type) => {
                let _ = write!(self.res, " -> ");
                self._type(&_type);
                let _ = write!(self.res, " ");
            }
            
        }
    }
}

//================
//   web_server_main()
//================
impl <'a> Rust<'a> {
    fn web_server_main(
        &mut self,
        els: &Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>
    ) {
    
        for el in els {
            match el {
                BlockElement::Expr(Expr::Ret(v)) => {
                    let v = &**v;
                    match v {
                        Expr::StructLiteral(data) => {
                            // FIXME just a quick hack to demo the project, in the real app, the attributes will alter the ast 
                            if self.is_attr("web_server", attrs) || self.is_attr("مخدم_شع", attrs){
                                self.web_server(&data);
                            }
                        },
                        _ => todo!()   // FIXME
                    }
                    
                }
                _ => todo!() // TODO
            }
        }
    }
}

//================
//   fn_body()
//================
// FIXME: no need to match BlockElement::Expr variants here, just extract the Expr and send it to self.expr(..)
impl <'a> Rust<'a> {
    fn fn_body(
        &mut self,
        els: &Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>
    ) {
        let _ = writeln!(self.res, "{{");    
        self.indent.inc();
        for el in els {
            let _ = write!(self.res, "{}", self.indent);    
            match el {
                BlockElement::Expr(Expr::BinOp(bin_op)) => {


                    if let Some((name, args)) = &self.maybe_fn_call(&bin_op, attrs) {   
                        self.temp_std(name, args); 
                    } else if self.is_imported_module(&bin_op.l_opr) {   // FIXME this is a workaround , for now only importing modules in same dir are supported, `use` can be much more complex , handle all scenarios
                        self.fix_import_path(&bin_op);
                    }else {
                        todo!("todo: unsupported element: {:#?}", bin_op);
                    }
                    let _ = writeln!(self.res, ";");
                },
                BlockElement::Expr(Expr::StructLiteral(data)) => {
                    // FIXME just a quick hack to demo the project, in the real app, the attributes will alter the ast 
                    if self.is_attr("web_server", attrs) {
                        self.web_server(&data);
                    }
                },
                BlockElement::Expr(Expr::Match(_match)) => self._match(&_match) ,
                BlockElement::Expr(Expr::For(_for)) => self._for(&_for) ,
                BlockElement::Expr(Expr::While(_while)) => self._while(&_while) ,
                BlockElement::Expr(Expr::If(_if)) => self._if(&_if) ,
                BlockElement::Decl(decl) => self.decl(&decl) ,

                // FIXME temporary hardcoded variants
                BlockElement::Expr(Expr::Ok(expr)) => {
                    let _ = write!(self.res, "Ok(");
                    self.expr(&expr);
                    let _ = write!(self.res, ")");
                },
                BlockElement::Expr(Expr::Err(expr)) => {
                    let _ = write!(self.res, "Err(");
                    self.expr(&expr);
                    let _ = write!(self.res, ")");
                },
                BlockElement::Expr(Expr::Some(expr)) => {
                    let _ = write!(self.res, "Some(");
                    self.expr(&expr);
                    let _ = write!(self.res, ")");
                },
                BlockElement::Expr(Expr::None) => {
                    let _ = write!(self.res, "None");
                },
                BlockElement::Expr(Expr::Int(num)) => {
                    let _ = write!(self.res, "{}", num);
                },
                BlockElement::Expr(Expr::Float(num)) => {
                    let _ = write!(self.res, "{}", num);
                },                
                BlockElement::Expr(Expr::Ref(name)) => {
                    let _ = write!(self.res, "{}", name);
                },             
                BlockElement::Expr(Expr::Ret(expr)) => {
                    self.expr(&expr);
                },            
                x => {
                    todo!("{:?}", x) // TODO
                }
            }
            let _ = writeln!(self.res, "");    
            
        }
                
        self.indent.dec();
        let _ = writeln!(self.res, "{}}}", self.indent);
    }
}


//================
//   is_attr()
//================
impl <'a> Rust<'a> {
    fn is_attr(
        &mut self,
        name: &str,
        attrs: &Option<Vec<Attr>>
    ) -> bool {
        if let Some(attrs) = attrs {
            if let Some(attr) =  attrs.get(0) {
                self.is_ref_attr(name, attr) 
            } else {
                false
            }                     
        } else {
            false
        }
    }

}

//================
//   is_ref_attr()
//================
impl <'a> Rust<'a> {
    fn is_ref_attr(
        &mut self,
        name: &str, 
        attr: &Attr
    ) -> bool {
        match &attr.expr {
            AttrExpr::Ref(v) => {
                v.to_string().as_str() == name 
            }
        }
    }
}

//================
//   maybe_fn_call()
//================
impl <'a> Rust<'a> {
    fn maybe_fn_call(
        &mut self,
        op: &BinOp,
        attrs: &Option<Vec<Attr>>
    ) -> Option<(String, Tuple)> {                                  

        match &*op.l_opr {
            Expr::Ref(t) => {
                let name = t.to_string();

                match &*op.r_opr {
                    Expr::Tuple(args) => {
                        
                        return Some((name, args.clone())) 
                    },
                    x => {
                        None
                    }
                }
            },
            _ => None
        }
    }
}


//================
//   maybe_struct_init()
//================
impl <'a> Rust<'a> {
    fn maybe_struct_init(
        &mut self,
        op: &BinOp,
        attrs: &Option<Vec<Attr>>
    ) -> Option<(String, StructLiteral)> {                                  

        match &*op.l_opr {
            Expr::Ref(t) => {
                let name = t.to_string();

                match &*op.r_opr {
                    Expr::StructLiteral(fields) => {
                        
                        return Some((name, fields.clone())) 
                    },
                    x => {
                        None
                    }
                }
            },
            _ => None
        }
    }
}


//================
//  struct_init()
//================  
// TODO: support multiple formats / indentation according to the number of fields and context
//  e.g:
//          let x = Struct{ a: int}
//          let x = Struct {
//                  a: 1,
//                  b: 2
//          }
// ...etc
impl <'a> Rust<'a> {     
    pub fn struct_init(
        &mut self, 
        name: &String, 
        fields: &StructLiteral
    ) {
        let _ = writeln!(self.res, "{} {{", name);
        self.indent.inc();
        for (name, expr) in fields.items.iter() {
            let _ = write!(self.res, "{}", self.indent);
            let expr = expr.clone().expect("optional values are not supported yet"); // FIXME : if value is absent then assign default
            let _ = write!(self.res, "{}: ", name);
            self.expr(&expr);
            let _ = writeln!(self.res, ",");
        };

        self.indent.dec();
        let _ = write!(self.res, "{}}}", self.indent);  
    
    }
}


//================
//   _struct()
//================
impl <'a> Rust<'a> {
    fn _struct(
        &mut self,
        _struct: &Struct,
    ) {
        let _  = writeln!(self.res, "#[derive(Debug, Clone)]"); // FIXME : add Debug/Clone by default for DEV , improve later by adding them as needed
        let _  = writeln!(self.res, "struct {} {{" , _struct.name); 
        if let Some(fields) = &_struct.fields {
            self.indent.inc();
            for (name, _type) in fields.iter() {
                let _ = write!(self.res, "{}", self.indent);
                let _ = write!(self.res, "pub {}: ", name );    // FIXME: by default all structs are pub, later on introduce -/+ to restrict
                self._type(_type);
                let _ = writeln!(self.res, ",");

            }
            self.indent.dec();
            let _ = writeln!(self.res, "{}}}\n", self.indent);
        } else {
            let _ = writeln!(self.res, "}}\n");
        }

        // FIXME: for quicker DEV: implement Display for every struct by default to print the Debug impl
        //          improve later
        let _ = writeln!( self.res, "{}impl std::fmt::Display for {} {{", self.indent, _struct.name);
        self.indent.inc();
        let _ = writeln!( self.res, "{}fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{", self.indent);
        self.indent.inc();
        let _ = writeln!( self.res, "{}write!(f, \"{{:#?}}\", self)", self.indent);
        self.indent.dec();
        let _ = writeln!(self.res, "{}}}", self.indent);
        self.indent.dec();
        let _ = writeln!(self.res, "{}}}\n", self.indent);
                
    }
}

//================
//   struct_impl()
//================
// FIXME: code is assuming a single method/func impl blocks, later on when parser is updated for multi support, change the code here as well.
impl <'a> Rust<'a> {
    fn struct_impl(
        &mut self,
        struct_impl: &StructImpl,
    ) {
        let _ = writeln!( self.res, "{}impl {} {{", self.indent, struct_impl.name);
        self.indent.inc();

        for _fn in &struct_impl.fns {
            self._fn(_fn);
        }

        self.indent.dec();
        let _ = writeln!(self.res, "{}}}\n\n", self.indent);

    }
}

//================
//   _trait()
//================
impl <'a> Rust<'a> {
    fn _trait(
        &mut self,
        _trait: &Trait,
    ) {
        todo!();    // TODO
    }
}

//================
//   _enum()
//================
impl <'a> Rust<'a> {
    fn _enum(
        &mut self,
        _enum: &Enum,
    ) {
        todo!();    // TODO
    }
}

//================
//   enum_impl()
//================
impl <'a> Rust<'a> {
    fn enum_impl(
        &mut self,
        enum_impl: &EnumImpl,
    ) {
        todo!();    // TODO
    }
}

//================
//   type_annotation()
//================
impl <'a> Rust<'a> {
    fn type_annotation(
        &mut self,
        _type: &Type
    ) {
        let _ = write!(self.res, ": ");
        self._type(_type);
    }
}

//================
//   rs_type()
//================
// FIXME, separate ar / en
impl <'a> Rust<'a> {
    fn rs_type(
        &mut self,
        _type: &Type
    ) -> String {
        let _type = match _type {
            Type::OptionType(opt_type) => {
                format!("Option<{}>", self.rs_type(&opt_type.some_type))
            },
            Type::ResultType(res_type) => {
                format!("Result<{}, {}>",
                 self.rs_type(&res_type.ok_type),
                 self.rs_type(&res_type.err_type),
                )

            },            
            Type::PrimitiveType(PrimitiveType{id}) => {
                let _type = match id.value.to_string().as_str() {
                    "bool" => "bool",
                    "منطقي" => "bool",
                    "char" => "char",
                    "محرف" => "char",
                    "int" => "i32",
                    "صحيح" => "i32",
                    "float" => "f32",
                    "عائم" => "f32",
                    "str" => "String",
                    "سلسلة" => "String",
                    _ => panic!("unkown primitive type {}", id.value)
                };
                _type.to_string()
            },
            Type::ListType(ListType{els_type}) => {
                format!("Vec<{}>", self.rs_type(&els_type))
            },
            Type::TupleType(TupleType{types}) => {
                let mut res = String::from("(");
                for (i,el_type) in types.iter().enumerate() {
                    let _ = write!(res, "{}", self.rs_type(&el_type));
                    if i < types.len() - 1 {
                        let _ = write!(res, ", ");
                    }
                }

                let _ = write!(res, ")");
                res
            },            
            _ => todo!()
        };
        _type
    }    
}


//================
//   _type()
//================
impl <'a> Rust<'a> {
    fn _type(
        &mut self,
        _type: &Type
    ) {
        let rs_type = self.rs_type(&_type);
        let _ = write!(self.res, "{}", rs_type); 
    }    
}

//================
//  expr()
//================    
impl <'a> Rust<'a> {         
    fn expr(
        &mut self,
        _expr: &Expr
    ) {

        match _expr {
            Expr::Bool(v) 
            | Expr::Char(v)
            | Expr::Str(v) => {
                let _ = write!(self.res, "\"{}\"", v);
                let _ = write!(self.res, ".to_string()");  // FIXME, for now treat all str as String
            },
            Expr::Int(v)
            | Expr::Float(v) => {
                let _ = write!(self.res, "{}", to_western_num(&v.to_string()));
            },
            Expr::Ref(id) => {
                let _ = write!(self.res, "{}", id.to_string());
            }
            Expr::List(l) => self.list(l),
            Expr::Tuple(tuple) => self.tuple(tuple),
            Expr::PreUniOp(uni_op) => self.pre_uni_op(uni_op),
            Expr::PostUniOp(uni_op) => self.post_uni_op(uni_op),
            Expr::BinOp(bin_op) => self.bin_op(bin_op),
            // Expr::Ret(expr) => self.expr(&expr),        // FIXME: sometimes we need to write explicit return statements.

            Expr::Match(_match) => self._match(&_match) ,
            Expr::For(_for) => self._for(&_for) ,
            Expr::While(_while) => self._while(&_while) ,
            Expr::If(_if) => self._if(&_if) ,
            Expr::Ret(expr) => self.expr(&expr) ,   // FIXME: sometimes we need to explicitly print "return"
            // FIXME temporary hardcoded variants
            Expr::Ok(expr) => {
                let _ = write!(self.res, "Ok(");
                self.expr(&expr);
                let _ = write!(self.res, ")");
            },
            Expr::Err(expr) => {
                let _ = write!(self.res, "Err(");
                self.expr(&expr);
                let _ = write!(self.res, ")");
            },
            Expr::Some(expr) => {
                let _ = write!(self.res, "Some(");
                self.expr(&expr);
                let _ = write!(self.res, ")");
            },
            Expr::None => {
                let _ = write!(self.res, "None");
            },

            x => {
                todo!("expr: {:?}", x)
            }            
        }
    }    
}

//================
// rs_bin_op()
//================  
impl <'a> Rust<'a> {     
    pub fn rs_bin_op(
        &mut self, 
        op: &Token
    ) {
        let rs_bin_op = match &op.value {
            TokenValue::BitwiseAnd => "&".to_string(),
            TokenValue::BitwiseOr => "|".to_string(),
            TokenValue::OpenParen  | TokenValue::OpenBracket=> "".to_string(),
            x => x.to_string()
        };
        let _ = write!(self.res, " {} ", rs_bin_op);
    }
}

//================
//  pre_uni_op()
//================  
impl <'a> Rust<'a> {     
    pub fn pre_uni_op(
        &mut self, 
        uni_op: &UniOp
    ) {
        match uni_op {
            UniOp{ opr, op } => {
                if matches!(op.value , TokenValue::Minus) {
                    let _ = write!(self.res, "-(");
                    self.expr(opr);
                    let _ = write!(self.res, ")");  // FIXME use parens only if necessary, such as having a binary expression
                }
            },
            _ => todo!()
        }
    }
}

//================
//  post_uni_op()
//================  
impl <'a> Rust<'a> {     
    pub fn post_uni_op(
        &mut self, 
        uni_op: &UniOp
    ) {
        match uni_op {
            UniOp{ opr, op } => {
                match &op.value  {
                    TokenValue::Question => {
                        let _ = write!(self.res, "(");
                        self.expr(opr);
                        let _ = write!(self.res, ")?");  // FIXME use parens only if necessary, such as having a binary expression    
                    },
                    TokenValue::Exclamation => {
                        let _ = write!(self.res, "(");
                        self.expr(opr);
                        let _ = write!(self.res, ").unwrap()");  // FIXME use parens only if necessary, such as having a binary expression                            
                    }
                    _ => todo!()
                }
            },

            _ => todo!()
        }
    }
}

//================
//  bin_op()
//================  
impl <'a> Rust<'a> {     
    pub fn bin_op(
        &mut self, 
        bin_op: &BinOp
    ) {
        
        if self.is_imported_module(&bin_op.l_opr) {   // FIXME this is a workaround , for now only importing modules in same dir are supported, `use` can be much more complex , handle all scenarios
            self.fix_import_path(&bin_op);
        } else if let Some((name, args)) = &self.maybe_fn_call(&bin_op, &None) {
            self.temp_std(name, args); 
        } else if let Some((name, fields)) = &self.maybe_struct_init(&bin_op, &None) {
            self.struct_init(name, fields); 
        } else {    
            self.expr(&bin_op.l_opr);
            self.rs_bin_op(&bin_op.op);
            self.expr(&bin_op.r_opr);
        }
    }
}

//================
//  fix_import_path()
//================  
impl <'a> Rust<'a> {     
    fn fix_import_path(
        &mut self,
        bin_op: &BinOp
    ) {
        self.expr(&bin_op.l_opr);
        let _ = write!(self.res, "::");
        self.expr(&bin_op.r_opr);
    }
}

//================
//  temp_std()
//================  
impl <'a> Rust<'a> {     
    pub fn temp_std(
        &mut self, 
        name: &String, 
        args: &Tuple
    ) {
        match name.as_str() {
            // "احضر"          | "import"      => self.import(args),    // FIXME: this is hardcoded and handled inside declare for the moment
            "اطبع_سطر"      | "println"     => self.println(args),
            "اطبع"          | "print"       => self.print(args),
            "مخدم_شع"       | "web_view"    => self.web_view(args),
            "mobile_view" => self.mobile_view(args),
            "gui_view" => self.gui_view(args),
            // _ => panic!("could not resolve: `{}`", name)
            _ => self.user_defined_fn(&name, args)

        }
    }
}

//================
//  user_defined_fn()
//================  
impl <'a> Rust<'a> {   
    pub fn user_defined_fn(
        &mut self,
        name: &String,
        args: &Tuple
    ) { 
        let _ = write!(self.res, "{}", name); 
        self.tuple(args);
    }
}
//================
//  import()
//================  
// FIXME: hardcoded: assuming that we have one argument passed to import(), that will either represent a file.rs or a file.seen / .س
//          that exists in the same directory as main.seen
impl <'a> Rust<'a> {   
    pub fn import(
        &mut self,
        pattern: &Pattern,
        args: &Tuple
    ) { 

        let _mod = &args.items[0];   // FIXME hardcoded / no error handling
        let _mod = format!("{}", _mod);
 
        // FIXME : Hardcoded extensions, 
        // FIXME : what if the seen file is not transpiled to rust?         
        let _mod = if str::ends_with(&_mod, ".rs") { 
            _mod.strip_suffix(".rs").unwrap()
        } else if str::ends_with(&_mod, ".seen") {
            _mod.strip_suffix(".seen").unwrap()
        } else if str::ends_with(&_mod, ".س")  {    
            _mod.strip_suffix(".س").unwrap()
        } else {    // FIXME: other cases such as images, other pls ...etc are not covered. assuming a filename without extension ( also a seen file)
            _mod.as_str()
        };

        

        let _ = writeln!(self.res, "#[path = \"{}.rs\"]", _mod);
        if let Pattern::Id(id_pat) = pattern {
            self.imports.push(id_pat.id.to_string());
            let _ = writeln!(self.res, "mod {};", id_pat.id);    
        } else {
            todo!("other patterns are not supported yet");  // FIXME
        }
        

    }
}

//================
//  is_import_module()
//================  
impl <'a> Rust<'a> {   
    pub fn is_imported_module(
        &mut self,
        expr: &Expr,
    ) -> bool { 
        match expr {
            Expr::Ref(_ref) => {
                return self.imports.contains(&_ref.value.to_string());
            }
            _ =>  false // TODO : currently, only Id Pattern is supported
        }
    }
}

//================
//  println()
//================  

        // FIXME,  no need for this function, call _macro() instead , lookup builtin["println!"].
impl <'a> Rust<'a> {   
    pub fn println(
        &mut self,
        args: &Tuple
    ) { 
        let _ = write!(self.res, "println!"); 
        self.print_args(args);
    }
}


//================
//  print()
//================  
impl <'a> Rust<'a> {   
    pub fn print(
        &mut self,
        args: &Tuple
    ) { 
        let _ = write!(self.res, "print!"); 
        self.print_args(args);
    }
}

//================
//  print_args()
//================  
// FIXME, println! / print! can have a variable number of args, currently , this method only prints a single value   
impl <'a> Rust<'a> {   
    pub fn print_args(
        &mut self,
        args: &Tuple
    ) { 
        let _ = write!(self.res, "(\"{{}}\",");    
        for (i, item) in args.items.iter().enumerate() {
            self.expr(&item);
            if i < args.items.len() - 1 {
                let _ = write!(self.res, ", ");
            }
        }
        let _ = write!(self.res, ")");    



    }
}



//================
//  list()
//================  
impl <'a> Rust<'a> {   
    pub fn list(
        &mut self,
        list: &List
    ) {
        // FIXME, formatting lists / tuples with big expressions on multiple lines
        // FIXME, vec![] is one of many cases for creating a list
        let _ = write!(self.res, "vec![");    
        for (i, item) in list.items.iter().enumerate() {
            self.expr(&item);
            if i < list.items.len() - 1 {
                let _ = write!(self.res, ", ");
            }
        }
        let _ = write!(self.res, "]");    
        
    }
}

//================
//  tuple()
//================  
impl <'a> Rust<'a> {   
    pub fn tuple(
        &mut self,
        tuple: &Tuple
    ) {
        let _ = write!(self.res, "(");    
        for (i, item) in tuple.items.iter().enumerate() {
            self.expr(&item);
            if i < tuple.items.len() - 1 {
                let _ = write!(self.res, ", ");
            }
        }
        let _ = write!(self.res, ")");    
        
    }
}

//================
//  block()
//================  
impl <'a> Rust<'a> {   
    pub fn block(
        &mut self,
        block: &Vec<BlockElement>
    ) {
        let _ = writeln!(self.res, " {{",);
        self.indent.inc();

        for (_,el) in block.iter().enumerate() {
            let _ = write!(self.res, "{}", self.indent);
            self.block_element(el);
            let _ = writeln!(self.res, ";");    // FIXME, no ; if return
        } 

        self.indent.dec();
        let _ = write!(self.res, "{}}}", self.indent);
    }
}

//================
//  _match()
//================  
impl <'a> Rust<'a> {   
    pub fn _match(
        &mut self,
        _match: &Match
    ) { 

        let _ = write!(self.res, "{}match ",self.indent); 
        self.expr(&_match.expr);
        let _ = writeln!(self.res, " {{",);
        self.indent.inc();
        for (i, arm) in _match.arms.iter().enumerate() {
            self.arm(&arm.pattern, &arm.block);
            if i < _match.arms.len() - 1 { 
                let _ = writeln!(self.res, ",",);
            } else {
                let _ = writeln!(self.res, "");
            }
        }
        self.indent.dec();
        let _ = write!(self.res, "{}}}", self.indent);
 
    }
}

//================
//  arm()
//================  
impl <'a> Rust<'a> {   
    pub fn arm(
        &mut self,
        pattern: &Pattern,
        block: &Vec<BlockElement>
    ) { 
        let _ = write!(self.res, "{}",self.indent ); 
        self.pattern(&pattern);
        let _ = write!(self.res,  "=> ");
        if block.len() == 1 {
                self.block_element(&block[0]);
        } else {
            self.block(&block);
        }

    }
}

//================
// _for()
//================
impl<'a> Rust<'a> {
    fn _for(
        &mut self,    
        _for: &For,
    ) {
        let _ = write!(self.res, "for ",); 
        self.in_expr(&_for.in_expr);
        self.block(&_for.block);
    }
}

//================
// in_expr()
//================
impl<'a> Rust<'a> {
    fn in_expr(
        &mut self,    
        in_expr: &InExpr,
    ) {
        self.pattern(&in_expr.pattern);
        let _ = write!(self.res, " in ",); 
        self.expr(&in_expr.expr);
    }
}

//================
// _while()
//================
impl<'a> Rust<'a> {
    fn _while(
        &mut self,    
        _while: &While,
    ) {
        let _ = write!(self.res, "while "); 
        self.expr(&_while.expr);
        self.block(&_while.block);
    }
}

//================
// _if()
//================
impl<'a> Rust<'a> {
    fn _if(
        &mut self,    
        _if: &If,
    ) {
        for (i, branch) in _if.branches.iter().enumerate() {
            if i == 0 {
                let _ = write!(self.res, "if ");   
                let expr = branch.expr.as_ref().unwrap().clone();
                self.expr(&expr);
                self.block(&branch.block);
            } else {
                if branch.expr.is_none() {
                    let _ = write!(self.res, " else ");         
                    self.block(&branch.block);
                } else {
                    let _ = write!(self.res, " else if ");
                    self.expr(&branch.expr.as_ref().unwrap().clone());
                    self.block(&branch.block);                    
                }
            }
        }
        
    }
}


//================
//  pattern()
//================  
impl <'a> Rust<'a> {   
    pub fn pattern(
        &mut self,
        pattern: &Pattern,
    ) { 

        match pattern {
            Pattern::PrimitiveLiteral(expr) => {
                self.expr(&expr);
            },
            Pattern::Id(IdPattern{id}) => {
                let _ = write!(self.res, "{}", id);
            },          
            Pattern::List(list_pat) => todo!(),
            Pattern::Tuple(tuple_pat) => todo!(),
            Pattern::Struct(struct_pat) => todo!(),
            Pattern::Enum(enum_pat) => todo!(),
            Pattern::Wildcard  => {
                let _ = write!(self.res, "_");
            }
        }

    }
}

//================
//  block_element()
//================  
impl <'a> Rust<'a> {   
    pub fn block_element(
        &mut self,
        el: &BlockElement,
    ) { 
        match el {
            BlockElement::MainArgs => {
                self.main_args()
            }
            BlockElement::Decl(decl) => {
                self.decl(&decl);
            },
            BlockElement::Expr(expr) => self.expr(&expr)
        }
    }
}

//================
//  main_args()
//================  
impl <'a> Rust<'a> {   
    pub fn main_args(
        &mut self,
    ) { 
        let _ = writeln!(self.res, "let args: Vec<String> = std::env::args().collect();");         
    }
}

//================
//  decl()
//================  
impl <'a> Rust<'a> {   
    pub fn decl(
        &mut self,
        decl: &Decl

    ) { 
        if self.expect_import(decl) {
            if let Some(Expr::BinOp(bin_op)) = &decl.expr {
                if let Expr::Tuple(args) = &*bin_op.r_opr {
                    self.import( &decl.pattern ,&args);
                }
            }
        } else {
            let _ = write!(self.res, "let ");         
            let _ = write!(self.res, "mut ");    // FIXME, for now everything will be treated as mut
            self.pattern(&decl.pattern);
            match &decl._type {
                None => (),
                Some(_type) => self.type_annotation(&_type)
            }
            
            if let Some(expr) = &decl.expr {
                let _ = write!(self.res, " = ");
                self.expr(&expr);    
            }
            let _ = write!(self.res, ";");
        }
    }
}

//================
//  expect_import()
//================  
impl <'a> Rust<'a> {   
    pub fn expect_import(
        &mut self,
        decl: &Decl
    ) -> bool { 
        // FIXME: quick workaround, check if import func call is next and handle it as an import 
        //  this is more like hardcoding, what if we have more complex calls like this?
        //          io := filter(import("std"), {io})
        if let Some(Expr::BinOp(bin_op)) = &decl.expr {
            if let expr = &*bin_op.l_opr {
                if let Expr::Ref(t) = expr {
                    if t.value.to_string() == "import" || t.value.to_string() == "احضر" { // FIXME: hardcoded translations, should use enums instead
                        return true;
                    }
                }
            }
        }
        false    
    }
}

//================
//  web_server()
//================  
impl <'a> Rust<'a> { 
    pub fn  web_server(
        &mut self,
        data: &StructLiteral
    ) {
        let mut path = self.proj_dir.res.pages.clone();
        path.push(INDEX_HTML);
        let index_html = self.html
                        .as_mut()
                        .unwrap()
                        .page(&mut path, data);
        self.actix(&path, data);

    }
}

//================
//  actix()
//================  
impl <'a> Rust<'a> {
    pub fn  actix(
        &mut self,
        path: &PathBuf,
        data: &StructLiteral

    ) {

    let actix_web = ActixWeb::new();
    let actix_file = ActixFiles::new();

    self.cargo_toml.add(actix_file);
    self.cargo_toml.add(actix_web);

    let res_dir = match self.src_lang {
        Lang::Ar => "موارد",
        Lang::En => "res"
    };

    let pages_dir = match self.src_lang {
        Lang::Ar => "صفحات",
        Lang::En => "pages"
    };    

    let server_start_msg = match self.src_lang {
        Lang::Ar => "لقد تم تشغيل المخدم , العنوان : ",
        Lang::En => "server started: "
    };

    let mut settings = None;
    
    let iter = if let Some((t, Some(expr))) = data.items.get(0) {

        let mut iter = data.items.iter();
        match &t.value  {
            TokenValue::Id(x) => {
                if x == "data" ||  x == "root" || x == "بيانات" || x == "جذر" {
                    match expr {
                        Expr::StructLiteral(literal) => {
                            iter = literal.items.iter();
                        }
                        _ => ()
                    }    
                }
            },
            _ => ()
        }
        iter

    } else {
        data.items.iter()
    };


    for (k,v) in iter {
        match k.to_string().as_str() {
            "settings" | "اعدادات" => {
                settings = self.server_settings(v);
                break;
            },
            _ => panic!("expecting server settings")
        }
    };

    let settings = settings.expect("expecting server settings");


    let hostname = match self.src_lang {
        Lang::Ar => {
            match settings.hostname.as_str() {
                "المضيف_المحلي" => "localhost".to_string(),
                x => to_western_num(&x.to_string())
            }
        },
        Lang::En => settings.hostname
    };

    let port = match self.src_lang {
        Lang::Ar => to_western_num(&settings.port.to_string()),
        Lang::En => settings.port
    };


    

    // FIXEME, hardcoding the example for demo
    let _ = write!(self.res, 
r#"use actix_web::{{App, HttpServer}};
use actix_files::Files;

const HOSTNAME: &str = "{hostname}";
const PORT: u32 = {port};

#[actix_web::main]
async fn main() -> std::io::Result<()> {{
    let addr = format!("{{}}:{{}}", HOSTNAME, PORT);
    let server = HttpServer::new(move || {{
        App::new()            
            .service(Files::new("/", "./{res_dir}/{pages_dir}").index_file("index.html"))  

    }});

    println!("{server_start_msg}\n\t\thttp://{{}}", addr);
    server
    .bind(addr)?
    .run()
    .await
}}"#);
        
        match fs::write(&self.proj_dir.src.main, &self.res){
            Err(err) => panic!("{:?}", err),
            Ok(_) => ()
        }
    }
}

//================
//   ServerSettings()
//================
pub struct ServerSettings {
    pub hostname: String,
    pub port: String
}

impl ServerSettings {
    pub fn new() -> Self {
        Self {
            hostname : String::new(),
            port: String::new()
        }
    }
}

//================
//   server_settings()
//================
impl <'a> Rust<'a> {
    pub fn server_settings (
        &mut self,
        data: &Option<Expr>
    ) -> Option<ServerSettings> {
        let mut settings = ServerSettings::new();

        let data = match data {
            Some(Expr::StructLiteral(sruct_literal)) => sruct_literal,
            _ => return None
        };

        for (k,v) in data.items.iter() {
            match k.to_string().as_str() {
                "hostname" | "اسم_المضيف"=> {
                    
                    if let Some(v) = v { 
                        match v {
                            Expr::Str(v) => settings.hostname = v.value.to_string(),
                            _ => panic!("unexpected hostname value")
                        }
                    }
                    
                },
                "port" | "منفذ"=> {
                    if let Some(v) = v { 
                        match v {
                            Expr::Int(v) => settings.port = v.value.to_string().parse().expect("port should be a number"),
                            _ => panic!("unexpected port value")
                        }
                    }                    
                },
                _ => panic!("unsupported: {:?}", k)
            }
        }
        Some(settings)
    }
}


//================
//  web_view()
//================  
impl <'a> Rust<'a> {
    pub fn web_view(
        &mut self,
        args: &Tuple        
    ) {
        todo!();
    }
}


//================
//  mobile_view()
//================  
impl <'a> Rust<'a> { 
    pub fn mobile_view(
        &mut self,
        args: &Tuple        
    ) {
        todo!();
    }
}

//================
//  gui_view()
//================  
impl <'a> Rust<'a> {   
    pub fn gui_view(
        &mut self,
        args: &Tuple        
    ) {
        todo!();
    }
}



