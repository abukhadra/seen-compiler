use crate::lang::syntax_tree::ast::*;

use std::{
    fmt::{
        self,
    },
};

//================
//   Display Expr
//================
impl fmt::Display for Expr {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool(t)  
            | Self::Int(t)  
            | Self::Float(t) 
            | Self::Ref(t) => write!(f, "{:?}", t.value ),
            Self::Char(t) => write!(f, "{}", t.value ),
            Self::Str(t) => write!(f, "{}", t.value ),
            Self::List(e) =>  write!(f, "{:?}", e),
            Self::Tuple(e) =>  write!(f, "{:?}", e),
            Self::StructLiteral(e) => write!(f, "{:?}", e),
            Self::BinOp(e) => write!(f, "{:?}", e),
            Self::PreUniOp(e) => write!(f, "{:?}", e),
            Self::PostUniOp(e) => write!(f, "{:?}", e),
            Self::Fn(e) => write!(f, "{:?}", e),
            Self::Match(e) => write!(f, "{:?}", e),
            Self::For(e) => write!(f, "{:?}", e),
            Self::While(e) => write!(f, "{:?}", e),
            Self::If(e) => write!(f, "{:?}", e),
            Self::Code(e) => write!(f, "{:?}", e),
            Self::Ret(e) => write!(f, "return {:?}", e),

            Self::Ok(e) => write!(f, "Ok({:?})", e),
            Self::Err(e) => write!(f, "Err({:?})", e),
            Self::Some(e) => write!(f, "Some({:?})", e),
            Self::None => write!(f, "None"),            


        }
    }
}