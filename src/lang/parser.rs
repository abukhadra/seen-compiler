use std::collections::HashMap;

use super::{
    token::{
        Token, 
        TokenValue
    },
    operator,
    syntax_tree::ast::*,
    symtab::SymTab,
    error::{
        Error,
    },
};

use crate::error;

//======================
//  expect!()
//======================
macro_rules! expect {
    
    (&$self:ident, $pattern:pat_param) => {
        matches!($self.lookahead().value, $pattern)
    };
}

//======================
//  require!()
//======================
macro_rules! require {
    (&$self:ident, $pattern:pat_param, $msg:expr) => {
        if expect!(&$self, $pattern) {
            let t= $self.next();
            Ok(t)
        } else {
            let t= $self.lookahead();
            Err(error!($msg, t))
        }    
    };
}

//================
// Parser
//================
pub struct Parser<'a> {
    token_index: i32,
    tokens: Option<&'a Vec<Token>>,
    ast: Option<Vec<ModElement>>,
    symtab: Option<SymTab>,
    errors: Option<Vec<Error>>,
    // indents: Vec<Token> 
} // TODO

impl<'a> Parser<'a>{
    //---------------------
    //  new()
    //---------------------    
    pub fn new() -> Self {
        Self {
            token_index: -1,
            tokens: None,
            ast: None,
            symtab: None,
            errors: None,
            // indents: vec![]
        }
    }

    //---------------------
    //  init()
    //---------------------    
    pub fn init(
        &mut self,
        tokens: &'a Vec<Token>,    
    ) {
        self.token_index = -1;
        self.tokens = Some(tokens);
        self.ast = Some(vec![]);
        self.symtab = Some(SymTab::new());
        self.errors = Some(vec![]);
        // self.indents = vec![];   
        
    }    

    //---------------------
    //  parse()
    //---------------------
    pub fn parse(
        &mut self,
        tokens: &'a Vec<Token>,        
    ) -> (Vec<ModElement>, SymTab, Vec<Error>) {
        self.init(tokens);
        while !self.expect_eof() {
            let attrs = self.maybe_attrs(); // FIXME: should test if Some(attrs) and test only for constructs that accept/require attrs, then in the else test for the ones that don't

            let t = self.lookahead();
            // self.indents.push(t);
            if let Some(decl) = self.maybe_short_import() { // import("..", ...)  <===>   name := import("...", name)
                self.mod_insert(ModElement::Decl(decl))
            } else if let Some(decl) = self.maybe_let_decl() {
                self.mod_insert(ModElement::Decl(decl));
            } else if let Some(e) = self.maybe_lambda_or_decl(&attrs) {
                match e {
                    LambdaOrDecl::Lambda(_fn) => self.mod_insert(ModElement::MainFn(_fn)),
                    LambdaOrDecl::Decl(decl) => self.mod_insert(ModElement::Decl(decl))
                }
                
            } else if self.expect_id() {
                let id = self.next();
                if let Some(_fn) = self.maybe_fn(Some(&id), &attrs, false) {    // TODO instead of passing a boolean to indicate method/func , split it to maybe_fn() / maybe_method() for readability
                    let _ = self.symtab().insert_fn(&_fn);
                    self.mod_insert(ModElement::Fn(_fn));    
                } else if let Some(e) = self.maybe_struct(&id, &attrs) {
                    self.mod_insert(ModElement::Struct(e));
                } else if let Some(e) = self.maybe_struct_impl(&id, &attrs) {
                    self.mod_insert(ModElement::StructImpl(e));
                } else if let Some(e) = self.maybe_enum_impl(&id) {
                    self.mod_insert(ModElement::EnumImpl(e));
                }  else  if let Some(decl) = self.maybe_short_decl(Some(&id)) {
                    let _ = self.symtab().insert_decl(&decl);
                    self.mod_insert(ModElement::Decl(decl));
                } else {
                    self.insert_err(
                        error!(format!("expecting a function or a declaration: {:?}", id.value), id)
                    );
                }
                
            } else if let Some(e) = self.maybe_trait() {
                self.mod_insert(ModElement::Trait(e));
            } else if let Some(e) = self.maybe_enum() {
                self.mod_insert(ModElement::Enum(e));
            } else {                    
                let t = self.lookahead();

                if let Some(attrs) = attrs {
                    self.insert_err(
                        error!(format!("expecting  a function,  or a struct after attributes: {:?}", attrs), t)
                    );    
                }

                self.insert_err(
                    error!(format!("unknown token: {:?}", t.value), t)
                );
                break;
            };
            // self.indents.pop();
        }

        (
            self.ast.take().unwrap(),
            self.symtab.take().unwrap(),
            self.errors.take().unwrap()
        )
    }

    //---------------------
    //  symtab()
    //---------------------    
    fn symtab(&mut self) -> &mut SymTab {
        self.symtab.
            as_mut()
            .unwrap()
    }

    //---------------------
    //  token()
    //---------------------    
    fn token(
        &mut self,
        i: usize
    ) -> Token {
        self.tokens
            .unwrap()
            .get(i)
            .unwrap()
            .clone()        
    }

    //---------------------
    //  current()
    //---------------------    
    fn current(&mut self) -> Token { 
        self.token(self.token_index as usize) 
    }    

    //---------------------
    //  next()
    //---------------------    
    fn next(&mut self) -> Token {
        self.token_index += 1;
        let t = self.token(self.token_index as usize);

        match t.value {
            TokenValue::NewLine => {
                self.next()
            },
            _ => t            
        }
    }

    //---------------------
    //  lookahead()
    //---------------------    
    fn lookahead(&mut self) -> Token {

        let t = self.lookahead_n_ws(1);

        let t = match t.value {

            TokenValue::NewLine => {
                self.lookahead_n_ws(2)
            },
            _ => t
        };                    
        t
    }

    //---------------------
    //  lookahead_n_ws()
    //---------------------    
    fn lookahead_n_ws(
        &mut self, 
        n: i32
    ) -> Token {
        let lookahead_index = (self.token_index + n) as usize;
        let max_index = self.tokens.unwrap().len() - 1;
        if lookahead_index > max_index { 
            return self.token(max_index )
        }

        self.token(lookahead_index)
    }    

    //---------------------
    //  expect_let()
    //---------------------    
    fn expect_let(&mut self) -> bool { expect!(&self, TokenValue::Let) }

    //---------------------
    //  expect_in()
    //---------------------    
    fn require_in(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::In, 
            String::from("expecting `in`")
        )
    }

    //---------------------
    //  expect_caret()
    //---------------------    
    fn expect_caret(&mut self) -> bool { expect!(&self, TokenValue::Caret) }

    //---------------------
    //  expect_where()
    //---------------------    
    fn expect_where(&mut self) -> bool { expect!(&self, TokenValue::Where) }

    //---------------------
    //  expect_id()
    //---------------------    
    fn expect_id(&mut self) -> bool { expect!(&self, TokenValue::Id(_)) }


    //---------------------
    //  require_id()
    //---------------------    
    fn require_id(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::Id(_), 
            String::from("expecting an id")
        )
    }        

    //---------------------
    //  expect_code()
    //---------------------    
    fn expect_code(&mut self) -> bool { expect!(&self, TokenValue::Code(_)) }

    //---------------------
    //  expect_bool()
    //---------------------    
    fn expect_bool(&mut self) -> bool { expect!(&self, TokenValue::Bool(_)) }

    //---------------------
    //  expect_char()
    //---------------------    
    fn expect_char(&mut self) -> bool { expect!(&self, TokenValue::Char(_)) }

    //---------------------
    //  expect_str()
    //---------------------    
    fn expect_str(&mut self) -> bool { expect!(&self, TokenValue::Str(_)) }


    //---------------------
    //  expect_int()
    //---------------------    
    fn expect_int(&mut self) -> bool { expect!(&self, TokenValue::Int(_)) }

    //---------------------
    //  expect_float()
    //---------------------    
    fn expect_float(&mut self) -> bool { expect!(&self, TokenValue::Float(_)) }    

    //---------------------
    //  expect_res()
    //---------------------    
    fn expect_res(&mut self) -> bool { expect!(&self, TokenValue::Res) }

    //---------------------
    //  expect_ok()
    //---------------------    
    fn expect_ok(&mut self) -> bool { expect!(&self, TokenValue::Ok) }

    //---------------------
    //  expect_err()
    //---------------------    
    fn expect_err(&mut self) -> bool { expect!(&self, TokenValue::Err) }

    //---------------------
    //  expect_some()
    //---------------------    
    fn expect_some(&mut self) -> bool { expect!(&self, TokenValue::Some) }

    //---------------------
    //  expect_none()
    //---------------------    
    fn expect_none(&mut self) -> bool { expect!(&self, TokenValue::None) }    

    //---------------------
    //  expect_equal()
    //---------------------    
    fn expect_equal(&mut self) -> bool { expect!(&self, TokenValue::Equal) }

    //---------------------
    //  require_equal()
    //---------------------    
    fn require_equal(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::Equal, 
            String::from("expecting `=`")
        )
    }

    //---------------------
    //  expect_question()
    //---------------------    
    fn expect_question(&mut self) -> bool { expect!(&self, TokenValue::Question) }

    //---------------------
    //  expect_exclamation()
    //---------------------    
    fn expect_exclamation(&mut self) -> bool { expect!(&self, TokenValue::Exclamation) }

    
    //---------------------
    //  require_decl_asign()
    //---------------------    
    fn require_decl_asign(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::DeclAsign, 
            String::from("expecting `:=`")
        )
    }    

    //---------------------
    //  expect_underscore()
    //---------------------    
    fn expect_underscore(&mut self) -> bool { expect!(&self, TokenValue::Underscore) }

    //---------------------
    //  expect_hash()
    //---------------------    
    fn expect_hash(&mut self) -> bool { expect!(&self, TokenValue::Hash) }

    //---------------------
    //  expect_bar()
    //---------------------    
    fn expect_bar(&mut self) -> bool { expect!(&self, TokenValue::Bar) }

    //---------------------
    //  expect_dot()
    //---------------------    
    fn expect_dot(&mut self) -> bool { expect!(&self, TokenValue::Dot) }

    //---------------------
    //  require_dot()
    //---------------------    
    fn require_dot(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::Dot, 
            String::from("expecting `.`")
        )
    }

    //---------------------
    //  expect_double_colon()
    //---------------------    
    fn expect_double_colon(&mut self) -> bool { expect!(&self, TokenValue::DoubleColon) }    

    //---------------------
    //  expect_at()
    //---------------------    
    fn expect_at(&mut self) -> bool { expect!(&self, TokenValue::At) }


    //---------------------
    //  expect_thin_arrow()
    //---------------------    
    fn expect_thin_arrow(&mut self) -> bool { expect!(&self, TokenValue::ThinArrow) }

    //---------------------
    //  require_thin_arrow()
    //---------------------    
    fn require_thin_arrow(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::ThinArrow, 
            String::from("expecting `->`")
        )
    }

    //---------------------
    //  expect_arrow()
    //---------------------    
    // fn expect_arrow(&mut self) -> bool { expect!(&self, TokenValue::Arrow) }

    //---------------------
    //  require_arrow()
    //---------------------    
    fn require_arrow(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::Arrow, 
            String::from("expecting `=>`")
        )
    }

    //---------------------
    //  expect_open_bracket()
    //---------------------    
    fn expect_open_bracket(&mut self) -> bool { expect!(&self, TokenValue::OpenBracket) }
    
    //---------------------
    //  require_open_bracket()
    //---------------------    
    fn require_open_bracket(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::OpenBracket, 
            String::from("expecting `[`")
        )
    }

    //---------------------
    //  expect_close_bracket()
    //---------------------    
    fn expect_close_bracket(&mut self) -> bool { expect!(&self, TokenValue::CloseBracket) }

    //---------------------
    //  require_close_bracket()
    //---------------------    
    fn require_close_bracket(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::CloseBracket, 
            String::from("unclosed, expecting `]`")
        )
    }

    //---------------------
    //  expect_open_curly()
    //---------------------    
    fn expect_open_curly(&mut self) -> bool { expect!(&self, TokenValue::OpenCurly) }


    //---------------------
    //  require_open_curly()
    //---------------------    
    fn require_open_curly(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::OpenCurly,
            String::from("missing {")
        )
    }

    //---------------------
    //  expect_close_curly()
    //---------------------    
    fn expect_close_curly(&mut self) -> bool { expect!(&self, TokenValue::CloseCurly) }


    //---------------------
    //  require_close_curly()
    //---------------------    
    fn require_close_curly(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::CloseCurly, 
            String::from("unclosed, expecting `}}`")
        )
    }    

    //---------------------
    //  expect_open_angle()
    //---------------------    
    fn expect_open_angle(&mut self) -> bool { expect!(&self, TokenValue::LT) }


    //---------------------
    //  require_open_angle()
    //---------------------    
    fn require_open_angle(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::LT,
            String::from("missing `<`")
        )
    }

    //---------------------
    //  expect_close_angle()
    //---------------------    
    fn expect_close_angle(&mut self) -> bool { expect!(&self, TokenValue::GT) }


    //---------------------
    //  require_close_angle()
    //---------------------    
    fn require_close_angle(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::GT, 
            String::from("unclosed, expecting `>`")
        )
    }    

    //---------------------
    //  expect_open_paren()
    //---------------------    
    fn expect_open_paren(&mut self) -> bool { expect!(&self, TokenValue::OpenParen) }

    //---------------------
    //  require_open_paren()
    //---------------------    
    fn require_open_paren(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::OpenParen, 
            String::from("expecting `(`")
        )
    }    
    
    //---------------------
    //  expect_close_paren()
    //---------------------    
    fn expect_close_paren(&mut self) -> bool { expect!(&self, TokenValue::CloseParen) }


    //---------------------
    //  require_close_paren()
    //---------------------    
    fn require_close_paren(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::CloseParen, 
            String::from("unclosed, expecting `)`")
        )
    }    


    //---------------------
    //  expect_comma()
    //---------------------    
    fn expect_comma(&mut self) -> bool { expect!(&self, TokenValue::Comma) }

    //---------------------
    //  optional_comma()
    //---------------------    
    fn optional_comma(&mut self) {
        if self.expect_comma() { self.next(); }
    }

    //---------------------
    //  expect_colon()
    //---------------------    
    fn expect_colon(&mut self) -> bool { expect!(&self, TokenValue::Colon) } 

    //---------------------
    //  require_colon()
    //---------------------    
    fn require_colon(&mut self) -> Result<Token, Error> { 
        require!(
            &self, 
            TokenValue::Colon, 
            String::from("expecting `:` `}}`")
        )
    }    

    //---------------------
    //  expect_new_line()
    //---------------------    
    fn expect_new_line(&mut self) -> bool {
        let t = self.lookahead_n_ws(1);

        match t.value {
            TokenValue::NewLine => true,
            _ => false
        }
    }            

    //---------------------
    //  expect_semicolon()
    //---------------------    
    fn expect_semicolon(&mut self) -> bool { expect!(&self, TokenValue::Semicolon) }

    //---------------------
    //  optional_semicolon()
    //---------------------    
    fn optional_semicolon(&mut self) {
        if self.expect_semicolon() { self.next(); }
    }  

    //---------------------
    //  expect_terminator()
    //---------------------    
    fn expect_terminator(&mut self) -> bool {
        let t = self.lookahead_n_ws(1);
        match t.value {
            TokenValue::NewLine 
            | TokenValue::Eof 
            |  TokenValue::Semicolon  => true,
            _ => false
        }
    }          
    
    //---------------------
    //  require_terminator()
    //---------------------    
    fn require_terminator(&mut self) -> Result<(), Error>{
        if !self.expect_terminator() {
            let t = self.lookahead();
            Err(error!( format!("expecting a new line or `;` after the expression "), t ))
        } else {
            self.optional_semicolon();
            Ok(())
        }
    }    

    //---------------------
    //  expect_prefix_uni_op()
    //---------------------    
    fn expect_prefix_uni_op(&mut self) -> bool {
        expect!(&self, TokenValue::Minus) 
        || expect!(&self, TokenValue::Exclamation) 
        || expect!(&self, TokenValue::Minus) 
    }

    //---------------------
    //  expect_postfix_uni_op()
    //---------------------    
    fn expect_postfix_uni_op(&mut self) -> bool {
        expect!(&self, TokenValue::Question) 
        || expect!(&self, TokenValue::Exclamation)
    }

    //---------------------
    //  expect_bin_op()
    //---------------------    
    fn expect_bin_op(&mut self) -> bool {
    
        expect!(&self, TokenValue::Add) 
        || expect!(&self, TokenValue::Sub)
        || expect!(&self, TokenValue::Mul)
        || expect!(&self, TokenValue::Div)
        || expect!(&self, TokenValue::Perc)
        || expect!(&self, TokenValue::Hash)
        || expect!(&self, TokenValue::Tilde)
        || expect!(&self, TokenValue::Caret)
        // || expect!(&self, TokenValue::ArgList)
        // || expect!(&self, TokenValue::Index)
        // || expect!(&self, TokenValue::OpenCurly)
        || self.expect_struct_literal()
        || expect!(&self, TokenValue::OpenParen)
        || expect!(&self, TokenValue::OpenBracket)
        || expect!(&self, TokenValue::Equal)
        || expect!(&self, TokenValue::AddEqual)
        || expect!(&self, TokenValue::SubEqual)
        || expect!(&self, TokenValue::MulEqual)
        || expect!(&self, TokenValue::DivEqual)
        || expect!(&self, TokenValue::BitwiseOrEqual)
        || expect!(&self, TokenValue::BitwiseAndEqual)
        || expect!(&self, TokenValue::BitwiseXorEqual)
        || expect!(&self, TokenValue::PipeForward)
        || expect!(&self, TokenValue::Eq)
        || expect!(&self, TokenValue::NE)
        || expect!(&self, TokenValue::GT)
        || expect!(&self, TokenValue::GE)
        || expect!(&self, TokenValue::LT)
        || expect!(&self, TokenValue::LE)
        || expect!(&self, TokenValue::LogicalAnd)
        || expect!(&self, TokenValue::LogicalAnd)
        || expect!(&self, TokenValue::BitwiseOr)
        || expect!(&self, TokenValue::BitwiseAnd)
        || expect!(&self, TokenValue::BitwiseXor)       
        || expect!(&self, TokenValue::LogicalOr)
        || expect!(&self, TokenValue::Dot)
        || expect!(&self, TokenValue::Dollar)
        || expect!(&self, TokenValue::At)

    } 

    //---------------------
    //  expect_index()
    //---------------------    
    fn expect_index(&mut self) -> bool {
        // expect!(&self, TokenValue::Index) 
        expect!(&self, TokenValue::OpenBracket) 
    }

    //---------------------
    //  require_index()
    //---------------------    
    fn require_index(&mut self) -> Result<Token, Error> {
        require!(
            &self, 
            // TokenValue::Index, 
            TokenValue::OpenBracket,
            String::from("expecting  an index `[...]` `}}`")
        )
    }    

    //---------------------
    //  expect_arg_list()
    //---------------------        
    fn expect_arg_list(&mut self) -> bool {
        // expect!(&self, TokenValue::ArgList)
        expect!(&self, TokenValue::OpenParen)
    }

    //---------------------
    //  expect_struct_literal()
    //---------------------        
    fn expect_struct_literal(&mut self) -> bool {
        // expect!(&self, TokenValue::OpenCurly)
        self.is_struct_literal()        
    }
    

    //---------------------
    //  require_arg_list()
    //---------------------        
    fn require_arg_list(&mut self) -> Result<Token, Error> {
        require!(
            &self, 
            // TokenValue::ArgList, 
            TokenValue::OpenParen, 
            String::from("expecting  args `id(...)` `}}`")
        )
    }   

    //---------------------
    //  expect_match()
    //---------------------    
    fn expect_match(&mut self) -> bool { expect!(&self, TokenValue::Match) }

    //---------------------
    //  expect_for()
    //---------------------    
    fn expect_for(&mut self) -> bool { expect!(&self, TokenValue::For) }

    //---------------------
    //  expect_while()
    //---------------------    
    fn expect_while(&mut self) -> bool { expect!(&self, TokenValue::While) }

    //---------------------
    //  expect_if()
    //---------------------    
    fn expect_if(&mut self) -> bool { expect!(&self, TokenValue::If) }

    //---------------------
    //  expect_else()
    //---------------------    
    fn expect_else(&mut self) -> bool { expect!(&self, TokenValue::Else) }    

    //---------------------
    //  expect_eof()
    //---------------------    
    fn expect_eof(&mut self) -> bool { expect!(&self, TokenValue::Eof) }

    //---------------------
    //  mod_insert()
    //---------------------        
    fn mod_insert(
        &mut self,
        e: ModElement
    ) { 
        self.ast
            .as_mut()
            .unwrap()
            .push(e);
    }

    //---------------------
    //  insert_err()
    //---------------------        
    fn insert_err(
        &mut self, 
        error: Error
    ) {
        self.errors.as_mut().unwrap().push(error);
    }    

    //---------------------
    //  res_to_opt()
    //---------------------        
    fn res_to_opt<T>(
        &mut self, 
        res: Result<T, Error>
    ) -> Option<T> {

        match res {
            Err(err) => {
                self.insert_err(err.to_owned());
                None
            },
            Ok(v) => {
                Some(v)
            }
        }
    }        

}

//================
// maybe_attrs()
//================
impl<'a> Parser<'a> {
    pub fn maybe_attrs(&mut self) -> Option<Vec<Attr>> {
        let mut attrs = vec![];
        while self.expect_at() {
            let attr = self.require_attr();
            let attr = self.res_to_opt(attr)?;
            attrs.push(attr);
        }
        if attrs.is_empty() { None } else { Some(attrs) }
    }
}

//================
// require_attr()
//================
impl<'a> Parser<'a> {
    pub fn require_attr(&mut self) -> Result<Attr,Error> {
        if !self.expect_at() { return Err(error!("expecting `@`".to_string(), self.lookahead())) }

        self.next();
        if let Some(id) = self.maybe_id() {
            Ok(Attr{ expr: AttrExpr::Ref(id) })
        } else {
            Err(error!("expecting an identifier after `@`".to_string(), self.lookahead()))
        }

    }
}

//================
// optional_type_annotation()
//================
impl<'a> Parser<'a> {
    pub fn optional_type_annotation (
        &mut self,
    ) -> Option<Type>{
        if !self.expect_colon() { return None }
        self.next();
        let _type = self.require_type();
        let _type = self.res_to_opt(_type)?;
        Some(_type)
    }
}

//================
// require_type()
//================
impl<'a> Parser<'a> {
    pub fn require_type (&mut self) -> Result<Type, Error>{
        let t = self.lookahead();
        let _type = if let Some(_type) = self.maybe_result_type() {
            Some(Type::ResultType(Box::new(_type)))
        } else if let Some(_type) = self.maybe_primitive_type() {
            Some(Type::PrimitiveType(_type))
        } else if let Some(_type) = self.maybe_list_type() {
            Some(Type::ListType(_type))
        } else if let Some(_type) = self.maybe_unit_or_tuple_type() {
            match _type {
                UnitOrTupleType::Unit => Some(Type::UnitType),
                UnitOrTupleType::Tuple(_type) => Some(Type::TupleType(_type))
            }
        } else if let Some(_type) = self.maybe_struct_type() {
            Some(Type::StructType(_type))
        } else {
            None
        };

        match _type {
            None => Err(error!("expecting a type".to_string(), t)),
            Some(_type) => {
                if let Some(_type) = self.maybe_option_type(&_type)  {
                    Ok(Type::OptionType(Box::new(_type)))
                } else {
                    Ok(_type)
                }
            }
        }


    }
}

//================
// maybe_option_type()
//================
impl<'a> Parser<'a> {
    pub fn maybe_option_type (
        &mut self,
        some_type: &Type
    ) -> Option<OptionType> {
        if self.expect_question() {
            self.next();
            Some(
                OptionType{
                    some_type: some_type.to_owned()
                }
            )
        } else {
            None
        }
    }
}

//================
// maybe_result_type()
//================
// FIXME: hardcoded Res<T,E> 
impl<'a> Parser<'a> {
    pub fn maybe_result_type (
        &mut self,
    ) -> Option<ResultType> {
        if ! self.expect_res() { return None }
        self.next();

        let open_angle = self.require_open_angle();
        let _ = self.res_to_opt(open_angle);

        let ok_type = self.require_type();
        let ok_type = self.res_to_opt(ok_type)?;

        self.optional_comma();

        let err_type = self.require_type();
        let err_type = self.res_to_opt(err_type)?;

        let close_angle = self.require_close_angle();
        let _ = self.res_to_opt(close_angle);

        Some(
            ResultType{
                ok_type,
                err_type
            }
        )
    }
}

//================
// maybe_primitive_type()
//================
// FIXME: separate ar/en
impl<'a> Parser<'a> {
    pub fn maybe_primitive_type (&mut self) -> Option<PrimitiveType>{
        if !self.expect_id() { return None }
        let id = self.next();
        match &id.value {
            TokenValue::Id(v)   =>  
                match v.as_str() {
                    "bool" 
                    | "منطقي"
                    | "int"
                    | "صحيح"
                    | "float"
                    | "عائم"
                    | "char"
                    | "محرف" 
                    | "str" 
                    | "سلسلة" => {
                        Some( PrimitiveType{ id } )
                    },
                    _ => None
            },
            _ => None
        }
    }
}

//================
// maybe_list_type()
//================
impl<'a> Parser<'a> {
    pub fn maybe_list_type (&mut self) -> Option<ListType>{
        if !self.expect_open_bracket() { return None }
        self.next();
        let _type = self.require_type();
        let _type = self.res_to_opt(_type)?;
        let _close_bracket = self.require_close_bracket();
        let _ = self.res_to_opt(_close_bracket)?;
        Some(ListType {els_type: Box::new(_type) } )
    }
}

//================
// UnitOrTupleType
//================
pub enum UnitOrTupleType {
    Unit,
    Tuple(TupleType)
}

//================
// maybe_tuple_or_unit_type()
//================
impl<'a> Parser<'a> {
    pub fn maybe_unit_or_tuple_type (&mut self) -> Option<UnitOrTupleType>{
        if !self.expect_open_paren() { return None }
        self.next();
        if self.expect_close_paren() { 
            self.next();
            return Some(UnitOrTupleType::Unit) 
        }
        let mut types = vec![];
        while !self.expect_close_paren() {
            let _type = self.require_type();
            let _type = self.res_to_opt(_type)?;
            types.push(_type);
            self.optional_comma();

            if self.expect_eof() {
                let t = self.lookahead();
                self.insert_err( error!( format!("unclosed, expecting )"), t ) );
                return None
            }

            if self.expect_close_paren() { 
                self.next();
                break;
            }
        }
        return Some(UnitOrTupleType::Tuple(TupleType{ types: Box::new(types)}))
    }
}

//================
// maybe_struct_type()
//================
impl<'a> Parser<'a> {
    pub fn maybe_struct_type (&mut self) -> Option<StructType>{
        if !self.expect_open_curly() { return None }
        todo!();    // TODO
    }
}

//================
// maybe_bool()
//================
impl<'a> Parser<'a> {
    pub fn maybe_bool (
        &mut self,
    ) -> Option<Token>{
        if !self.expect_bool() { return None  }
        Some( self.next() .clone() )
    }
}


//================
// maybe_char()
//================
impl<'a> Parser<'a> {
    pub fn maybe_char (
        &mut self,
    ) -> Option<Token>{
        if !self.expect_char() { return None }
        Some( self.next().clone() )
    }
}

//================
// maybe_string()
//================
impl<'a> Parser<'a> {
    pub fn maybe_string (
        &mut self,
    ) -> Option<Token>{
        if !self.expect_str() { return None }
        Some( self.next().clone() )
    }
}


//================
// maybe_int()
//================
impl<'a> Parser<'a> {
    pub fn maybe_int (
        &mut self,
    ) -> Option<Token>{
        if !self.expect_int() { return None }
        Some( self.next().clone() )
    }
}

//================
// maybe_float()
//================
impl<'a> Parser<'a> {
    pub fn maybe_float (
        &mut self,
    ) -> Option<Token>{
        if !self.expect_float() { return None }
        Some( self.next().clone() )
    }
}

//================
// maybe_id()
//================
impl<'a> Parser<'a> {
    pub fn maybe_id (&mut self) -> Option<Token>{
        if !self.expect_id() { return None }
        let id = self.next().clone();
        Some(id)
    }
}

//================
// maybe_struct_literal()
//================
impl<'a> Parser<'a> {
    pub fn maybe_struct_literal (&mut self) -> Option<StructLiteral> {
        if !self.is_struct_literal() { return None}
        let open_curly = if self.expect_open_curly() {
            self.next();
            true
        } else {
            false
        };
        let mut literal = StructLiteral{ items: vec![]};
        loop {
            if self.expect_eof() && open_curly {
                let t = self.lookahead();
                self.insert_err( error!("unterminated struct literal".to_string(), t));
                return None
            } 
            
            if open_curly && self.expect_close_curly() {
                self.next();
                return Some(literal);
            }


            let t = self.lookahead();
                               
            let key = self.require_id();
            let key = self.res_to_opt(key)?;

            let colon = self.require_colon();
            let _ = self.res_to_opt(colon)?;

            let expr = self.require_expr();
            let expr = self.res_to_opt(expr)?;

            literal.items.push((key, Some(expr)));
            self.optional_comma();
        }
    }
}


//================
// require_struct_lietral()
//================
impl<'a> Parser<'a> {
    pub fn require_struct_lietral (&mut self) -> Result<StructLiteral, Error> {
        if let Some(expr) = self.maybe_struct_literal() {
            Ok(expr)
        } else {
            Err(error!("expecting a struct literal".to_string(), self.lookahead()))
        }
    }
}

//================
// maybe_pattern()
//================
impl<'a> Parser<'a> {
    pub fn maybe_pattern (
        &mut self,
    ) -> Option<Pattern>{
        let p = if let Some(p) = self.maybe_primitive_literal() {
            Some(Pattern::PrimitiveLiteral(Box::new(p)))
        } else if let Some(id) = self.maybe_id() {
            if self.expect_open_curly() { 
                if let Some(mut p) = self.maybe_struct_pattern() {
                    p.name = Some(id);
                    Some(Pattern::Struct(p))
                } else {
                    let t = self.lookahead();
                    self.insert_err( error!( "expecting a a struct expression after { ".to_string(), t ));
                    None
                }
            } else if self.expect_dot() {
                if let Some(mut p) = self.maybe_enum_pattern() {
                    p.name = Some(id);
                    Some(Pattern::Enum(p))  
                } else {
                    let t = self.lookahead();
                    self.insert_err(
                        error!(
                            "expecting an enum variant identifer after .".to_string(),
                            t
                    ));
                    None                    
                }                
            } else {
                self.symtab().insert_id_pattern(&id);
                Some(Pattern::Id(IdPattern { id }))
            }
        } else if let Some(p) = self.maybe_list_pattern() {
            Some(Pattern::List(p))
        } else if let Some(p) = self.maybe_tuple_pattern() {
            Some(Pattern::Tuple(p))            
        } else if let Some(p) = self.maybe_struct_pattern() {
            Some(Pattern::Struct(p))
        } else if let Some(p) = self.maybe_enum_pattern() {
            Some(Pattern::Enum(p))  
        } else if let Some(_) = self.maybe_wildcard_pattern() {
            Some(Pattern::Wildcard)
        } else {
            None
        };

        p

    }
}

//================
// require_pattern()
//================
impl<'a> Parser<'a> {
    pub fn require_pattern (
        &mut self
    ) -> Result<Pattern, Error> {
        if let Some(pat) = self.maybe_pattern() {
            Ok(pat)
        } else {
            Err(error!("expecting a pattern".to_string(), self.lookahead()))
        }
    }
}

//================
// maybe_patterns()
//================
impl<'a> Parser<'a> {
    pub fn maybe_patterns (
        &mut self,
    ) -> Vec<Pattern> {
        let mut patterns = vec![];
        loop {
            match self.maybe_pattern() {
                Some(p) => {
                    patterns.push(p);
                    self.optional_comma();

                },
                _ => return patterns
            }
        }

    }
}

//================
// maybe_list_pattern()
//================
impl<'a> Parser<'a> {
    pub fn maybe_list_pattern (
        &mut self,
    ) -> Option<ListPattern>{
        if !self.expect_open_bracket() { return None; }
        let items = self.require_list_patterns();
        let items = self.res_to_opt(items)?;
        Some(ListPattern{items})
    }
}

//================
// require_list_patterns()
//================
impl<'a> Parser<'a> {
    pub fn require_list_patterns(
        &mut self,
    ) -> Result<Vec<Pattern>, Error> {
        if !self.expect_open_bracket() { 
            return Err( error!("expecting `[`".to_string(), self.lookahead()) )
         }

        self.next(); 
        let patterns = self.maybe_patterns();
        match self.require_close_bracket() {
            Ok(_) => Ok(patterns),
            Err(err) => Err(err)
        }
    }
}

//================
// maybe_tuple_pattern()
//================
impl<'a> Parser<'a> {
    pub fn maybe_tuple_pattern(
        &mut self,
    ) -> Option<TuplePattern>{
        if !self.expect_open_paren() {return None }

        self.next(); 
        let patterns = self.maybe_patterns();
        let close_paren = self.require_close_paren();
        self.res_to_opt(close_paren)?;
        Some(TuplePattern{items: patterns})
    }
}

//================
// maybe_params()
//================
impl<'a> Parser<'a> {
    pub fn maybe_params(
        &mut self,
    ) -> Option<Vec<Param>>{
        if !self.expect_open_paren() { return None }
        self.next(); 

        let mut params = vec![];
        loop {

             match self.maybe_pattern() {
                Some(pat) => {
                    let _type = self.optional_type_annotation();
                    params.push(Param{
                        pat,
                        _type
                    });
                    self.optional_comma();

                },
                None => break
            }
        }
        let close_paren = self.require_close_paren();
        let _ = self.res_to_opt(close_paren)?;
        Some(params)

    }
}

//================
// require_params()
//================
impl<'a> Parser<'a> {
    pub fn require_params(
        &mut self,
    ) -> Result<Vec<Param>, Error>{
        match self.maybe_params() {
            None => Err( error!("missing parameters (..) ".to_string(), self.lookahead()) ),
            Some(params) => Ok(params)
        }
    }
}

//================
// maybe_struct_pattern()
//================
impl<'a> Parser<'a> {
    pub fn maybe_struct_pattern (
        &mut self,
    ) -> Option<StructPattern> {
        if !self.expect_open_curly() { return None }
        self.next();
        let struct_patterns = self.require_struct_patterns();
        let items = self.res_to_opt(struct_patterns)?;
        let close_curly = self.require_close_curly();
        let _ = self.res_to_opt(close_curly)?;
        Some(StructPattern{ name: None, items})
    }
}

//================
//  require_struct_patterns()
//================
impl<'a> Parser<'a> { 
    fn require_struct_patterns(
        &mut self,
    ) -> Result<StructPatternFields, Error> {
        let mut pattern = StructPatternFields::from([]);
        loop {
            
            if self.expect_id() {
                let t = self.next();
                
                if self.expect_colon() {
                    self.next();
                    if let Some(p) = self.maybe_pattern() {
                        pattern.insert(t, Some(p));
                    } else {
                        let t = self.current();
                        self.insert_err( error!( format!("expecting a pattern: {:?}", t.value), t ) );   
                    }    
                } else {
                    pattern.insert(t, None);
                }

                self.optional_comma();  

            } else  {
                return Ok(pattern);
            }
        }
    }    
}

//================
//  maybe_enum_patterns()
//================
impl<'a> Parser<'a> { 
    fn maybe_enum_pattern(
        &mut self,
    ) -> Option<EnumPattern> {
        
        if !self.expect_dot()  { return None }
        let id = self.require_id();
        let variant_name = self.res_to_opt(id)?;
        if !self.expect_arg_list() { return None}
        self.next();
        let pattern = self.require_pattern();
        let pattern = self.res_to_opt(pattern)?;
        let close_paren = self.require_close_paren();
        let _ = self.res_to_opt(close_paren)?;

        let variant = EnumVariant { 
            name: variant_name, 
            pattern: Some(Box::new(pattern))
        };

        Some( EnumPattern {  name: None,  variant  } )

    }
}

//================
// maybe_wildcard_pattern()
//================
impl<'a> Parser<'a> {
    pub fn maybe_wildcard_pattern (&mut self) -> Option<Token> {
        if !self.expect_underscore() { return None }
        Some(self.next())
    }
}

//================
// require_exprs()
//================
impl<'a> Parser<'a> {
    pub fn require_exprs (
        &mut self,
        closing_symbol: TokenValue
    )  -> Result<Vec<Expr>, Error> {
        let mut exprs = vec![];

        while self.lookahead().value != closing_symbol {
            if let Some(e) = self.maybe_expr() {
                exprs.push(e);
                self.optional_comma();
            } else { 
                break; 
            }  
        }
        if self.lookahead().value == closing_symbol {
            self.next();
            Ok(exprs)              
        } else {
            Err( error!( format!("unclosed, expecting {:?}", closing_symbol), self.lookahead() ) )
        }
    }
}

//================
// maybe_list()
//================
impl<'a> Parser<'a> {
    pub fn maybe_list (&mut self)  -> Option<List> {
        if !self.expect_open_bracket() { return None }
        self.next();
        let exprs = self.require_exprs(TokenValue::CloseBracket);
        let exprs = self.res_to_opt(exprs)?;
        Some(List{items: exprs})
    }
}

// FIXME, change to maybe_param_list, no one element tuple is allowed (like haskell)
//================
// maybe_tuple()
//================
impl<'a> Parser<'a> {
    pub fn maybe_tuple (&mut self)  -> Option<Tuple> {
        if !self.expect_open_paren() { return None }
        self.next();
        let exprs = self.require_exprs(TokenValue::CloseParen);
        let exprs = self.res_to_opt(exprs)?;
        Some(Tuple{items: exprs})
    }
}

//================
// require_tuple()
//================
impl<'a> Parser<'a> {
    pub fn require_tuple (&mut self)  -> Result<Tuple, Error> {
        if let Err(err) = self.require_open_paren() {
            Err(err)
        } else {
            match self.require_exprs(TokenValue::CloseParen) {
                Err(err) => Err(err),
                Ok(exprs) => Ok(Tuple{items: exprs})
            }
        } 
    }
}

//================
// maybe_group_tuple_unit()
//================
impl<'a> Parser<'a> {
    pub fn maybe_group_tuple_unit (&mut self)  -> Option<Expr> {
        if !self.expect_open_paren() { return None }
        self.next();
        let exprs = self.require_exprs(TokenValue::CloseParen);
        let exprs = self.res_to_opt(exprs)?;
        let expr = match exprs.len() {
            0 => Expr::Unit,
            1 => exprs[0].clone(),
            _ => Expr::Tuple(Tuple{items: exprs})
        };

        Some(expr)
    }
}

//================
// maybe_short_decl()
//================
impl<'a> Parser<'a> {
    pub fn maybe_short_decl (
        &mut self,
        id: Option<&Token>
    ) -> Option<Decl> {
        let t = self.lookahead();
        let pattern = if let Some(id) = id {
                Some( Pattern::Id( IdPattern { id: id.to_owned() } ) )
        } else {
            self.maybe_pattern()
        };

        let pattern = pattern?;

        let t = self.lookahead();
        let decl = self.require_decl_asign();
        let _ = self.res_to_opt(decl)?;
        
        let expr = self.maybe_expr();
        let termiantor = self.require_terminator();
        let _ = self.res_to_opt(termiantor)?;

        match expr {
            None => {
                self.insert_err(
                    error!( format!("declaration: expecting an expression: {:?}", t.value), t  )
                );
                None   
            },
            Some(expr) => {
                Some(
                    Decl {
                        pattern: pattern, 
                        _type: None,
                        expr: Some(expr)
                    }
                )    
            }
        }
    } 
}

//================
// maybe_let_decl()
//================
impl<'a> Parser<'a> {
    pub fn maybe_let_decl (&mut self) -> Option<Decl> {
        if ! self.expect_let() { return None }
        self.next();
        let pattern = self.require_pattern();
        let pattern = self.res_to_opt(pattern)?;

        let _type = self.optional_type_annotation();
        let t = self.lookahead();

        if self.expect_equal() {

            self.next();
            let expr = self.maybe_expr();
            let termiantor = self.require_terminator();
            let _ = self.res_to_opt(termiantor)?;
            match expr {
                None => {
                    self.insert_err(
                        error!( format!("declaration: expecting an expression: {:?}", t.value), t  )
                    );
                    None   
                },
                Some(expr) => {
                    Some(
                        Decl {
                            pattern: pattern, 
                            _type: _type,
                            expr: Some(expr)
                        }
                    )    
                }
            }
        } else {
            Some(
                Decl {
                    pattern: pattern, 
                    _type: _type,
                    expr: None
                }
            )

        }
    }

}

//================
// require_let_decl()
//================
impl<'a> Parser<'a> {
    pub fn require_let_decl (&mut self) -> Result<Decl, Error> {
        let t = self.lookahead();
        match self.maybe_let_decl() {
            None => Err(error!( format!("found let keyword, expecting a declaration: {:?}", t.value), t  )),
            Some(decl) => Ok(decl)
        }
    }
}

//================
// maybe_prefix_uni_op()
//================
impl<'a> Parser<'a> {
    pub fn maybe_prefix_uni_op (
        &mut self,
    )  -> Option<UniOp> {
        if !self.expect_prefix_uni_op() { return None }
        let op = self.next();
        let opr = self.require_prim();
        let opr = self.res_to_opt(opr)?;
        Some( UniOp { opr: Box::new(opr),  op }) 
    }
}


//================
// while_op()
//================
impl<'a> Parser<'a> {
    pub fn while_op (
        &mut self,
        l_opr: &Expr,
        cond: Option<bool>,
    ) -> Expr {
        
        let mut expr = l_opr.clone();

        while self.expect_bin_op() || self.expect_postfix_uni_op(){
            if let Some(cond) = cond {
                if !cond { break; }
            }  
            match self.require_op(&expr) {
                Err(err) => panic!("bug : {:?} ", err),
                Ok(_expr) => {
                    expr = _expr;
                }
            } 
        }
        expr
    }
}

//================
// maybe_op()
//================
impl<'a> Parser<'a> {
    pub fn maybe_op (
        &mut self,
        l_opr: &Expr,
    )  -> Option<Expr> {
        let op = self.lookahead();
        let l_opr = l_opr.clone();
        if self.expect_postfix_uni_op() {
            let postfix_op = self.next();
            let l_opr = self.l_opr(&l_opr, &postfix_op);
            Some(l_opr)
            
        } else if self.expect_bin_op() {

            if let Some(expr) = self.l_opr_prefix_bin(&l_opr, &op) {
                return Some(expr)
            } 

            let r_opr = self.require_r_opr(&l_opr);
            let mut r_opr = self.res_to_opt(r_opr)?;

            if self.expect_bin_op() {
                let next_op = self.lookahead().to_string();

                r_opr = match &r_opr {
                    Expr::BinOp(BinOp{op, .. }) => {
                        self.while_op(
                            &r_opr, 
                            Some(
                                operator::prec_bin(&next_op)  > operator::prec_bin(&op.to_string()) 
                                || operator::is_bin_rassoc(&next_op)
                            )
                        )
                                
                    },
                    Expr::PreUniOp(UniOp{op, ..}) => {
                        self.while_op(
                            &r_opr, 
                            Some(
                                operator::prec_bin(&next_op)  > operator::prec_uni(&op.to_string()) 
                                || operator::is_bin_rassoc(&next_op)
                            )
                        )
                                
                    },                    
                    _ => {
                        self.while_op(
                            &r_opr, 
                            None
                        )                                                
                    }
                }
            }

            let expr = Expr::BinOp(
                BinOp {
                    l_opr: Box::new(l_opr.to_owned()),
                    r_opr: Box::new(r_opr),
                    op
                }
            );

            Some(expr)

        } else {
            None
        }
    }
}

//================
// require_op()
//===============
impl<'a> Parser<'a> {
    pub fn require_op (
        &mut self,
        l_opr: &Expr,
    )  -> Result<Expr, Error> {
        let t = self.lookahead();
        match self.maybe_op(l_opr) {
            None => Err(error!( format!("expect an operation: {:?}", t.value), t  )),
            Some(expr) => Ok(expr)
        }
    }
}


//================
// l_opr()
//================
impl<'a> Parser<'a> {
    fn l_opr(
        &mut self,       
        l_opr: &Expr,
        op: &Token
    ) -> Expr {

        let l_opr = if let Some(expr) = self.l_opr_prefix_postfix(l_opr, op) {
            expr
        } else if let Some(expr) = self.l_opr_bin_postfix(l_opr, op) {
            expr
        } else {
            // x?
            Expr::PostUniOp(
                UniOp {
                    opr: Box::new(l_opr.to_owned()),
                    op: op.to_owned()
                }
            )
        };
        l_opr
    }
}

// FIXME refactor: instead of checking for postfix operator in the caller, pass an operator and check it here along with the uniop
//================
// l_opr_prefix_postfix()
//================
impl<'a> Parser<'a> {
    fn l_opr_prefix_postfix(
        &mut self,        
        l_opr: &Expr,
        postfix_op: &Token
    ) -> Option<Expr> {
        match l_opr {
            // -x?
            Expr::PreUniOp(UniOp{opr, op} ) => {
                if operator::prec_uni(&postfix_op.to_string()) > operator::prec_uni(&op.to_string()) {
                    Some(
                        Expr::PreUniOp(
                            UniOp {
                                opr: Box::new(
                                    Expr::PostUniOp(                                        
                                        UniOp {
                                            opr: opr.to_owned(), 
                                            op: postfix_op.to_owned()
                                        }
                                    )
                                ),
                                op: op.to_owned()
                            }
                        )
                    )
                } else {
                    Some(
                        Expr::PostUniOp(
                            UniOp {
                                opr: Box::new(l_opr.to_owned()),
                                op: postfix_op.to_owned()
                            }
                        )
                    )
                }                
            },
            _ => None
        }
    }
}

//================
// l_opr_prefix_bin()
//================
impl<'a> Parser<'a> {
    fn l_opr_prefix_bin(
        &mut self,        
        expr: &Expr,
        bin_op: &Token
    ) -> Option<Expr> {

        match expr {
            Expr::PreUniOp(UniOp{op, opr}) => {        
                if operator::prec_bin(&bin_op.to_string()) > operator::prec_uni(&op.to_string()) {
                    let opr = self.require_op(&opr);
                    let opr = self.res_to_opt(opr)?;
                    Some(
                        Expr::PreUniOp(
                            UniOp {
                                opr: Box::new(opr),
                                op: op.to_owned()
                            }
                        )
                    )
                    
                } else {
                    // -x + ...
                    let r_opr = self.require_r_opr(expr);
                    let r_opr = self.res_to_opt(r_opr)?;
                    Some (
                        Expr::BinOp(
                            BinOp {
                                l_opr: Box::new(expr.to_owned()),
                                r_opr: Box::new(r_opr),
                                op: bin_op.to_owned()
                            }
                        )
                    )
                    
                }
            }
            _ => None
        }
    }
}

//================
// l_opr_bin_postfix()
//================
impl<'a> Parser<'a> {
    fn l_opr_bin_postfix(
        &mut self,        
        expr: &Expr,
        postfix_op: &Token
    ) -> Option<Expr> {
        match expr {
            // x + y? + z
            Expr::BinOp(BinOp{l_opr, r_opr, op}) => {
                if operator::prec_uni(&postfix_op.to_string()) > operator::prec_bin(&op.to_string()) { 
                    Some(
                        Expr::BinOp (
                            BinOp {  
                                l_opr: l_opr.to_owned(), 
                                r_opr: Box::new(
                                    Expr::PostUniOp(
                                        UniOp{
                                            opr: r_opr.to_owned() , 
                                            op: postfix_op.to_owned()
                                        }
                                    )
                                ), 
                                op: op.to_owned()
                            }
                        )
                    )
                } else {    // x()?
                    Some( 
                        Expr::PostUniOp (
                            UniOp {
                                opr: Box::new(expr.to_owned()),
                                op: postfix_op.to_owned()
                            }
                        )
                    )
                }                
            },
            _ => None
        }
    }
}

//================
// require_r_opr()
//================
impl<'a> Parser<'a> {
    fn require_r_opr(
        &mut self,        
        l_opr: &Expr,
    ) -> Result<Expr, Error> {
        let t = self.lookahead();
        let r_opr = if self.expect_index() {
            let index = self.require_list_index();
            self.res_to_opt(index)              
        } else if self.expect_arg_list() {
            let fn_call = self.require_fn_call();   // FIXME: also variants! replace with require_fn_call_variant
            self.res_to_opt(fn_call)            
        } else if self.expect_struct_literal() {
            let struct_init = self.require_struct_init();   
            self.res_to_opt(struct_init)            
        } else if self.expect_dot() {
            let access = self.require_access(&l_opr);
            self.res_to_opt(access)
        } else {
            self.next();            
            // self.maybe_prim()
            self.maybe_expr()
        };      
    

        match r_opr {
            None => Err(error!( format!("expecting a right operand: {:?}", t.value), t  )),
            Some(r_opr) => Ok(r_opr)
        }        
    }
}

//================
// require_access()
//================
impl<'a> Parser<'a> {
    fn require_access(
        &mut self,
        l_opr: &Expr,
    ) -> Result<Expr, Error> {
        self.require_dot()?;        

        match l_opr {
            Expr::BinOp(bin_op) => {
                if bin_op.is_access() {
                    // symtab.skip();  
                }
            },
            _ => ()
        }
        self.require_prim()
    }
}

//================
// require_args()
//================
impl<'a> Parser<'a> {
    fn require_args(&mut self) -> Result<Tuple, Error> {
        if let Err(err) = self.require_arg_list() {
            Err(err)
        } else {
            match self.require_exprs(TokenValue::CloseParen) {
                Err(err) => Err(err),
                Ok(exprs) => Ok(Tuple{items: exprs})
            }
        }     
    }
}

//================
// require_fn_call()
//================
impl<'a> Parser<'a> {
    fn require_fn_call(&mut self) -> Result<Expr, Error> {
        let tuple = self.require_args()?;
        Ok(Expr::Tuple(tuple))
    }
}


//================
// require_struct_init()
//================
impl<'a> Parser<'a> {
    fn require_struct_init(&mut self) -> Result<Expr, Error> {
        let struct_literal = self.require_struct_lietral()?;
        Ok(Expr::StructLiteral(struct_literal))
    }
}


//================
// require_list_index()
//================
impl<'a> Parser<'a> {
    fn require_list_index(&mut self) -> Result<Expr, Error> {
        self.require_index()?;
        let expr = self.require_expr()?;
        self.require_close_bracket()?;
        Ok(expr)
    }
}

//================
// maybe_struct()
//================
impl<'a> Parser<'a> {
    pub fn maybe_struct (
        &mut self,
        id: &Token,
        attrs: &Option<Vec<Attr>>
    )  -> Option<Struct> {

        if !self.expect_open_curly() { return None }
        self.next();

        let name = id.clone();
        self.symtab().new_scope();        
        let attrs = if let Some(attrs) = attrs {
            attrs.clone()
        } else {
            vec![]
        };

        let fields = self.maybe_struct_fields();
        let fields = self.res_to_opt(fields);

        let impls = vec![];

        self.symtab().exit_scope();
            
        Some( 
            Struct {
                name,
                attrs,
                fields,
                impls
            }
        )
    }
}

//================
// maybe_struct_fields()
//================
impl<'a> Parser<'a> {
    fn maybe_struct_fields (
        &mut self,
    )  -> Result<StructFields, Error> {
        let mut fields: StructFields = HashMap::new();
        
        loop {
            if self.expect_id() {
                let field = self.require_field()?;
                fields.insert(field.0, field.1);
            }  else {
                let close_curly = self.require_close_curly()?;
                return Ok(fields)
            }
            self.optional_comma();
        }         
    }
}

//================
// require_field()
//================
impl<'a> Parser<'a> {
    fn require_field(
        &mut self
    ) -> Result<(Token, Type), Error> {
        let field_name = self.require_id()?;
        let _ = self.require_colon()?;
        let field_type = self.require_type()?;
        Ok((field_name, field_type))
    }
}

//================
// maybe_struct_impl()
//================
// FIXME: support only one func/method with multiple impl blocks for now.. later on allow an impl block to have multiple funcs/methods
// FIXME: add to symtab
impl<'a> Parser<'a> {
    pub fn maybe_struct_impl (
        &mut self,
        id: &Token,
        attrs: &Option<Vec<Attr>>
    )  -> Option<StructImpl> {
        if ! self.expect_double_colon() { return None }
        self.next();
        let is_method = if self.expect_caret() {
            self.next();
            false
        } else {
            true
        };
        let fn_id = self.maybe_id();
        let _fn = self.require_fn(fn_id.as_ref(), attrs, is_method);
        if let Some(_fn) = self.res_to_opt(_fn) {
            Some(
                StructImpl {
                    name: id.clone(),
                    fns: vec![_fn]
                }    
            )    
        } else {
            None
        }
    }
}

//================
// maybe_trait()
//================
impl<'a> Parser<'a> {
    pub fn maybe_trait (
        &mut self,
    )  -> Option<Trait> {
        None    // TODO
    }
}

//================
// maybe_enum()
//================
impl<'a> Parser<'a> {
    pub fn maybe_enum (
        &mut self,
    )  -> Option<Enum> {
        None    // TODO
    }
}

//================
// maybe_enum_impl()
//================
impl<'a> Parser<'a> {
    pub fn maybe_enum_impl (
        &mut self,
        id: &Token,
    )  -> Option<EnumImpl> {
        None    // TODO
    }
}

//================
// LambdaOrDecl()
//================
#[derive(Debug)]
pub enum LambdaOrDecl {
    Lambda(Fn),
    Decl(Decl)
}

//================
// thin_arrow_or_decl()
//================
impl<'a> Parser<'a> {
    pub fn thin_arrow_or_decl (
        &mut self,
    )  -> Result<TokenValue, Error> {
        let t = self.lookahead();
        match t.value {
            TokenValue::ThinArrow | TokenValue::DeclAsign => {
                Ok(t.value)
            },
            _ => {
                Err(
                    error!("expecting a function defintion ( -> ) or declaration statement (:= )".to_string(), t)
                )              
            }
        }        
    }
}

//================
// maybe_lambda_or_decl()
//================
impl<'a> Parser<'a> {
    pub fn maybe_lambda_or_decl (
        &mut self,
        attrs: &Option<Vec<Attr>>
    )  -> Option<LambdaOrDecl> {

        if !self.expect_open_paren() { return None }

        let after_close_paren = self.look_after_close_paren(); 
        let t = self.res_to_opt(after_close_paren)?;  
        let sym = match t.value {
            TokenValue::ThinArrow | TokenValue::DeclAsign=> {
                Some(t.value)
            },
            // TokenValue::Colon => {
            //     let arrow_or_decl = self.thin_arrow_or_decl(); 
            //     Some(self.res_to_opt(arrow_or_decl)?)
            // },
            _ => None
        };

        match sym {
            Some(TokenValue::ThinArrow) => {
                match self.maybe_fn(None, attrs, false) {
                    Some(_fn) => Some(LambdaOrDecl::Lambda(_fn)),
                    None => None
                }                        
            },
            Some(TokenValue::DeclAsign) => {
                match self.maybe_short_decl(None) {
                    Some(decl) => Some(LambdaOrDecl::Decl(decl)),
                    None => None
                }                        
            }
            _ => None
        }         
    }
}

//================
// maybe_short_import()
//================
impl<'a> Parser<'a> {
    pub fn maybe_short_import (
        &mut self,
    )  -> Option<Decl> {
        return None;    // FIXME: not implemented
    }
}

//================
// maybe_fn()
//================
impl<'a> Parser<'a> {
    pub fn maybe_fn (
        &mut self,
        id: Option<&Token>,
        attrs: &Option<Vec<Attr>>, 
        is_method: bool
    )  -> Option<Fn> {
        if !self.expect_open_paren() { return None }
        self.symtab().new_scope();
        let params = self.require_params();
        let params = self.res_to_opt(params)?;                                              

        let ret_type = self.optional_type_annotation();

        let thin_arrow = self.require_thin_arrow();
        let _ = self.res_to_opt(thin_arrow)?;
        
        let block = self.require_block();
        let block = self.res_to_opt(block)?;        

        self.symtab().exit_scope();

        Some(
            Fn{
                attrs: attrs.clone(),
                is_method,
                name: id.cloned(),
                params,
                ret_type,
                block
            }
        )                    
            
    }        
}

//================
// require_fn()
//================
impl<'a> Parser<'a> {
    pub fn require_fn(
        &mut self,
        id: Option<&Token>,
        attrs: &Option<Vec<Attr>>,
        is_method: bool
    ) -> Result<Fn, Error>{
        match self.maybe_fn(id, attrs, is_method) {
            None => Err( error!("expecting function declaration".to_string(), self.lookahead()) ),
            Some(params) => Ok(params)
        }
    }
}

//================
// expect_decl()
//================ 
// FIXME: this doesn't work, declarations can have patterns, not just IDs
//          instead, look at struct_literal_or_block() implementation 
impl<'a> Parser<'a> {
    pub fn expect_short_decl (
        &mut self,
    )  -> bool {
        let t = if self.expect_open_bracket() {
            let t =  self.look_after_close_bracket();
            self.res_to_opt(t)
        } else if self.expect_open_curly() {
            let t =  self.look_after_close_curly();
            self.res_to_opt(t)
        } else if self.expect_open_paren() {
            let t =  self.look_after_close_paren();
            self.res_to_opt(t)
        } else if self.expect_id() {
            self.look_after_id()
        } else {
            None
        };

        match t {
            None => false,
            Some(t) => {
                match t.value {
                    TokenValue::DeclAsign => true,
                    _ => false
                }
            }
        }

    }
}

//================
// is_struct_literal()
//================ 
impl<'a> Parser<'a> {
    pub fn is_struct_literal(&mut self) -> bool {
        let mut i = 1;
        let mut starts_with = vec![];
        loop {
            let t = self.lookahead_n_ws(i);
            match t.value  {
                TokenValue::NewLine => (),
                TokenValue::Eof => break,
                x => {
                    starts_with.push(x);
                    if starts_with.len() == 3 { 
                        break; 
                    }
                }   
            }
            i += 1;
        }
        match starts_with[..] {
            [
                TokenValue::OpenCurly, 
                TokenValue::Id(_),
                TokenValue::Colon
            ] => true, 
            _ => false
        }
    }
}

//================
// require_block()
//================ 
impl<'a> Parser<'a> {
    pub fn require_block (
        &mut self,
    )  -> Result<Vec<BlockElement>, Error> {
        self.symtab().new_scope();
        let block = if self.expect_open_curly() {
            self.require_block_curly()
        } else {
            self.require_block_one_liner()
        };
        self.symtab().exit_scope();

        block
    }
}

//================
// return_last_expr()
//================ 
impl<'a> Parser<'a> {
    pub fn return_last_expr (
        &mut self,
        els: &mut Vec<BlockElement>
    ) {
        // let mut last = els.pop();
        if let Some(BlockElement::Expr(expr)) = els.last() {
            let expr = expr.clone();
            els.pop();
            els.push(BlockElement::Expr(Expr::Ret(Box::new(expr))));
        }
    }
}

//================
// require_block_curly()
//================ 
impl<'a> Parser<'a> {
    pub fn require_block_curly (
        &mut self,
    )  -> Result<Vec<BlockElement>, Error> {
        let mut els = vec![];      

        if self.is_struct_literal() {
            let t = self.lookahead();
            let expr = Expr::StructLiteral(self.require_struct_lietral()?);
            // self.symtab().exit_scope();
            return Ok(vec![BlockElement::Expr(expr)])
        } else {
            self.next(); 
        }

        loop {
            if self.expect_let() {
                let decl = self.require_let_decl()?;
                els.push(BlockElement::Decl(decl));
            } else if self.expect_short_decl() {
                if let Some(decl) = self.maybe_short_decl(None) {
                    self.require_terminator()?;
                    els.push(BlockElement::Decl(decl));                    
                }                
            } else {
                if let Some(expr) = self.maybe_expr() {
                    self.require_terminator()?;
                    els.push(BlockElement::Expr(expr));
                    
                } else {
                    let _ = self.require_close_curly()?;
                    self.return_last_expr(&mut els);
                    return Ok(els)
                }                    
            }  
        }          
    }
}

//================
// require_block_one_liner()
//================ 
impl<'a> Parser<'a> {
    pub fn require_block_one_liner (
        &mut self,
    )  -> Result<Vec<BlockElement>, Error> {

        let mut els = vec![];        
        if self.expect_let() {
            let decl = self.require_let_decl()?;
            els.push(BlockElement::Decl(decl));
        } else if self.expect_short_decl() {
            if let Some(decl) = self.maybe_short_decl(None) {
                self.require_terminator()?;
                els.push(BlockElement::Decl(decl));    
            }                
        } else {
            if let Some(expr) = self.maybe_expr() {
                self.require_terminator()?;
                els.push(BlockElement::Expr(expr));
                
            } else {
                let t = self.lookahead();
                self.insert_err(error!("expecting end of block".to_string(),t));                                
            }                    
        }          

        self.return_last_expr(&mut els);   // TODO: this should be called only on blocks that return, not on cases like  x:= 3 , make sure to split the 2 scenarios
        return Ok(els)
    }
}

//================
//  look_after_id()
//================
impl<'a> Parser<'a> {
    pub fn look_after_id(&mut self) -> Option<Token> {
        if !self.expect_id() { return None }
        let mut i = 1;
        let mut tokens = vec![];
        while !self.expect_eof(){
            let t = self.lookahead_n_ws(i);
            if let TokenValue::NewLine = t.value {
            } else if matches!(t.value, TokenValue::Id(_)) && tokens.is_empty() {
                tokens.push(t);
            } else {
                tokens.push(t);
                if tokens.len() == 2 {
                    break;
                } 
            }
            i +=1;
        }        
        return Some(tokens[1].clone());
    }
}

//================
//  look_after_close_paren()
//================
impl<'a> Parser<'a> {
    pub fn look_after_close_paren(&mut self) -> Result<Token, Error> {
        self.look_after_close_sym(
            TokenValue::OpenParen,
            TokenValue::CloseParen
        )
    }
}

//================
//  look_after_close_curly()
//================
impl<'a> Parser<'a> {
    pub fn look_after_close_curly(&mut self) -> Result<Token, Error> {
        self.look_after_close_sym(
            TokenValue::OpenCurly,
            TokenValue::CloseCurly
        )
    }
}

//================
//  look_after_close_bracket()
//================
impl<'a> Parser<'a> {
    pub fn look_after_close_bracket(&mut self) -> Result<Token, Error> {
        self.look_after_close_sym(
            TokenValue::OpenBracket,
            TokenValue::CloseBracket
        )
    }
}

//================
//  look_after_close_sym()
//================
// FIXME: ineffecient, if thre are many expressions nested and grouped with () , instead. use the lexer with a stack
//  to track open parens and if the closing is followed by : or -> then mark the opening as a param_list
impl<'a> Parser<'a> {
    pub fn look_after_close_sym(
        &mut self,
        open_sym: TokenValue,
        close_sym: TokenValue
    ) -> Result<Token, Error> {
        let mut i = 1;
        let mut level = 0;
        let mut res: Result<Token,Error>;
        loop {
            let t = self.lookahead_n_ws(i);
            if TokenValue::Eof == t.value {
                res = Err(
                    error!("unclosed , expected `)`".to_string(), t)
                );
            } else if open_sym == t.value {
                level += 1
            } else if close_sym == t.value {
                level -= 1;
                if level == 0 {
                    res = Ok(self.lookahead_n_ws(i+1));
                    break;
                } else {
                    level -= 1;
                }
            }
            i += 1;
        }
        res
    }
}

//================
// maybe_lambda()
//================
impl<'a> Parser<'a> {
    pub fn maybe_lambda(
        &mut self,
        attrs: &Option<Vec<Attr>>
    ) -> Option<Fn> {

        let after_close_paren = self.look_after_close_paren(); 
        let t = self.res_to_opt(after_close_paren)?;  
        match t.value {
            TokenValue::ThinArrow => {
                self.maybe_fn(None, attrs, false) 
            },
            _ => None
        }
    }
}



//================
// maybe_match()
//================
impl<'a> Parser<'a> {
    fn maybe_match(
        &mut self,    
    ) -> Option<Match> {
        if !self.expect_match() { return None }

        let t = self.next();
        let expr = self.require_expr();
        let open_curly = self.expect_open_curly();
        if open_curly { self.next(); }
        let expr = self.res_to_opt(expr)?;

        let mut arms = vec![];

        loop {

            if let Some(arm) = self.maybe_match_arm() {
                arms.push(arm);
            } else {
                break;
            }    
        }
        if open_curly { 
            let close_curly = self.require_close_curly();
            let _ = self.res_to_opt(close_curly);
        } 

        Some( 
            Match { 
                expr: Box::new(expr), 
                arms 
            }
        ) 
    }
}

//================
// maybe_match_arm()
//================
impl<'a> Parser<'a> {
    fn maybe_match_arm(
        &mut self,    
    ) -> Option<Arm> {
        if self.expect_close_curly() { return None }

        let pattern = self.require_pattern();
        let pattern = self.res_to_opt(pattern)?;

        let arrow = self.require_arrow();
        let _ = self.res_to_opt(arrow)?;
        
        let expr = self.require_block();
        let expr = self.res_to_opt(expr)?;

        Some(
            Arm {
                pattern,
                block: expr
            }
        )
    }
}

//================
// maybe_for()
//================
impl<'a> Parser<'a> {
    fn maybe_for(
        &mut self,    
    ) -> Option<For> {
        
        if !self.expect_for() { return None };
        
        let t = self.next();
        
        let in_expr = self.require_in_expr();
        let in_expr = self.res_to_opt(in_expr)?;  
        let block = self.require_block();
        let block = self.res_to_opt(block)?;        
        Some( For { in_expr, block } )
    }
}


//================
// require_in_expr()
//===============
impl<'a> Parser<'a> {
    pub fn require_in_expr (
        &mut self,
    )  -> Result<InExpr, Error> {

        let pattern = self.require_pattern()?;

        self.require_in()?;
        let expr = self.require_expr()?;

        Ok(InExpr { pattern, expr: Box::new(expr) })
    }
}

//================
// maybe_while()
//================
impl<'a> Parser<'a> {
    fn maybe_while(
        &mut self,    
    ) -> Option<While> {
        if !self.expect_while() { return None };
        self.next();
        let expr = self.require_expr();
        let expr = self.res_to_opt(expr)?;

        let block = self.require_block();
        let block = self.res_to_opt(block)?;


        // let block = self.require_block(vec![TokenValue::End]);
        // let (block, is_multiline) = self.res_to_opt(block)?;

        // if is_multiline {
        //     let end = self.require_end();
        //     let _ = self.res_to_opt(end)?;    
        // }

        Some( 
            While { 
                expr: Box::new(expr), 
                block: Box::new(block)
            } 
        )        
    }
}

//================
// maybe_if()
//================
impl<'a> Parser<'a> {
    fn maybe_if(
        &mut self,  
    ) -> Option<If> {
        // let mut multiline;
        let _if = self.maybe_if_branch()?;
        // multiline = _multiline;    

        // let (_if, _multiline) = self.maybe_if_branch()?;
        // multiline = _multiline;    
        let mut branches= vec![_if];
        
        while self.expect_else() { 
            self.next();
            // if let Some((_else_if, _multiline) ) = self.maybe_if_branch() {
                if let Some(_else_if ) = self.maybe_if_branch() {                
                // multiline = _multiline;    
                branches.push(_else_if);
            } else {
                let _else = self.require_else();
                let _else = self.res_to_opt(_else)?;                
                // let (_else, _multiline) = self.res_to_opt(_else)?;
                // multiline = _multiline;
                branches.push(_else);

            }
        };

        // if multiline {
        //     let end = self.require_end();
        //     let _ = self.res_to_opt(end)?;                    
        // }
        
        Some( If { branches } )
                 
    }
}

//================
// maybe_if_branch()
//================
impl<'a> Parser<'a> {
    fn maybe_if_branch(
        &mut self,  
    // ) -> Option<(IfBranch, bool)> {
    ) -> Option<IfBranch> {
        if !self.expect_if() { return None };
        self.next();
        let expr = self.require_expr();
        let expr = self.res_to_opt(expr)?;

        let block = self.require_block();
        let block = self.res_to_opt(block)?;

        // let terminators = vec![TokenValue::End, TokenValue::Else];
        // let block = self.require_block(terminators);
        // let (block, multiline) = self.res_to_opt(block)?;
        let branch = IfBranch{ expr: Some(Box::new(expr)), block };
        // Some((branch, multiline))
        Some(branch)

    }
}
//================
// require_else()
//================
impl<'a> Parser<'a> {
    fn require_else(
        &mut self,  
    // ) -> Result<(IfBranch, bool), Error> {
    ) -> Result<IfBranch, Error> {
        // let terminators = vec![TokenValue::End];
        // let (block, multiline) = self.require_block(terminators)?;
        let block = self.require_block()?;
        let branch = IfBranch{ expr: None, block };
        // Ok((branch, multiline))
        Ok(branch)
    }
}

//================
// maybe_ok()
//================
// FIXME Hardcoded variants for demo
impl<'a> Parser<'a> {
    fn maybe_ok(
        &mut self,    
    ) -> Option<Expr> {
        if !self.expect_ok() { return None}
        self.next();
        let arg_list = self.require_arg_list();
        let _ = self.res_to_opt(arg_list)?;

        let expr = self.require_expr();
        let expr = self.res_to_opt(expr)?;

        let close_paren = self.require_close_paren();
        let _ = self.res_to_opt(close_paren)?;        
        Some(Expr::Ok(Box::new(expr)))
    }
}

//================
// maybe_err()
//================
// FIXME Hardcoded variants for demo
impl<'a> Parser<'a> {
    fn maybe_err(
        &mut self,    
    ) -> Option<Expr> {
        if !self.expect_err() { return None}
        self.next();
        let arg_list = self.require_arg_list();
        let _ = self.res_to_opt(arg_list)?;

        let expr = self.require_expr();
        let expr = self.res_to_opt(expr)?;

        let close_paren = self.require_close_paren();
        let _ = self.res_to_opt(close_paren)?;        
        Some(Expr::Err(Box::new(expr)))
    }
}

//================
// maybe_some()
//================
// FIXME Hardcoded variants for demo
impl<'a> Parser<'a> {
    fn maybe_some(
        &mut self,    
    ) -> Option<Expr> {
        if !self.expect_some() { return None}
        self.next();
        let arg_list = self.require_arg_list();
        let _ = self.res_to_opt(arg_list)?;

        let expr = self.require_expr();
        let expr = self.res_to_opt(expr)?;

        let close_paren = self.require_close_paren();
        let _ = self.res_to_opt(close_paren)?;        
        Some(Expr::Some(Box::new(expr)))
    }
}

//================
// maybe_none()
//================
// FIXME Hardcoded variants for demo
impl<'a> Parser<'a> {
    fn maybe_none(
        &mut self,    
    ) -> Option<Expr> {
        if !self.expect_none() { return None}
        self.next();
        Some(Expr::None)
    }
}



//================
// maybe_code()
//================
impl<'a> Parser<'a> {
    fn maybe_code(
        &mut self,    
        id: &Token,
    ) -> Option<Code> {
        if !self.expect_code() { return None}
        let code = self.next();
        Some(
            Code {
                lang: id.clone(),
                code
            }
        )
    }
}

//================
// disambiguate_paren()
//================
impl<'a> Parser<'a> {
    pub fn disambiguate_paren(&mut self) -> Option<Expr> {
                
        if let Some(_fn) = self.maybe_lambda(&None) {
            return Some(Expr::Fn(_fn))            
        } else {
            let t = self.lookahead();
                        
            if let Some(expr) = self.maybe_group_tuple_unit() {
                Some(expr)
            } else {
                self.insert_err(error!("expecting an expression".to_string(), t));
                None
            }
        }
    }
}

//================
// maybe_primitive_literal()
//================
impl<'a> Parser<'a> {
    fn maybe_primitive_literal(
        &mut self,    
    ) -> Option<Expr> {
        if let Some(t) = self.maybe_bool() {
            Some(Expr::Bool(t))
        } else if let Some(t) = self.maybe_char() {
            Some(Expr::Char(t))
        } else if let Some(t) = self.maybe_string() {
            Some(Expr::Str(t))
        } else if let Some(t) = self.maybe_int() {
            Some(Expr::Int(t))
        } else if let Some(t) = self.maybe_float() {
            Some(Expr::Float(t))            
        } else {
            None
        }
    }

}

//================
// maybe_literal()
//================
impl<'a> Parser<'a> {
    fn maybe_literal(
        &mut self,    
    ) -> Option<Expr> {
        if let Some(e) = self.maybe_primitive_literal() {
            Some(e)
        } else if let Some(e) = self.maybe_list() {
            Some(Expr::List(e))    
        } else if let Some(e) = self.maybe_tuple() {
            Some(Expr::Tuple(e))    
        } else if let Some(e) = self.maybe_struct_literal() {
            Some(Expr::StructLiteral(e))
        } else {
            None
        }
    }
}

//================
// maybe_prim()
//================
impl<'a> Parser<'a> {
    fn maybe_prim(
        &mut self,    
    ) -> Option<Expr> {
        if self.expect_open_paren() {   
            self.disambiguate_paren()
        } else if let Some(e) = self.maybe_prefix_uni_op() {
            Some(Expr::PreUniOp(e))
        } else if let Some(e) = self.maybe_literal() {
            Some(e)
        } else if let Some(t) = self.maybe_id() {
            self.symtab().insert_ref(&t);
            Some(Expr::Ref(t))            
        } else if let Some(e) = self.maybe_match() {
            Some( Expr::Match(e) )
        } else if let Some(e) = self.maybe_for() {
            Some( Expr::For(e) )
        } else if let Some(e) = self.maybe_while() {
            Some( Expr::While(e) )
        } else if let Some(e) = self.maybe_if() {
            Some( Expr::If(e) )
        }  else if let Some(e) = self.maybe_ok() {
            Some(e)
        }  else if let Some(e) = self.maybe_err() {
            Some(e)
        }  else if let Some(e) = self.maybe_some() {
            Some(e)
        }  else if let Some(e) = self.maybe_none() {
            Some(e)
        } else {
            None
        }
    }
}

//================
// require_prim()
//================
impl<'a> Parser<'a> {
    pub fn require_prim (
        &mut self,
    ) -> Result<Expr, Error>{
        let t = self.lookahead();
        match self.maybe_prim() {
            None => Err(error!( format!("expecting an expression: {:?}", t.value), t  )),
            Some(expr) => Ok(expr)
        }
    }
}

//================
// maybe_expr()
//================
impl<'a> Parser<'a> {
    pub fn maybe_expr (
        &mut self,
    ) -> Option<Expr>{
        let expr = self.maybe_prim();
        match expr {
            None => None,
            Some(mut expr) => {

                match &expr {
                    Expr::Ref(t) => {
                        if let Some(code) = self.maybe_code(&t) {
                            Some(Expr::Code(code))
                        } else {
                            expr = self.while_op(&expr, None);
                            Some(expr)
                        }
                    },
                    _ => {                        
                        expr = self.while_op(&expr, None);                        
                        Some(expr)        
                    }
                }
            }
        }
    }
}

//================
// require_expr()
//================
impl<'a> Parser<'a> {
    pub fn require_expr (
        &mut self,
    ) -> Result<Expr, Error>{
        let t = self.lookahead();
        match self.maybe_expr() {
            None => Err(error!( format!("expecting an expression: {:?}", t.value), t  )),
            Some(expr) => Ok(expr)
        }

    }
}