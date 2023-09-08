//================
//   Color
//================
pub struct Color {}

impl Color {
    //---------------------
    //  red()
    //---------------------    
    pub fn red(v: &str) -> String {
        format!("\x1b[31m{}\x1b[0m", v)
    }
    
}
