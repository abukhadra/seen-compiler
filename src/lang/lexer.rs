use std::{
    iter::Peekable,
    fmt::Write
};

use regex::Regex;

use crate::{
    lang::{
        Lang,
        error::Error,
        script::Script,
        token::{
            Location, 
            TokenValue,
            Token, 
        },        
    },
};


//================
//   Constants
//================
const NULL: char = '\0';
const TATWEEL: char = '\u{640}'; // ـ

//================
//   Lexer
//================
pub struct Lexer<'a> {
    lang: &'a Lang,
    start_location: Location,
    end_location: Location,
    current: char,
    look_ahead: char,
    iter: Peekable<std::str::Chars<'a>>,
    tokens: Option<Vec<Token>>,
    errors: Option<Vec<Error>>,
    space_indent: bool,
    tab_indent: bool,
    indent_conflict_reported: bool,
    any_letter: Regex,
    any_numeric: Regex,
}

impl<'a> Lexer<'a> {
    //---------------------
    //  new()
    //---------------------    
    pub fn new() -> Self {
        Lexer {
            lang: &Lang::Ar,
            start_location: Location::new(1,1),
            end_location: Location::new(1,1),
            current: NULL,
            look_ahead: NULL,
            iter: "".chars().into_iter().peekable(),
            tokens: None,
            errors: None,
            space_indent: false,
            tab_indent: false,
            indent_conflict_reported: false,
            any_letter: Regex::new(r"\p{L}").unwrap(),
            any_numeric: Regex::new(r"\p{N}").unwrap(),
        }
    }

    //---------------------
    //  tokens()
    //---------------------        
    pub fn tokens(
        &mut self,
        lang: &'a Lang,
        script: &'a Script,
    ) -> (Vec<Token>, Vec<Error>) {
        self.init(lang, script);
        while !self.expect_eof() {
            self.start_location = self.end_location.clone();
            let c = self.next();

            match self.lang {
                Lang::Ar => {
                    match c {
                        '؟' => self.add_token(TokenValue::Question),
                        '\\' => self.ar_comment_asgmt_div_bwand(),
                        '/' => self.ar_bwor(),
                        '«' => self.ar_string(),
                        '‹' => self.ar_character(),   
                        '0'..='9' => {
                            self.number_western();
                            self.number_postfix();
                        },
                        '٠' ..='٩' => {

                            self.number_eastern();
                            self.number_postfix();
                        }                                                 
                        _ => self.common(c)  
                    }      
                }, 
                Lang::En => {
                    match c {
                        '?' => self.add_token(TokenValue::Question),
                        '/' => self.en_comment_asgmt_div_bwand(),
                        '\\' => self.en_bwor(),
                        '"' => self.en_string(),
                        '\'' => self.en_character(),  
                        '0'..='9' => {
                            self.number_en();
                            self.number_postfix();
                        },
                        '٠' ..='٩' => {
                            self.insert_error(format!("only English Numerals are allowed in English source files: {}", self.current));
                            self.skip_invalid_num_or_id();
                        }                                                      
                        _ => self.common(c)                              
                    }
                }
            }                
        }   

        self.start_location = self.end_location.clone();
        self.add_token(TokenValue::Eof);

        (
            self.tokens.take().unwrap(),          
            self.errors.take().unwrap()           
        ) 

    }

    //---------------------
    //  common()
    //---------------------     
    fn common(
        &mut self, 
        c: char
    ) {
        match c {
            '\n' => self.newline(),
            '\r' => (),
            '\t' | ' ' => self.start_location.column += 1,  
            '+' => self.asgmt_or_plus(),
            '-' => self.asgmt_minus_sub_thin_arrow(),
            '*' => self.asgmt_or_mul(),
            '%' => self.add_token(TokenValue::Perc),
            '#' => self.code(self.start_location.column),
            '~' => self.add_token(TokenValue::Tilde),
            '^' => self.add_token(TokenValue::Caret),
            '=' => self.asgmt_or_equal_or_arrow(),          
            '!' => self.ne_or_exclamation(),
            '>' => self.gt_or_ge(),
            '<' => self.lt_or_le(),
            '&' => self.land(),
            '|' => self.bar_lor_pipe(),
            '.' => self.period_or_float(),             
            ':' => self.colon_or_decl(),   
            '$' => self.add_token(TokenValue::Dollar),
            '@' => self.add_token(TokenValue::At),                 
            '[' => self.add_token(TokenValue::OpenBracket),
            ']' => self.add_token(TokenValue::CloseBracket),
            '(' => self.add_token(TokenValue::OpenParen),
            ')' => self.add_token(TokenValue::CloseParen),
            '{' => self.add_token(TokenValue::OpenCurly),
            '}' => self.add_token(TokenValue::CloseCurly),
            ';' => self.add_token(TokenValue::Semicolon),
            ',' => self.add_token(TokenValue::Comma),

            '\u{1EE4D}' => self.add_token(TokenValue::Res),
            '⎔' => self.add_token(TokenValue::At),
            '؛' => self.add_token(TokenValue::Semicolon),
            '⏎' => self.add_token(TokenValue::Ret),
            '✓' => self.add_token(TokenValue::Ok),
            '✗' => self.add_token(TokenValue::Err),

            x => {
                let v = x.to_string();
                if x == '_' || self.any_letter.is_match(v.as_str()) {
                    self.id_or_keyword();
                    // let _res = self.id_or_keyword();
                    // self.start_location = self.end_location.clone();
                    // if self.expect_open_bracket() {
                    //     self.next();
                    //     self.add_token(TokenValue::Index)
                    // } else if self.expect_open_paren() {
                    //     self.next();
                    //     self.add_token(TokenValue::ArgList)
                    // }
                } else {
                    self.insert_error(format!("unrecognized character: {}", self.current));
                }                
            }
        }
    }


    //---------------------
    //  init()
    //---------------------      
    fn init(
        & mut self,
        lang: &'a Lang,
        script: &'a Script,
    ) {
        self.lang = lang;
        self.start_location = Location::new(1,1);
        self.end_location = Location::new(1,1);        
        self.tokens=  Some(vec![]);
        self.errors= Some(vec![]);        
        self.current = NULL;
        self.look_ahead = NULL;
        self.iter = 
                script
                    .content
                    .chars()
                    .into_iter()
                    .peekable();
        self.space_indent = false;
        self.tab_indent = false;
        self.indent_conflict_reported = false;
    }


    //---------------------
    //  add_token()
    //---------------------    
    fn add_token(
        &mut self, 
        value: TokenValue
    ) {
        let tokens = self.tokens.as_mut().unwrap();
        tokens.push( 
            Token::new(
                value, 
                self.start_location.clone()
            )
        );
    }

    //---------------------
    //  next()
    //---------------------    
    fn next(&mut self) -> char {
        let c = self.iter
                    .next();

        if let Some(c) = c {
            self.current = c;

            match c {
                '\n' => {
                    self.end_location.line += 1;
                    self.end_location.column = 1;
                }

                _ => self.end_location.column += 1,
            }

            if let Some(&look_ahead) = self.iter.peek() {  
                self.look_ahead = look_ahead;
            } else {
                self.look_ahead = NULL;
            }
        } else {
            self.current = NULL;
            self.look_ahead = NULL;
        }

        self.current
    }

    //---------------------
    //  expect()
    //---------------------    
    fn expect(&mut self, c1: char, c2: char) -> bool {
        self.current == c1 
        && self.look_ahead == c2
    }

    //---------------------
    //  skip()
    //---------------------        
    fn skip(&mut self, count: i32) {
        let mut i = 0;
        while i != count {
            self.next();
            i += 1;
        }
    }

    //---------------------
    //  skip_invalid_num_or_id()
    //---------------------        
    fn skip_invalid_num_or_id(&mut self) {
        while self.expect_letter() || self.expect_numeric() || self.expect_underscore()   {
            self.skip(1);
        }
    }    

    //---------------------
    //  expect_tatweel()
    //---------------------        
    fn expect_tatweel(&mut self) -> bool {
        self.look_ahead == TATWEEL 
    }

    //---------------------
    //  expect_open_bracket()
    //---------------------        
    fn expect_open_bracket(&mut self) -> bool {
        self.look_ahead ==  '['
    }

    //---------------------
    //  expect_open_paren()
    //---------------------        
    fn expect_open_paren(&mut self) -> bool {
        self.look_ahead == '('
    }

    //---------------------
    //  expect_letter()
    //---------------------        
    fn expect_letter(&mut self) -> bool {
        let v = self.look_ahead.to_string();
        self.any_letter
            .is_match(v.as_str())
    }

    //---------------------
    //  expect_numeric()
    //---------------------        
    fn expect_numeric(&mut self) -> bool {
        let v = self.look_ahead.to_string();
        self.any_numeric
            .is_match(v.as_str())
    }    

    //---------------------
    //  expect_underscore()
    //---------------------        
    fn expect_underscore(&mut self) -> bool {
        self.look_ahead == '_'
    }


    //---------------------
    //  expect_eof()
    //---------------------        
    fn expect_eof(&mut self) -> bool {
        self.iter
            .peek()
            .is_none()
    }

    //---------------------
    //  expect_eol()
    //---------------------        
    fn expect_eol(&mut self) -> bool {
        self.look_ahead == '\n' || self.look_ahead == '\r' || self.expect_eof()
    }

    //---------------------
    //  insert_error()
    //---------------------        
    fn insert_error(&mut self, msg: String) {
        self.errors.as_mut().unwrap().push(   
            Error::new(
                self.start_location.clone(),
                self.end_location.clone(),
                msg,
            )
        );
    }


    //---------------------
    //  comment()
    //---------------------        
    fn comment(&mut self) {
        while !self.expect_eof() {
            if self.expect_eol() {
                // self.next();
                break;
            } else {
                self.next();
            }
        }
        // self.start_location = self.end_location.clone();
    }

    //---------------------
    //  multiline_comment()
    //---------------------        
    fn multiline_comment(
        &mut self,
        sym: char
    ) {
        let mut levels = vec![];
        while !self.expect_eof() {
            if self.expect(sym, '*') {
                self.skip(2); 
                levels.push(self.end_location.clone());
            } else if self.expect('*', sym) {
                if levels.len() > 1 {
                    self.skip(2);
                    levels.pop();
                } else {
                    self.skip(1);
                    levels.pop();
                    break;
                } // for level , s
            } else {
                self.skip(1)
            } // skip others
        }

        if let Some(location) = levels.pop() {
            self.start_location = location;
            self.insert_error("unclosed comment".to_string());
        }
    }


    //---------------------
    //  ar_comment_asgmt_div_bwand()
    //---------------------        
    fn ar_comment_asgmt_div_bwand(&mut self) {
        match self.look_ahead {
            '\\' => self.comment(),
            '*' => self.multiline_comment('\\'),
            '=' => self.add_token(TokenValue::DivEqual),
            '/' => self.add_token(TokenValue::BitwiseAnd),    
            _ => self.add_token(TokenValue::Div),
        }
    }

    //---------------------
    //  en_comment_asgmt_div_bwand()
    //---------------------        
    fn en_comment_asgmt_div_bwand(&mut self) {
        match self.look_ahead {
            '/' => self.comment(),
            '*' => self.multiline_comment('/'),
            '=' => self.add_token(TokenValue::DivEqual),
            '\\' => self.add_token(TokenValue::BitwiseAnd),    
            _ => self.add_token(TokenValue::Div),
        }
    }

    //---------------------
    //  ar_bwor()
    //---------------------        
    fn ar_bwor(&mut self) {
        match self.look_ahead {
            '\\' => self.add_token(TokenValue::BitwiseOr),    
            _ => (),
        }
    }


    //---------------------
    //  en_bwor()
    //---------------------        
    fn en_bwor(&mut self) {
        match self.look_ahead {
            '/' => self.add_token(TokenValue::BitwiseOr),    
            _ => (),
        }
    }

    //---------------------
    //  escape_character()
    //---------------------        
    fn escape_character(&mut self, c: char) -> char {
        match self.lang {
            Lang::Ar => self.ar_escape_character(c),
            Lang::En => self.en_escape_character(c)
        }
    }

    //---------------------
    //  ar_escape_character()
    //---------------------        
    fn ar_escape_character(&mut self, c: char) -> char {
        if self.current == '/' {
            match self.next() {
                'س' => '\n',    // سطر
                'ر' => '\r',   //  ارجاع
                'ج' => '\t',    // جدولة
                '‹' => '‹',
                '›' => '›',
                '«' => '«',
                '»' => '»',
                _ => {
                    self.insert_error(format!("invalid escape character: /{} ", self.current));
                    c
                }
            }
        } else {
            c
        }
    }

    //---------------------
    //  en_escape_character()
    //---------------------        
    fn en_escape_character(&mut self, c: char) -> char {
        if self.current == '\\' {
            match self.next() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\'' => '\'',
                '"' => '\"',
                '\\' => '\\',
                _ => {
                    self.insert_error(format!("invalid escape character: \\{} ", self.current));
                    c
                }
            }
        } else {
            c
        }
    }    

    //---------------------
    //  enclosed_value()
    //---------------------        
    fn enclosed_value(&mut self, symbol: char) -> String {
        let mut value = String::from("");

        while !self.expect_eof() {
            self.next();

            if self.current == symbol {
                break;
            }

            if self.expect_eol() {
                self.insert_error(
                    format!(
                        "unclosed {} literal, expecting {} ",
                            if symbol == '"' || symbol == '«'{
                                "String"
                            } else if symbol == '\'' || symbol == '‹' {
                                "Character"
                            } else {
                                panic!("enclosed_value(): unexpected literal symbol!")
                            },
                        symbol
                    )
                );
                break;
            }

            let c = self.escape_character(self.current);
            value.push(c);
        }
        value
    }

    //---------------------
    //  multiline_string()
    //---------------------        
    fn multiline_string(
        &mut self,
        close_sym: char
    ) {
        let mut value = String::from("");

        while !self.expect_eof() {
            self.next();
            if self.expect_eof() {
                self.insert_error(
                    format!("unclosed multiline String literal, expecting {}{}{}",close_sym, close_sym, close_sym)
                );
                break;
            }

            if self.expect(close_sym, close_sym) {
                self.skip(1); // first 
                if self.look_ahead == close_sym {
                    self.skip(1); // second 
                    self.add_token(TokenValue::Str(value));
                    break;
                } else {
                    value.push(close_sym); // add first skipped
                    value.push(close_sym); // add current 
                }
            } else {
                let c = self.escape_character(self.current);
                value.push(c);
            }
        }
    }

    //---------------------
    //  ar_multiline_or_empty_string()
    //---------------------        
    fn ar_multiline_or_empty_string(&mut self) {
        self.skip(2); // skip ««
        match self.current {
            '«' => self.multiline_string('»'),
            _ => {
                self.add_token(TokenValue::Str("".to_string()));
            }
        }
    }


    //---------------------
    //  en_multiline_or_empty_string()
    //---------------------        
    fn en_multiline_or_empty_string(&mut self) {
        self.skip(2); // skip ""
        match self.current {
            '"' => self.multiline_string('"'),
            _ => {
                self.add_token(TokenValue::Str("".to_string()));
            }
        }
    }

    //---------------------
    //  ar_string()
    //---------------------        
    fn ar_string(&mut self) {
        match self.look_ahead {
            '«' => self.ar_multiline_or_empty_string(),
            _ => {
                let value = self.enclosed_value('»');
                self.add_token(TokenValue::Str(value));
            }
        }
    }

    //---------------------
    //  en_string()
    //---------------------        
    fn en_string(&mut self) {
        match self.look_ahead {
            '\"' => self.en_multiline_or_empty_string(),
            _ => {
                let value = self.enclosed_value('"');
                self.add_token(TokenValue::Str(value));
            }
        }
    }

    //---------------------
    //  ar_character()
    //---------------------        
    fn ar_character(&mut self) {
        let value = self.enclosed_value('›');
        self.add_token(TokenValue::Char(value));
    }

    //---------------------
    //  en_character()
    //---------------------        
    fn en_character(&mut self) {
        let value = self.enclosed_value('\'');
        self.add_token(TokenValue::Char(value));
    }

    //---------------------
    //  multiline_code()
    //---------------------    
    fn multiline_code(
        &mut self,
        hash_col: usize
    ) -> String {
        let mut res = "".to_string();
        let mut line = "".to_string();

        let terminator = match self.lang { 
            Lang::Ar  => "اه",
            Lang::En => "end"
        };

        while !self.expect_eof() {
            if self.start_location.column <= hash_col &&
              line.trim() == terminator {
                break;
            } else if self.expect_eol() {
                line.push(self.next());
                let _ = write!(res, "{}", line);
                line = "".to_string();
            } else {
                line.push(self.next())
            }
        }
        res
    }

    // let js_code = js# 
    //                     test
    //               end 


    //---------------------
    //  code()
    //---------------------        
    fn code(&mut self, hash_col: usize) {
        let mut value = String::from("");
        while !self.expect_eof() {
            if self.expect_eol() {
                if value.trim() == "" {
                    value = self.multiline_code(hash_col);  
                } else {
                    self.next();
                }
                
                self.add_token(TokenValue::Code(value));
                break;
            }
            value.push(self.next());
        }
    }


    //---------------------
    //  asgmt_or_equal_or_arrow()
    //---------------------        
    fn asgmt_or_equal_or_arrow(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::Eq)
            },
            '>' => {
                self.next();
                self.add_token(TokenValue::Arrow)
            },            
            _ => self.add_token(TokenValue::Equal),
        }
    }

    //---------------------
    //  asgmt_or_plus()
    //---------------------      
    fn asgmt_or_plus(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::AddEqual)
            },
            _ => self.add_token(TokenValue::Add)
        }
    }

    //---------------------
    //  asgmt_minus_sub_thin_arrow()
    //---------------------          
    fn asgmt_minus_sub_thin_arrow(&mut self) {
        if self.expect_letter() 
        || self.expect_numeric() 
        || self.expect_underscore()   
        || self.expect_open_paren()   
        || self.expect_open_bracket()   {
            self.add_token(TokenValue::Minus)
        } else {
            match self.look_ahead {
                '=' => {
                    self.next();
                    self.add_token(TokenValue::SubEqual)
                },
                '>' => {
                    self.next();
                    self.add_token(TokenValue::ThinArrow)
                },            
                _ => self.add_token(TokenValue::Sub)
            }    
        }

    }

    //---------------------
    //  asgmt_or_mul()
    //---------------------      
    fn asgmt_or_mul(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::MulEqual)
            },
            _ => self.add_token(TokenValue::Mul)
        }
    }

    //---------------------
    //  ne_or_exclamation()
    //---------------------        
    fn ne_or_exclamation(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::NE)
            }
            _ => self.add_token(TokenValue::Exclamation),
        }
    }

    //---------------------
    //  gt_or_ge()
    //---------------------        
    fn gt_or_ge(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::GE)
            }
            _ => self.add_token(TokenValue::GT),
        }
    }

    //---------------------
    //  lt_or_le()
    //---------------------        
    fn lt_or_le(&mut self) {
        match self.look_ahead {
            '=' => {
                self.next();
                self.add_token(TokenValue::LE)
            }
            _ => self.add_token(TokenValue::LT),
        }
    }

    //---------------------
    //  land()
    //---------------------        
    fn land(&mut self){
        if self.look_ahead ==  '&' { 
            self.skip(1);
            self.add_token(TokenValue::LogicalAnd);
        } 
    }

    //---------------------
    //  bar_lor()
    //---------------------        
    fn bar_lor_pipe(&mut self) {
        match self.look_ahead { 
            '|' => {
                self.skip(1);
                self.add_token(TokenValue::LogicalOr);    
            },
            '>' => {
                self.skip(1);
                self.add_token(TokenValue::PipeForward);    
            },            
            _ => self.add_token(TokenValue::Bar)
        } 
    }

    //---------------------
    //  id_or_keyword_ar()
    //---------------------        
    fn id_or_keyword_ar(&mut self) {
        let mut v = String::from(self.current);

        while !self.expect_eof() {
            if self.expect_letter() || self.expect_numeric() || self.expect_underscore() {
                if self.expect_tatweel() {
                    self.skip(1);
                } else {
                    v.push(self.next());
                }
            } else {
                break;
            }

        }

        match v.as_str() {
            "صواب"  => self.add_token(TokenValue::Bool(v)),
            "خطا"  => self.add_token(TokenValue::Bool(v)),

            "قيمة" => self.add_token(TokenValue::Some),
            "بلا" => self.add_token(TokenValue::None),

            "ليكن" => self.add_token(TokenValue::Let),
            "في" => self.add_token(TokenValue::In),
            "حيث" => self.add_token(TokenValue::Where),
            "طابق" => self.add_token(TokenValue::Match),
            "لكل" => self.add_token(TokenValue::For),
            "بينما" => self.add_token(TokenValue::While),
            "اذا" => self.add_token(TokenValue::If),
            "والا" => self.add_token(TokenValue::Else),
            // "احضر" => self.add_token(TokenValue::Use),
            "_" => self.add_token(TokenValue::Underscore),
            _ => self.add_token(TokenValue::Id(v)),
        }

    }

    //---------------------
    //  id_or_keyword_en()
    //---------------------      
    fn id_or_keyword_en(&mut self) {
        let mut v = String::from(self.current);
        while !self.expect_eof() {
            if self.expect_letter() || self.expect_numeric() || self.expect_underscore() {
                v.push(self.next());
            } else {
                break;
            }            
        }

        match v.as_str() {
            "true"  => self.add_token(TokenValue::Bool(v)),
            "false"  => self.add_token(TokenValue::Bool(v)),

            "Res" => self.add_token(TokenValue::Res),
            "Ok" => self.add_token(TokenValue::Ok),
            "Err" => self.add_token(TokenValue::Err),
            "Some" => self.add_token(TokenValue::Some),
            "None" => self.add_token(TokenValue::None),

            "let" => self.add_token(TokenValue::Let),
            "in" => self.add_token(TokenValue::In),
            "where" => self.add_token(TokenValue::Where),
            "match" => self.add_token(TokenValue::Match),
            "for" => self.add_token(TokenValue::For),
            "while" => self.add_token(TokenValue::While),
            "if" => self.add_token(TokenValue::If),
            "else" => self.add_token(TokenValue::Else),
            "_" => self.add_token(TokenValue::Underscore),
            _ => self.add_token(TokenValue::Id(v)),
        }
    }

    //---------------------
    //  id_or_keyword()
    //---------------------        
    fn id_or_keyword(&mut self) {
        
        match self.lang {
            Lang::Ar =>self.id_or_keyword_ar(),
            Lang::En =>self.id_or_keyword_en()
        }
    }

    //---------------------
    //  period_or_float()
    //---------------------        
    fn period_or_float(&mut self) {
        let mut value = String::from(self.current);
        match self.look_ahead {
            '0'..='9' => {
                value.push_str(self.fractional().as_str());
                self.add_token(TokenValue::Float(value));
            }
            _ => self.add_token(TokenValue::Dot),
        }
    }

    //---------------------
    //  colon_or_decl()
    //---------------------   
    fn colon_or_decl(&mut self) {
        match self.look_ahead { 
            '=' => {
                self.skip(1);
                self.add_token(TokenValue::DeclAsign);    
            }
            ':' => {
                self.skip(1);
                self.add_token(TokenValue::DoubleColon);
            }
            _ => self.add_token(TokenValue::Colon)
        } 
    }

    //---------------------
    //  digit_eastern()
    //---------------------        
    fn digit_eastern(&mut self) -> (String, bool) {
        let mut v = String::from(self.current);
        let mut expect_float = false;
        while !self.expect_eof() {
            match self.look_ahead {
                '٠'..='٩' => v.push(self.next()),
                ',' => {
                    v.push(self.next());
                    expect_float = true;
                    v.push_str(self.fractional().as_str());
                },
                '0'..='9' => {
                    self.insert_error(
                        format!(
                            "you can either use Eastern Arabic digits (٠ - ٩) or Western (0 - 9) but not a mix: {}", 
                            self.current
                        )
                    );
                    self.skip_invalid_num_or_id();
                },
                _ => break,
            }
        }
        (v, expect_float)
    }

    //---------------------
    //  digit_western()
    //---------------------        
    fn digit_western(&mut self) -> (String, bool) {
        let mut v = String::from(self.current);
        let mut expect_float = false;
        while !self.expect_eof() {
            match self.look_ahead {
                '0'..='9' => v.push(self.next()),
                '.' => {
                    v.push(self.next());
                    expect_float = true;
                    v.push_str(self.fractional().as_str());
                },
                '٠'..='٩' => {
                    self.insert_error(
                        format!(
                            "you can either use Eastern Arabic digits (٠ - ٩) or Western (0 - 9) but not a mix: {}", 
                            self.current
                        )
                    );
                    self.skip_invalid_num_or_id();
                },                
                _ => break,
            }
        }
        (v, expect_float)
    }

    //---------------------
    //  digit_en()
    //---------------------        
    fn digit_en(&mut self) -> (String, bool) {
        let mut v = String::from(self.current);
        let mut expect_float = false;
        while !self.expect_eof() {
            match self.look_ahead {
                '0'..='9' => v.push(self.next()),
                '.' => {
                    v.push(self.next());
                    expect_float = true;
                    v.push_str(self.fractional().as_str());
                }
                _ => break,
            }
        }
        (v, expect_float)
    }

    //---------------------
    //  number_eastern()
    //---------------------        
    fn number_eastern(&mut self) {
        let (v, expect_float) = self.digit_eastern();
        self.number_type(v, expect_float);
    }

    //---------------------
    //  number_western()
    //---------------------        
    fn number_western(&mut self) {
        let (v, expect_float) = self.digit_western();
        self.number_type(v, expect_float);
    }    

    //---------------------
    //  number_en()
    //---------------------        
    fn number_en(&mut self) {
        let (v, expect_float) =  self.digit_en();
        self.number_type(v, expect_float);
    }    

    //---------------------
    //  number_postfix()
    //---------------------       
    fn number_postfix(&mut self) {
        if self.expect_letter() {
            self.insert_error(
                format!(
                    "invalid postfix: {}", 
                    self.look_ahead
                )
            );                        
            self.skip_invalid_num_or_id();
        }
    }
    
    //---------------------
    //  number_type()
    //---------------------            
    fn number_type(
        &mut self,
        v: String,
        expect_float: bool
    ) {
        if expect_float {
            self.add_token(TokenValue::Float(v));
        } else {
            self.add_token(TokenValue::Int(v));
        }        
    }

    //---------------------
    //  fractional()
    //---------------------        
    fn fractional(&mut self) -> String {
        let mut value = String::from("");
        while !self.expect_eof() {
            match self.look_ahead {
                '0'..='9' => value.push(self.next()),
                _ => break,
            }
        }
        value
    }

    //---------------------
    //  newline()
    //---------------------        
    fn newline(&mut self) {
        loop {

            if self.look_ahead == '\n' {
                self.skip(1);
            } else {
                let tokens = self.tokens.as_mut().unwrap();
                if let Some(Token{value:TokenValue::NewLine, ..} ) = tokens.last() {
                    let _ = tokens.pop();
                } 
                self.add_token(TokenValue::NewLine);                
                break;        
            }
        }
    }

}