use crate::lang::{
    syntax_tree::ast, 
    token::{ Token, TokenValue, Location }
};

use super::CrateFeatures;

pub struct CustomProtocol {
    id: String,
    features: Option<ast::List>
}

impl CustomProtocol {
    pub fn new() -> Self {
        Self {
            id: "custom-protocol".to_string(), 
            features: Some(ast::List{
                items: vec![
                    ast::Expr::Str(Token{
                        value: TokenValue::Str(String::from("tauri/custom-protocol")),
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

impl CrateFeatures for CustomProtocol {
    fn id(&self) -> &String { &self.id }
    fn features(&self) -> &Option<ast::List> { &self.features }
}