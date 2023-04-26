use super::Crate;

pub struct ActixFiles {
    id: String,
    version: String
}

impl ActixFiles {
    pub fn new() -> Self {
        Self {
            id: "actix-files".to_string(), 
            version:"0.6.2".to_string()
        
        }
    }
}

impl Crate for ActixFiles {
    fn id(&self) -> &String { &self.id }
    fn version(&self) -> &String { &self.version }
}