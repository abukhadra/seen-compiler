#![allow(warnings)]

pub mod build;
pub mod rust;
pub mod html;
pub mod react_native;

use crate::lang::syntax_tree::ast::{
    Attr,
    AttrExpr
};

//================
//   MobileOS
//================
#[derive(Debug)]
pub enum MobileOS {
    Android,
    IOS,
}

//================
//   is_attr()
//================
pub fn is_attr(
    name: &str,
    attrs: &Option<Vec<Attr>>
) -> bool {
    if let Some(attrs) = attrs {
        if let Some(attr) =  attrs.get(0) {
            is_ref_attr(name, attr) 
        } else {
            false
        }                     
    } else {
        false
    }
}

//================
//   is_ref_attr()
//================
fn is_ref_attr(
    name: &str, 
    attr: &Attr
) -> bool {
    match &attr.expr {
        AttrExpr::Ref(v) => {
            v.to_string().as_str() == name 
        }
    }
}