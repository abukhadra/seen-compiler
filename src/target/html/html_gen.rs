use std::{
    fs,
    fmt::{
        Write,
    }, 
    path::PathBuf
};


use crate::lang::{
    token::TokenValue,
    Lang,
    syntax_tree::ast::{
        StructLiteral,
        Expr
    }
};

use crate::util::indent::Indent;

//================
//   Constants
//================
const HTML_EXT: &'static str = "html";

//================
//   Html
//================
pub struct Html{
    src_lang: Lang,
    path: String,
    seen_ext: String,
    indent: Indent,
    res: String
}

impl Html {
    //---------------------
    //  new()
    //---------------------    
    pub fn new (
        src_lang: &Lang,
        path: &String, 
        seen_ext: &str
    ) -> Self {
        Self {
            src_lang: src_lang.clone(),
            path: path.clone(),
            seen_ext: seen_ext.to_string(),
            indent: Indent::new(),
            res: String::new()
        }
    }    
}

//================
//   page()
//================
impl Html {     
    pub fn page(
        &mut self,
        path: &mut PathBuf,
        data: &StructLiteral
    ) {

        let _ = writeln!(
            self.res, 
            "{}",
            match self.src_lang {
                Lang::Ar => "<html dir=\"rtl\">",
                Lang::En => "<html>",
            }
        );
        self.indent.inc();

        let iter = if let Some((t, Some(expr))) = data.items.get(0) {

            let mut iter = data.items.iter();
            match &t.value  {
                TokenValue::Id(x) => {
                    if x == "data" ||  x == "root" || x == "بيانات" || x == "جذر" {   // FIXME : separate ar/en
                        match expr {
                            Expr::StructLiteral(literal) => {
                                iter = literal.items.iter();
                            }
                            _ => ()
                        }    
                    }
                },
                _ => ()
            }
            iter
    
        } else {
            data.items.iter()
        };

        // for (k,v) in data.items.iter() {
            for (k,v) in iter {
            match k.to_string().as_str() {
                "homepage" | "ص_رئيسية" => {
                    self.homepage(v);
                    break;

                },
                "view" | "عرض" => {
                    self.view(v);
                    break;

                },

                _ => ()
            };
        }
       
        let _ = writeln!(self.res, "</html>");
        
        match fs::write(&path, &self.res){
            Err(err) => panic!("path: {:?}, {:?}", path, err),
            Ok(_) => ()
        }
    }
}


//================
//   homepage()
//================
impl Html {     
    pub fn homepage(
        &mut self,
        data: &Option<Expr>
    ) {
        
        let mut title = &None;
        let mut content = &None;

        let data = match data { 
            Some(Expr::StructLiteral(sruct_literal)) => sruct_literal,
            _ => panic!("expecting data")
        };

        // FIXME: item lookup should be done in order, not just a loop. for example we should look for title then body,
        //          currently the code does not take order into consideration
        for (k,v) in data.items.iter() {
            match k.to_string().as_str() {
                "title" | "عنوان"=> {                                   
                    title = v;
                },
                "content" | "محتوى"=> {
                        content = v;  
                },
                _ => panic!("unsupported: {:?}", k)
            }
        }

        let title = if let Some(title) = title { 
            format!("{}", title) 
        } else { 
            String::new() 
        };

        let content = if let Some(content) = content { 
            format!("{}", content) 
        } else { 
            String::new() 
        };        

        let _ = writeln!(self.res, "{}<head>", self.indent);
        self.indent.inc();        
        let _ = writeln!(self.res, "{}<meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\" />", self.indent);
        let _ = writeln!(self.res, "{}<title>{}</title>", self.indent, title);
        self.indent.dec();        
        let _ = writeln!(self.res, "{}</head>", self.indent);

        let _ = writeln!(self.res, "{}<body>", self.indent);
        self.indent.inc();        
        let _ = writeln!(self.res, "{}{}", self.indent, content);
        self.indent.dec();
        let _ = writeln!(self.res, "{}</body>", self.indent);
        self.indent.dec();

    }
}

//================
//   view()
//================
impl Html {     
    pub fn view(
        &mut self,
        data: &Option<Expr>
    ) {

        let mut title = &None;
        let mut content = &None;

        let data = match data { 
            Some(Expr::StructLiteral(sruct_literal)) => sruct_literal,
            _ => panic!("expecting data")
        };

        // FIXME: item lookup should be done in order, not just a loop. for example we should look for title then body,
        //          currently the code does not take order into consideration
        for (k,v) in data.items.iter() {
            match k.to_string().as_str() {
                "title" | "عنوان"=> {                                   
                    title = v;
                },
                "content" | "محتوى"=> {
                        content = v;  
                },
                _ => panic!("unsupported: {:?}", k)
            }
        }

        let title = if let Some(title) = title { 
            format!("{}", title) 
        } else { 
            String::new() 
        };

        let content = if let Some(content) = content { 
            format!("{}", content) 
        } else { 
            String::new() 
        };        

        let _ = writeln!(self.res, "{}<head>", self.indent);
        self.indent.inc();        
        let _ = writeln!(self.res, "{}<meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\" />", self.indent);
        let _ = writeln!(self.res, "{}<title>{}</title>", self.indent, title);
        self.indent.dec();        
        let _ = writeln!(self.res, "{}</head>", self.indent);

        let _ = writeln!(self.res, "{}<body>", self.indent);
        self.indent.inc();        
        let _ = writeln!(self.res, "{}{}", self.indent, content);
        self.indent.dec();
        let _ = writeln!(self.res, "{}</body>", self.indent);
        self.indent.dec();

    }
}