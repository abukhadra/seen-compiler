use std::fmt;

//================
//  Location
//================
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    //---------------------
    //  new()
    //---------------------        
    pub fn new(
        line: usize,
        column: usize
    ) -> Self {
        Self {
            line,
            column
        }
    }
}

//================
//  Display Location
//================
impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}:{}", self.line, self.column)
    }
}

//================
//  Default Location
//================
impl Default for Location {
    fn default() -> Self {
        Self { line: 0, column: 0 }
    }
}


//================
//  TokenValue
//================
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum TokenValue {
    // Identifier
    Id(String),

    // Literals
    Bool(String),
    Int(String),
    Float(String),
    Char(String),
    Str(String),
    
    Code(String),

    // Whitespace
    NewLine,

    // Operators
    Minus,    
    Question,
    Exclamation,
    Add,    
    Sub,
    Mul,
    Div,
    Perc,
    Hash,
    Tilde,
    Caret,
    ArgList,
    Index,
    Not,
    LogicalOr,      
    LogicalAnd,
    BitwiseOr,
    BitwiseAnd,    
    BitwiseXor,    
    PipeForward,
    Equal,    
    DeclAsign,
    AddEqual,
    SubEqual,
    MulEqual,
    DivEqual,
    BitwiseAndEqual,
    BitwiseOrEqual,
    BitwiseXorEqual,
    Eq,
    NE,
    GT,
    GE,
    LT,
    LE,
    Dot,
    DoubleDot,
    PrefixDoubleDot,
    PostfixDoubleDot,
    Dollar,
    At,


    // Punctuators
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Colon,
    DoubleColon,
    Comma,
    ThinArrow,
    Arrow,
    Bar,


    // Temp Keywords ( should be enums )
    Res,
    Some,
    Err,
    None,
    Ok,

    // Keywords
    Ret,
    Let,
    In,
    Where,
    Match,
    For,
    While,
    If,
    Else,
    Use,
    Underscore,
    Eof,
}


//================
//   Display TokenValue
//================
impl fmt::Display for TokenValue {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let x = match &self {
            Self::Id(val)
            | Self::Bool(val)
            | Self::Int(val)
            | Self::Float(val) => val.to_string(),
            Self::Char(val) => format!("{}", val.to_string()),
            Self::Str(val) => format!("{}", val.to_string()),
            Self::Code(val) => format!("{}", val.to_string()),
            Self::NewLine => "\\n".to_string(),
            Self::Minus => "-".to_string(),
            Self::Question => "?".to_string(),
            Self::Exclamation => "!".to_string(),
            Self::Add => "+".to_string(),            
            Self::Sub => "-".to_string(),
            Self::Mul => "*".to_string(),
            Self::Div => "/".to_string(),
            Self::ArgList  => "(".to_string(),
            Self::Index => "[".to_string(),
            Self::PipeForward => "|>".to_string(),
            Self::Equal => "=".to_string(),
            Self::AddEqual => "+=".to_string(),
            Self::SubEqual => "-=".to_string(),
            Self::MulEqual => "*=".to_string(),
            Self::DivEqual => "/=".to_string(),
            Self::BitwiseOrEqual => "\\/=".to_string(),
            Self::BitwiseAndEqual => "/\\=".to_string(),
            Self::BitwiseXorEqual => "(+)=".to_string(),
            Self::Perc => "%".to_string(),
            Self::Hash => "#".to_string(),
            Self::Tilde => "~".to_string(),
            Self::Caret => "^".to_string(),                  
            Self::Eq => "==".to_string(),
            Self::NE => "!=".to_string(),
            Self::GT => ">".to_string(),
            Self::GE => ">=".to_string(),
            Self::LT => "<".to_string(),
            Self::LE => "<=".to_string(),
            Self::Not => "!".to_string(),
            Self::LogicalOr => "||".to_string(),
            Self::LogicalAnd => "&&".to_string(),
            Self::BitwiseAnd => "/\\".to_string(),
            Self::BitwiseOr => "\\/".to_string(),
            Self::BitwiseXor => "(+)".to_string(),
            Self::Dot => ".".to_string(),
            Self::DoubleDot
            | Self::PrefixDoubleDot
            | Self::PostfixDoubleDot => "..".to_string(),
            Self::Dollar => "$".to_string(),
            Self::At => "@".to_string(),
            Self::OpenBracket => "[".to_string(),
            Self::CloseBracket => "]".to_string(),
            Self::OpenParen => "(".to_string(),
            Self::CloseParen => ")".to_string(),
            Self::OpenCurly => "{".to_string(),
            Self::CloseCurly => "}".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Colon => ":".to_string(),
            Self::DoubleColon => "::".to_string(),
            Self::Comma => ",".to_string(),
            Self::ThinArrow => "->".to_string(),
            Self::Arrow => "=>".to_string(),
            Self::Bar => "|".to_string(),         
            Self::DeclAsign => ":=".to_string(),   

            Self::Res => "Res".to_string(),
            Self::Ok => "Ok".to_string(),
            Self::Err => "Err".to_string(),
            Self::Some => "Some".to_string(),
            Self::None => "None".to_string(),

            Self::Ret => "ret".to_string(),
            Self::Let => "let".to_string(),
            Self::In => "in".to_string(),
            Self::Where => "where".to_string(),
            Self::Match => "match".to_string(),
            Self::For => "for".to_string(),
            Self::While => "while".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
            Self::Use => "use".to_string(),
            Self::Underscore => "_".to_string(),
            Self::Eof => "eof".to_string(),
        };

        write!(f, "{}", x)
    }
}

//================
//  Token
//================
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub location: Location,
}

impl Token {
    //---------------------
    //  new()
    //---------------------    
    pub fn new(
        value: TokenValue,
        location: Location
    ) -> Self {
        Self {
            value,
            location
        }
    }    
}

//================
//   Display Token
//================
impl fmt::Display for Token {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

//================
//  unescape_character()
//================
pub fn unescape_char(c: char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".into(),
        '\t' => "\\t".into(),
        '\'' => "\\'".into(),
        '\"' => "\\\"".into(),
        '\\' => "\\\\".into(),
        _ => c.into(),
    }
}
