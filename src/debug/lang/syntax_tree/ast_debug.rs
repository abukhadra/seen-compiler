use crate::lang::syntax_tree::ast::*;

use std::{
    fmt::{
        self,
    },
};

//================
//   Debug ModuleElement
//================
impl fmt::Debug for ModElement {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decl(e) => write!(f, "var {:?}", e),
            Self::MainFn(e) => write!(f, "fn t{:?}", e),
            Self::Fn(e) => write!(f, "fn {:?}", e),
            Self::Struct(e) => write!(f, "struct {:?}", e),
            Self::StructImpl(e) => write!(f, "impl struct {:?}", e),
            Self::Trait(e) => write!(f, "trait {:?}", e),
            Self::Enum(e) => write!(f, "enum {:?}", e),
            Self::EnumImpl(e) => write!(f, "impl enum {:?}", e),
        }
    }
}


//================
//   Debug Const
//================
impl fmt::Debug for Decl {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let _ = write!(f, "{:?}", self.pattern);
            if let Some(_type) = self._type.as_ref() { 
                let _ = write!(f, ":{:?}", _type);
            }
            write!(f, "= {:?}", self.expr)
    }
}

//================
//   Debug Expr
//================
impl fmt::Debug for Expr {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool(t)  
            | Self::Int(t)  
            | Self::Float(t) 
            | Self::Char(t) 
            | Self::Str(t)
            | Self::Ref(t) => write!(f, "{:?}", t.value ),
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



//================
//   Debug Pattern
//================
impl fmt::Debug for Pattern {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::PrimitiveLiteral(p) => p.fmt(f),
            Pattern::Id(p) => p.fmt(f),
            Pattern::List(p) => p.fmt(f),
            Pattern::Tuple(p) => p.fmt(f),
            Pattern::Struct(p) => p.fmt(f),
            Pattern::Enum(p) => p.fmt(f),
            Pattern::Wildcard =>  writeln!(f, "_")
        }
    }
}    

//================
//   Debug IdPattern
//================
impl fmt::Debug for IdPattern {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.id.value)
    }
}   


//================
//   Debug ListPattern
//================
impl fmt::Debug for ListPattern {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "[");
        self.items
            .iter()
            .for_each(
                |i| {
                    // let _ = write!(f,"{}", i.to_string());
                    let _ = i.fmt(f);
                }
            );
        writeln!(f, "]")
    }
}    


//================
//   Debug StructPattern
//================
impl fmt::Debug for StructPattern {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            let _ = write!(f, "{}", name);    
        }
        let _ = write!(f, "{{");
        let mut comma = false;
        for (k,v) in self.items.iter() {
            let _ = write!(
                f, 
                "{} {:?}", 
                if comma { "," } else { "" }, 
                k.value 
            );
            match v {
                Some(p) => {
                    let _ = write!(f, ":{:?}", p);
                },
                _ => ()
            }
            comma = true;
        }
        
        write!(f, " }}")
    }
}

//================
//   Debug EnumPattern
//================
impl fmt::Debug for EnumPattern {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            let _ = write!(f, "{}", name);    
        }        

        let _ = write!(f, ".{}", self.variant.name);    
        if let Some(v) = &self.variant.pattern {
            let _ = write!(f, ".({:?})", v);
        }
        Ok(())
    }
}