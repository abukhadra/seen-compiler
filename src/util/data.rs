use crate::lang::{
    syntax_tree::ast::{
        self,
        StructLiteral, 
        Expr
        }, 
        token::{
            Token,
            TokenValue
        },
    };

//================
//  data_iter()
//================  
pub fn data_iter<'a>(
    data:  &'a StructLiteral
) ->  std::slice::Iter<'a, (Token, Option<ast::Expr>)> { 

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

    iter
}
