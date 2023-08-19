use crate::lang::syntax_tree::ast;

pub mod actix_files;
pub mod actix_web;

//================
//  Crate
//================  
pub trait Crate {
    fn id(&self) -> &String;
    fn version(&self) -> &String;
    fn features(&self) -> &Option<ast::List>;
}