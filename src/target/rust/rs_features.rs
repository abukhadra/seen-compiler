
use crate::lang::syntax_tree::ast;

pub mod custom_protocol;


//================
//  CrateFeatures
//================  
pub trait CrateFeatures {
    fn id(&self) -> &String;
    fn features(&self) -> &Option<ast::List>;
}