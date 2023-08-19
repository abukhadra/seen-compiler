use std::{
    collections::HashMap,
    fmt::{ self, Debug }
};

use crate::lang::token::{Token, TokenValue};

//================
//   Aliases
//================
// FIXME: replace HashMaps with Vec<(T,E)> with lookup ( or use an ordered_hashmap implementation ), to preserve the order
pub type StructFieldsTypes = HashMap<Token, Option<Type>>;
pub type StructFields = HashMap<Token, Type>;
pub type StructPatternFields = HashMap<Token, Option<Pattern>>;
pub type StructLiteralFields = Vec<(Token, Option<Expr>)>;

//================
//   ModElement
//================
pub enum ModElement {
    Decl(Decl),
    MainFn(Fn),
    Fn(Fn),
    Struct(Struct),
    StructImpl(StructImpl),
    Trait(Trait),
    Enum(Enum),
    EnumImpl(EnumImpl)
}

//================
//   List
//================
#[derive(Clone, Debug)]
pub struct List {
    pub items: Vec<Expr>,
}

//================
//   Tuple
//================
#[derive(Clone, Debug)]
pub struct Tuple {
    pub items: Vec<Expr>,
}

//================
//   StructLiteral
//================
#[derive(Clone, Debug)]
pub struct StructLiteral {
    pub items: StructLiteralFields
}


//================
//   Decl
//================
#[derive(Clone)]
pub struct Decl {
    pub pattern: Pattern,
    pub _type : Option<Type>,
    pub expr: Option<Expr> 
}

//================
//   Expr
//================
#[derive(Clone)]
pub enum Expr {
    Unit,
    Bool(Token),
    Int(Token),
    Float(Token),
    Char(Token),
    Str(Token),
    Ref(Token),
    List(List),
    Tuple(Tuple),
    StructLiteral(StructLiteral),  
    BinOp(BinOp),
    PreUniOp(UniOp),
    PostUniOp(UniOp),
    Fn(Fn),
    Match(Match),
    For(For),
    While(While),
    If(If),
    Code(Code),
    Ret(Box<Expr>), // FIXME: THIS IS WRONG, return is a statement not an expressionm move to BlockElement

    // FIXME: HARDCODING VARIANTS FOR THE DEMO
    Ok(Box<Expr>),
    Err(Box<Expr>),
    Some(Box<Expr>),
    None,

}

//================
//   BlockElement
//================
#[derive(Clone, Debug)]
pub enum BlockElement {
    MainArgs,   // FIXME: this is a workaround, main args are nothing but a Decl
    Decl(Decl),
    Expr(Expr)
}

//================
//   UniOp
//================
#[derive(Clone, Debug)]
pub struct UniOp {
    pub opr: Box<Expr>,
    pub op: Token
}

//================
//   BinOp
//================
#[derive(Clone, Debug)]
pub struct BinOp {
    pub l_opr: Box<Expr>,
    pub r_opr: Box<Expr>,
    pub op: Token
}

impl BinOp {
    //---------------------
    //  is_access()
    //---------------------        
    pub fn is_access(&self) -> bool {
        matches!(self.op.value, TokenValue::Dot)
    }
}


//================
//   Fn
//================
#[derive(Clone, Debug)]
pub struct Fn {
    pub attrs: Option<Vec<Attr>>,
    pub is_method: bool,
    pub name: Option<Token>,
    pub params: Vec<Param>,
    pub ret_type: Option<Type>,
    pub block: Vec<BlockElement>
}

//================
//   Param
//================
#[derive(Clone, Debug)]
pub struct Param {
    // pub name: Token,
    pub pat: Pattern,
    pub _type: Option<Type>
}

//================
//   Type
//================
#[derive(Clone, Debug)]
pub enum Type{
    OptionType(Box<OptionType>),
    ResultType(Box<ResultType>),
    UnitType,
    PrimitiveType(PrimitiveType),
    ListType(ListType),
    TupleType(TupleType),
    StructType(StructType)
}

//================
//   Display Type
//================
impl fmt::Display for Type {
    //---------------------
    //  fmt()
    //---------------------      
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::OptionType(t) => todo!(),
            Type::ResultType(t) => todo!(),
            Type::UnitType => write!(f, "()"),
            Type::PrimitiveType(t) => write!(f, "{}", t.id),
            Type::ListType(t) => todo!(),
            Type::TupleType(t) => todo!(),
            Type::StructType(t) => todo!(),
        }
        
    }
}



//================
//   OptionType
//================
#[derive(Clone, Debug)]
pub struct OptionType {
    pub some_type: Type,
}

//================
//   ResultType
//================
#[derive(Clone, Debug)]
pub struct ResultType {
    pub ok_type: Type,
    pub err_type: Type
}

//================
//   PrimitiveType
//================
#[derive(Clone, Debug)]
pub struct PrimitiveType {
    pub id: Token,
}
 

//================
//   ListType
//================
#[derive(Clone, Debug)]
pub struct ListType{
    pub els_type: Box<Type>
}

//================
//   TupleType
//================
#[derive(Clone, Debug)]
pub struct TupleType{
    pub types: Box<Vec<Type>>
}

//================
//   StructType
//================
#[derive(Clone, Debug)]
pub struct StructType {
    name: Token,
    fields_types: StructFieldsTypes
}

//================
//   Struct
//================
#[derive(Debug)]
pub struct Struct{
    pub name: Token,
    pub attrs: Vec<Attr>,
    pub fields: Option<StructFields>,
    pub impls: Vec<Impl>
}

//================
//   StructImpl
//================
#[derive(Debug)]
pub struct StructImpl{
    pub name: Token,
    pub fns: Vec<Fn>,
}

//================
//   Impl
//================
#[derive(Debug)]
pub struct Impl{
    pub fns: Vec<Fn>
}


//================
//   Trait
//================
#[derive(Debug)]
pub struct Trait{
    pub name: Token,
    pub fns: Vec<TraitFn>,
    pub impls: Vec<Impl>
}


//================
//   TraitFn
//================
#[derive(Debug)]
pub struct TraitFn{
    pub name: Token,
    pub params: TuplePattern,
    pub ret_type: Option<Type>
}


//================
//   Enum
//================
#[derive(Debug)]
pub struct Enum{
    pub name: Token,
    pub variants: Vec<Variant>,
    pub impls: Vec<Impl>
}

//================
//   EnumImpl
//================
#[derive(Debug)]
pub struct EnumImpl{
    pub name: Token,
    pub fns: Vec<Fn>
}

//================
//   Variant
//================
#[derive(Debug)]
pub struct Variant {
    pub name: Token,
    pub value: Option<Expr>
}

//================
//   Pattern
//================
#[derive(Clone)]
pub enum Pattern {
    PrimitiveLiteral(Box<Expr>),
    Id(IdPattern),
    List(ListPattern),
    Tuple(TuplePattern),
    Struct(StructPattern),
    Enum(EnumPattern),
    Wildcard
}

impl Pattern {
    pub fn is_refutable(&self) -> bool {
        match self {
            Self::PrimitiveLiteral(_) => true,
            _ => false
        }
    }
}

//================
//   IdPattern
//================
#[derive(Clone)]
pub struct IdPattern {
    pub id: Token,
}


//================
//   ListPattern
//================
#[derive(Clone)]
pub struct ListPattern {
    pub items: Vec<Pattern>,
}

//================
//   TuplePattern
//================
#[derive(Clone, Debug)]
pub struct TuplePattern {
    pub items: Vec<Pattern>
}

//================
//   StructPattern
//================
#[derive(Clone)]
pub struct StructPattern {
    pub name: Option<Token>,
    pub items: StructPatternFields
}

//================
//   EnumPattern
//================
#[derive(Clone)]
pub struct EnumPattern {
    pub name : Option<Token>,
    pub variant: EnumVariant
}


//================
//   EnumVariant
//================
#[derive(Clone)]
pub struct EnumVariant {   
    pub name: Token,
    pub pattern: Option<Box<Pattern>>
}

//================
//   Attr
//================
#[derive(Clone, Debug)]
pub struct Attr {
    pub expr: AttrExpr
}

//================
//   AttrExpr
//================
#[derive(Clone, Debug)]
pub enum AttrExpr {
    Ref(Token)
}

//================
//   Match
//================
#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Box<Expr>,
    pub arms: Vec<Arm>
}


//================
//   Arm
//================
#[derive(Clone, Debug)]
pub struct Arm {
    pub pattern: Pattern,
    pub block: Vec<BlockElement>
}


//================
//   For
//================
#[derive(Clone, Debug)]
pub struct For {
    pub in_expr: InExpr,
    pub block: Vec<BlockElement>
}

//================
//   InExpr
//================
#[derive(Clone, Debug)]
pub struct InExpr {
    pub pattern: Pattern,
    pub expr: Box<Expr>,    
}

//================
//   While
//================
#[derive(Clone, Debug)]
pub struct While {
    pub expr: Box<Expr>,
    pub block: Box<Vec<BlockElement>>
}

//================
//   If
//================
#[derive(Clone, Debug)]
pub struct If {
    pub branches: Vec<IfBranch>
}

//================
//   IfBranch
//================
#[derive(Clone, Debug)]
pub struct IfBranch {
    pub expr: Option<Box<Expr>>,
    pub block: Vec<BlockElement>,
}


//================
//   Code
//================
#[derive(Clone, Debug)]
pub struct Code {
    pub lang: Token,
    pub code: Token,
}