//================
//   Text
//================
#[derive(Debug)]
pub struct Text {
    pub ar: String,
    pub en: String
}

impl Text {
    //---------------------
    //  new()
    //---------------------     
    pub fn new(
        ar: &str, 
        en: &str
    ) -> Self {
        Self {
            ar: ar.to_string(),
            en: en.to_string()    
        }
    }
}