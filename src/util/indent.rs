use std::fmt;

//================
//  Indent
//================
pub struct Indent {
    spaces: usize,
    level: usize,
}

impl Indent {
    //---------------------
    //  new()
    //---------------------       
    pub const fn new() -> Self {
        Self {
            spaces: 4,
            level: 0,
        }
    }

    //---------------------
    //  inc()
    //---------------------   
    pub fn inc(&mut self) {
        self.level += 1;
    }

    //---------------------
    //  dec()
    //---------------------       
    pub fn dec(&mut self) {
        self.level -= 1;
    }

}

//================
//  Display Indent
//================
impl fmt::Display for Indent {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:indent$}", "", indent = self.spaces * self.level)
    }
}
