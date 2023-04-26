use std::{
    fmt::{
        self, 
        Write,
    },
};   

use crate::lang::{
    script::Script,
    token::Location,
};

//================
//   Constants
//================
const MAX_NUM_LINES_TO_DISPLAY: usize = 6;
const INDENT_ERROR: &'static str = "\n        ";

//======================
//  error!()
//======================
#[macro_export]
macro_rules! error {
    ($msg:expr, $t:expr) => {
        Error::new(      
            $t.location.clone(),
            $t.location.clone(),
            $msg
        )
    };
}

//================
//   Error
//================
#[derive(Clone, Debug)]
pub struct Error {
    start_location: Location,
    end_location: Location,
    message: String,
}

impl Error {
    //---------------------
    //  new()
    //---------------------
    pub fn new(
        location: Location, 
        end_location: Location, 
        msg: String,
    ) -> Self {
        Error {
            start_location: location,
            end_location: end_location,
            message: msg,
        }
    }
}

//================
//   Error
//  Display
//================
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "error: {}", self.message)
    }
}


//================
//  print_errors()
//================
pub fn print_errors(
    errors: &Vec<Error>,
    script: &Script
) {
    for error in errors {
        eprint!("error: {}", error.message);
        eprint!("{}", INDENT_ERROR);
        eprint!("{}:", script.path);
        eprint!("{}", error.start_location);       
        eprint!("{}", INDENT_ERROR);             
        eprintln!("{}", snippet(error, &script.content));    
    }
}


//================
//  snippet()
//================
fn snippet(
    error: &Error,
    script_content: &String
) -> String {
    let total_count = error.end_location.line - error.start_location.line + 1;
    let mut snippet = String::from("");

    if total_count <= MAX_NUM_LINES_TO_DISPLAY {
        let _ = write!(
            snippet,
            "{}",
            extract_lines(
                error.start_location.line, 
                total_count,
                script_content
            )
        );
    } else {
        let count = MAX_NUM_LINES_TO_DISPLAY / 2;
        let _ = write!(
            snippet,
            "{}",
            extract_lines(
                error.start_location.line, 
                count,
                script_content
            )
        );
        let _ = writeln!(snippet, "{}.....", INDENT_ERROR);
        let _ = write!(
            snippet,
            "{}",
            extract_lines(
                error.end_location.line - count + 1, 
                count,
                script_content
            )
        );
    }

    snippet
}

//================
//  extract_lines()
//================
fn extract_lines(
    mut line_number: usize, 
    mut count: usize,
    script_content: &String
) -> String {     
    let mut lines = script_content.lines();

    let mut extracted_lines = String::from("");
    let index = line_number - 1; // Location starts at 1, while lines start at zero

    if let Some(line) = lines.nth(index) {
        let _ = write!(
            extracted_lines, 
            "{}{}\t| {}", 
            INDENT_ERROR, 
            line_number, 
            line
        );
        count -= 1;
        line_number += 1;
    }

    while count > 0 {
        let _ = write!(extracted_lines, "{}", INDENT_ERROR);

        if let Some(line) = lines.next() {
            let _ = write!(
                extracted_lines, 
                "{}\t| {}", 
                line_number, 
                line
            );
        }
        count -= 1;
        line_number += 1;
    }
    extracted_lines
} 