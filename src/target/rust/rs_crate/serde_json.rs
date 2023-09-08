use crate::lang::syntax_tree::ast;

use super::Crate;

pub struct SerdeJson {
    id: String,
    version: String,
    features: Option<ast::List>
}

impl SerdeJson {
    pub fn new() -> Self {
        Self {
            id: "serde_json".to_string(), 
            version:"1.0".to_string(),
            features: None
        
        }
    }
}

impl Crate for SerdeJson {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
    fn features(&self) -> &Option<ast::List> { &self.features }
}