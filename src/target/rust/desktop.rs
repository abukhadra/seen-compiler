pub mod tauri;

use std::fs;

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
//   desktop_main()
//================
impl <'a> Rust<'a> {
    pub fn desktop_main(
        &mut self,
        els: &Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>
    ) {
    
        for el in els {
            match el {
                BlockElement::Expr(Expr::Ret(v)) => {
                    let v = &**v;
                    match v {
                        Expr::StructLiteral(data) => {
                            // FIXME just a quick hack to demo the project, in the real app, the attributes will alter the ast 
                            if is_attr("desktop", attrs) || is_attr("مكتبي", attrs){
                                self.desktop(&data);
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
//  desktop()
//================  
impl <'a> Rust<'a> { 
    pub fn desktop(
        &mut self,
        data: &StructLiteral
    ) {
        let mut home = self.proj_dir.home.clone();
        let mut js_dir = home.clone();
        js_dir.push("js");
        fs::create_dir_all(&js_dir).expect("expecting icons dir to be created");		

        let mut html_file = js_dir;
        html_file.push(INDEX_HTML);
        let index_html = self.html
                        .as_mut()
                        .unwrap()
                        .page(&mut html_file, data);
        self.tauri_desktop(&home, data);

    }
}
