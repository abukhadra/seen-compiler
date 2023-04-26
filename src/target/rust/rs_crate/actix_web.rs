use super::Crate;

pub struct ActixWeb {
    id: String,
    version: String
}

impl ActixWeb {
    pub fn new() -> Self {
        Self {
            id: "actix-web".to_string(), 
            version:"4".to_string()
        
        }
    }
}


impl Crate for ActixWeb {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
}


