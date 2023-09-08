use crate::lang::syntax_tree::ast;

use super::Crate;

pub struct TauriBuild {
    id: String,
    version: String,
    features: Option<ast::List>
}

impl TauriBuild {
    pub fn new() -> Self {
        Self {
            id: "tauri-build".to_string(), 
            version:"1.2".to_string(),
            features: Some(ast::List{
                items: vec![
                ]
            })

        
        }
    }
}

impl Crate for TauriBuild {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
    fn features(&self) -> &Option<ast::List> { &None }
}