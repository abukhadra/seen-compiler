use std::{
    fs,
    path::PathBuf, 
    fmt::Write
};

use crate::{
    tool::npx::NPX,
    target::{
        is_attr,
        MobileOS,
        build::BuildDir,
        html::html_gen::Html,        
    }, 
    lang::{
        Lang,
        syntax_tree::ast::*
    },
    util::{
        ar::to_western_num,
        indent::Indent,
        data::data_iter
    
    }    
};

//================
//   Constants
//================
const APP_TSX: &'static str = "App.tsx";

//================
//   ReactNative
//================
pub struct ReactNative<'a> {
    pub src_lang: Lang,
    pub proj_name: String,
    pub path: String,
    pub indent: Indent,
    pub res: String,
    pub html: Option<Html>,
    pub proj_dir : &'a mut BuildDir,
    pub imports: Vec<String>,       // FIXME: vector of imported modules... used as a workaround for not having a resolver / semantic analyzer
                                    //          for now , we are supporint  module name ( single token ) imports
                                    //          during code generation, a reference will be checked if it exists in this vector, if it does , then we will use :: rather than . to access elements
                                    //          later on, the resolver / semantic analyzer should eliminate the need for this workaround
    pub redirect: bool                                     
}

impl <'a> ReactNative<'a> {
    //---------------------
    //  new()
    //---------------------
    pub fn new (
        project_struct: &'a mut BuildDir,
        redirect: bool
    ) -> Self {
        Self {
            src_lang: Lang::Ar,
            proj_name: String::new(),
            path: String::new(),
            indent: Indent::new(),
            res: String::new(),
            html: None,
            proj_dir: project_struct,
            imports: vec![],
            redirect
        }
    }

    //---------------------
    //  generate()
    //---------------------
    pub fn generate(
        &mut self,
        proj_name: &String,
        mut file_name: String,
        path: &String,
        src_lang: &Lang, //&str,
        ast: &'a mut Vec<ModElement>,
        main_mods: &Vec<String>,
    ) {

        self.src_lang = src_lang.clone();
        self.proj_name = proj_name.clone();
        self.path = path.clone();

        for el in ast.iter() {
            match el {
                // ModElement::Decl(el) => self.asgmt(el),
                ModElement::Decl(el) => self.decl(&el),
                ModElement::MainFn(el) => self.main_fn(el, main_mods),
                ModElement::Fn(el) => self._fn(el),
                ModElement::Struct(el) =>  self._struct(el),
                ModElement::StructImpl(el) => self.struct_impl(el),
                ModElement::Trait(el) => self._trait(el),
                ModElement::Enum(el) => self._enum(el),
                ModElement::EnumImpl(el) => self.enum_impl(el),                
            }
        }      
    }
}

//================
//   main_fn()
//================
impl <'a> ReactNative<'a> {
    // FIXME hardcoding 
    fn main_fn(
        &mut self,
        mut _fn: &'a Fn,
        main_mods: &Vec<String>
    ) {
        if is_attr("mobile", &_fn.attrs) || is_attr("محمول", &_fn.attrs) || 
            is_attr("ios", &_fn.attrs) || is_attr("آي_أو_إس", &_fn.attrs) || 
            is_attr("android", &_fn.attrs) || is_attr("اندرويد", &_fn.attrs) {     
                self.react_native_main(&self.proj_name.clone(), &_fn.block, &_fn.attrs);
            } else {
                todo!()
            }
    }
}

//================
//  react_native_main()
//================  
impl <'a> ReactNative<'a> {
    pub fn react_native_main(
        &mut self,
        proj_name: &String,
        els: &'a Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>

    ) {
        let home = self.proj_dir.home.clone();

        for el in els {
            match el {
                BlockElement::Expr(Expr::Ret(v)) => {
                    let v = &**v;
                    match v {
                        Expr::StructLiteral(data) => {
                            self.react_native_app_tsx(&home, data);
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
//  react_native_app_tsx()
//================  
// FIXME: hardcoding
impl <'a> ReactNative<'a> {
    fn  react_native_app_tsx(
        &mut self,
        path: &PathBuf,
        data: &'a StructLiteral
    ) {

        let mut app_tsx_path = path.clone();
        app_tsx_path.push("App.tsx");
        let mut app_tsx = String::new();

        for (k,v) in data_iter(data) {
            match k.to_string().as_str() {
                "view" | "عرض" => {
                    app_tsx = self.view(v);
                    break;
                },
                _ => todo!("{}", k)
            }
        };     
        fs::write(app_tsx_path, app_tsx).unwrap();       
    }
}


//================
//   view()
//================
impl <'a> ReactNative<'a> {     
    pub fn view(
        &mut self,
        data: &Option<Expr>
    ) -> String {

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

        format!( 
r#"import {{
  SafeAreaView,
  Text,
}} from 'react-native';


function App(): JSX.Element {{

  return (
    <SafeAreaView>
            <Text>{}</Text>
     </SafeAreaView>
  );
}}

export default App;"#,
    content
)
 
    }
}

//================
//  decl()
//================  
impl <'a> ReactNative<'a> {
    fn  decl(
        &mut self,
        mut decl: &'a Decl,        
    ) {
        todo!()
    }
}

//================
//  _fn()
//================  
impl <'a> ReactNative<'a> {
    fn  _fn(
        &mut self,
        mut _fn: &'a Fn,        
    ) {
        todo!()
    }
}

//================
//  _struct()
//================  
impl <'a> ReactNative<'a> {
    fn  _struct(
        &mut self,
        mut _struct: &'a Struct,        
    ) {
        todo!()
    }
}

//================
//  struct_impl()
//================  
impl <'a> ReactNative<'a> {
    fn  struct_impl(
        &mut self,
        mut struct_impl: &'a StructImpl,        
    ) {
        todo!()
    }
}

//================
//  _trait()
//================  
impl <'a> ReactNative<'a> {
    fn  _trait(
        &mut self,
        mut _trait: &'a Trait,        
    ) {
        todo!()
    }
}

//================
//  _enum()
//================  
impl <'a> ReactNative<'a> {
    fn  _enum(
        &mut self,
        mut _enum: &'a Enum,        
    ) {
        todo!()
    }
}

//================
//  enum_impl()
//================  
impl <'a> ReactNative<'a> {
    fn  enum_impl(
        &mut self,
        mut enum_impl: &'a EnumImpl,        
    ) {
        todo!()
    }
}