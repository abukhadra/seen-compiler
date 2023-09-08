use crate::lang::syntax_tree::ast;

use super::Crate;

pub struct ActixWeb {
    id: String,
    version: String,
    features: Option<ast::List>
}

impl ActixWeb {
    pub fn new() -> Self {
        Self {
            id: "actix-web".to_string(), 
            version:"4".to_string(),
            features: None
        
        }
    }
}


impl Crate for ActixWeb {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
    fn features(&self) -> &Option<ast::List> { &None }
}


