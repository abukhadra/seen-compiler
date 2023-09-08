pub mod actix;

use crate::{
    target::{
        is_attr,
        rust::rs_gen::Rust
    }, 
    lang::syntax_tree::ast::{
        BlockElement,
        Attr,
        Expr, StructLiteral
    }
};

//================
//   Constants
//================
const INDEX_HTML: &'static str = "index.html";

//================
//   web_server_main()
//================
impl <'a> Rust<'a> {
    pub fn web_server_main(
        &mut self,
        els: &'a Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>
    ) {
    
        for el in els {
            match el {
                BlockElement::Expr(Expr::Ret(v)) => {
                    let v = &**v;
                    match v {
                        Expr::StructLiteral(data) => {
                            // FIXME just a quick hack to demo the project, in the real app, the attributes will alter the ast 
                            if is_attr("web_server", attrs) || is_attr("مخدم_شع", attrs){
                                self.web_server(&data);
                            }
                        },
                        _ => todo!()   // FIXME
                    }
                    
                }
                _ => todo!() // TODO
            }
        }
    }
}

//================
//  web_server()
//================  
impl <'a> Rust<'a> { 
    pub fn  web_server(
        &mut self,
        data: &'a StructLiteral
    ) {
        let mut path = self.proj_dir.res.pages.clone();
        path.push(INDEX_HTML);
        let index_html = self.html
                        .as_mut()
                        .unwrap()
                        .page(&mut path, data);
        self.actix(data);

    }
}
