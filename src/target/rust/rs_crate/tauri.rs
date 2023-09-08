use crate::lang::{
    syntax_tree::ast, 
    token::{ Token, TokenValue, Location }
};

use super::Crate;

pub struct Tauri {
    id: String,
    version: String,
    features: Option<ast::List>
}

impl Tauri {
    pub fn new() -> Self {
        Self {
            id: "tauri".to_string(), 
            version:"1.2".to_string(),
            features: Some(ast::List{
                items: vec![
                    ast::Expr::Str(Token{
                        value: TokenValue::Str(String::from("shell-open")),
                        location: Location{
                            line: 0, 
                            column: 0
                        }
                    })
                ]
            })        
        }
    }
}

impl Crate for Tauri {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
    fn features(&self) -> &Option<ast::List> { &self.features }
}