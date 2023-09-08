use crate::lang::syntax_tree::ast;

use super::Crate;

pub struct ActixFiles {
    id: String,
    version: String,
    features: Option<ast::List>
}

impl ActixFiles {
    pub fn new() -> Self {
        Self {
            id: "actix-files".to_string(), 
            version:"0.6.2".to_string(),
            features: None
        
        }
    }
}

impl Crate for ActixFiles {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
    fn features(&self) -> &Option<ast::List> { &None }
}