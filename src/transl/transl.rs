// FIXME: translations should be externalized to data files
//          for example if we decide to add new package manager / langauge support to the conf files, we shouldn't need to recompile the source code ( like py->... ),
//          just add it to the data files 
//          also, we can have duplicates in multiple translation files , seen can get rid of redundancy , flag difference, or use different terms / translations for different content
// FIXME: multiple terms, 
//              for example we can have python as both python / py in english... but in arabic it would be only بايثون
//              but we could also have a short/long form for both languages, such as :  install / i 
//              another case is having aliases,  we can have a package name that is readable in english but not in arabic, so we can give it an alias in addition to its original name
//              also what if the same word in a langauge have multiple meanings and different translations in the other language? 
// FIXME: add citation

use crate::lang::Lang;
use crate::transl::text::Text;

//================
//   Constants
//================
const CONF_AR :&str = "هيئة";
const CONF_EN :&str = "conf";

const SEEN_EXT_AR :&str = "س";
const SEEN_EXT_EN :&str = "seen";

const MISSING_CONF_ERR_AR : &str = "ملف هيئة.س مفقود";
const MISSING_CONF_ERR_EN : &str = "missing conf.seen";

//================
//   Transl
//================
#[derive(Debug)]
pub struct Transl {
    _proj_lang: Lang,
    seen_ext: Text,
    src: Text,
    main: Text,
    build: Text,

    // conf.seen
    conf: Text,
    name: Text,
    deps: Text,
    id: Text,
    prebuild: Text,
    rust: Text,
    rs: Text,
    python: Text,
    py: Text,
    missing_conf_err: Text,

    // conf.seen -> rust
    features: Text,
    f: Text,
    version: Text, 
    v: Text,
    

    // conf.seen -> python
    py_path: Text,
    pkg_man: Text, 

    // plotting example
    plot: Text,
    legend: Text,
    data: Text,
    size: Text,
    label_x: Text,
    label_y: Text,
    title: Text,

    // pip.seen
    exec: Text,
    install: Text,

    // translate.seen
    translate: Text,

    // web_server
    // title: Text,
    web_server: Text,
    settings: Text,
    hostname: Text,
    port: Text,
    localhost: Text,
    // title: Text,
    homepage: Text,
    content:  Text,              

    // html
    label: Text,

}

impl Transl {
    //---------------------
    //  new()
    //---------------------     
    pub fn new(_proj_lang: &Lang) -> Self {
        Self {
            _proj_lang: _proj_lang.clone(),
            // project
            seen_ext:           Text::new(      SEEN_EXT_AR,                  SEEN_EXT_EN                     ),
            src:                Text::new(      "مصدر",                       "src"                           ),
            main:               Text::new(      "رئيسي",                      "main"                          ),            
            build:              Text::new(      "بنية",                       "build"                         ),

            // conf.seen
            conf:               Text::new(      CONF_AR,                      CONF_EN                         ),            
            name:               Text::new(      "الاسم",                       "name"                          ),
            deps:               Text::new(      "تبعيات",                     "deps"                          ),
            id:                 Text::new(      "معرف",                       "id"                            ),
            prebuild:           Text::new(      "قبل_البناء",                 "prebuild"                      ),
            rust:               Text::new(      "رست",                        "rust"                          ),            
            rs:                 Text::new(      "رست",                        "rs"                            ),            
            python:             Text::new(      "بايثون",                     "python"                        ),
            py:                 Text::new(      "بايثون",                     "py"                            ),
            missing_conf_err:   Text::new(      MISSING_CONF_ERR_AR,          MISSING_CONF_ERR_EN             ),

            // conf.seen -> rust
            features:           Text::new(      "سمات",                       "features"                      ),
            f:                  Text::new(      "سمات",                       "f"                             ),        // FIXME: using f, while it's actually features, should it be feats? 
            version:            Text::new(      "اصدار",                      "version"                       ),
            v:                  Text::new(      "اصدار",                      "v"                             ),

            // conf.seen -> python
            py_path:            Text::new(      "مسار_بايثون",               "py_path"                        ),
            pkg_man:            Text::new(      "مدير_الحزم",                "pkg_man"                        ),

            // plotting example
            plot:               Text::new(      "رسم_بياني",                  "plot"                          ),
            legend:             Text::new(      "مفتاح",                      "legend"                        ),
            data:               Text::new(      "بيانات",                     "data"                          ),
            size:               Text::new(      "حجم",                        "size"                          ), 
            label_x:            Text::new(      "لصيقة_س",                    "label_x"                       ),
            label_y:            Text::new(      "لصيقة_ص",                    "label_y"                       ),
            title:              Text::new(      "عنوان",                      "title"                         ),

            // pip.seen
            exec:               Text::new(      "شغل",                        "exec"                          ),
            install:            Text::new(      "ثبت",                        "install"                       ),

            // translate.seen
            translate:          Text::new(      "ترجمة",                       "translate"                    ),

            // web_server
            web_server:         Text::new(      "مخدم_شع",                    "web_server"                    ),
            settings:           Text::new(      "اعدادات",                    "settings"                      ),
            hostname:           Text::new(      "اسم_المضيف",                 "hostname"                      ),
            port:               Text::new(      "منفذ",                       "port"                          ),
            localhost:          Text::new(      "المضيف_المحلي",              "localhost"                     ),
            // title:              Text::new(      "عنوان",                      "title"                          ),
            homepage:           Text::new(      "ص_رئيسية",                   "homepage"                      ),
            content:            Text::new(      "محتوى",                      "content"                       ),

            // html
            label:              Text::new(      "لصيقة",                      "label"                         ),

        }
    }

    //---------------------
    //  to_str()
    //--------------------- 
    pub fn to_str(
        &self, 
        text: &Text
    ) -> String {
        match self._proj_lang {
            Lang::Ar => text.ar.clone(),
            Lang::En => text.en.clone()
        }
    }

}

//================
//   Transl
//    project
//================
impl Transl {

    //---------------------
    //  seen_ext()
    //--------------------- 
    pub fn seen_ext(&self) -> String { self.to_str(&self.seen_ext) }        


    //---------------------
    //  seen_ext_ar()
    //--------------------- 
    pub fn seen_ext_ar() -> String { SEEN_EXT_AR.to_string() }        

    //---------------------
    //  seen_ext_en()
    //--------------------- 
    pub fn seen_ext_en() -> String { SEEN_EXT_EN.to_string() }            

    //---------------------
    //  src()
    //--------------------- 
    pub fn src(&self) -> String { self.to_str(&self.src) }        

    //---------------------
    //  main()
    //--------------------- 
    pub fn main(&self) -> String { self.to_str(&self.main) }        


    //---------------------
    //  build()
    //--------------------- 
    pub fn build(&self) -> String { self.to_str(&self.build) }        

}


//================
//   Transl
//    conf.seen
//================
impl Transl {

    //---------------------
    //  conf()
    //--------------------- 
    pub fn conf(&self) -> String { self.to_str(&self.conf) }    

    //---------------------
    //  conf_ar()
    //--------------------- 
    pub fn conf_ar() -> String { CONF_AR.to_string() }    

    //---------------------
    //  conf_en()
    //--------------------- 
    pub fn conf_en() -> String { CONF_EN.to_string() }    


    //---------------------
    //  name()
    //--------------------- 
    pub fn name(&self) -> String { self.to_str(&self.name) }        

    //---------------------
    //  deps()
    //--------------------- 
    pub fn deps(&self) -> String { self.to_str(&self.deps) }        

    //---------------------
    //  id()
    //--------------------- 
    pub fn id(&self) -> String { self.to_str(&self.id) }     

    //---------------------
    //  prebuild()
    //--------------------- 
    pub fn prebuild(&self) -> String { self.to_str(&self.prebuild) }         

    //---------------------
    //  rust()
    //--------------------- 
    pub fn rust(&self) -> String { self.to_str(&self.rust) }     

    //---------------------
    //  rs()
    //--------------------- 
    pub fn rs(&self) -> String { self.to_str(&self.rs) }     

    //---------------------
    //  python()
    //--------------------- 
    pub fn python(&self) -> String { self.to_str(&self.python) }     

    //---------------------
    //  py()
    //--------------------- 
    pub fn py(&self) -> String { self.to_str(&self.py) }     

    //---------------------
    //  missing_conf_err()
    //--------------------- 
    pub fn missing_conf_err(&self) -> String { self.to_str(&self.missing_conf_err) }             

    //---------------------
    //  missing_conf_err_ar()
    //--------------------- 
    pub fn missing_conf_err_ar() -> String { MISSING_CONF_ERR_AR.to_string() }     

    //---------------------
    //  missing_conf_err_en()
    //---------------------     
    pub fn missing_conf_err_en() -> String { MISSING_CONF_ERR_EN.to_string() }     


}


//================
//   Transl
//  conf.seen ->
//    Rust 
//================
impl Transl {

    //---------------------
    //  features()
    //--------------------- 
    pub fn features(&self) -> String { self.to_str(&self.features) }   

    //---------------------
    //  f()
    //--------------------- 
    pub fn f(&self) -> String { self.to_str(&self.f) }   

    //---------------------
    //  version()
    //--------------------- 
    pub fn version(&self) -> String { self.to_str(&self.version) }       

    //---------------------
    //  v()
    //--------------------- 
    pub fn v(&self) -> String { self.to_str(&self.v) }           
}


//================
//   Transl
//  conf.seen ->
//    Python 
//================
impl Transl {

    //---------------------
    //  py_path()
    //--------------------- 
    pub fn py_path(&self) -> String { self.to_str(&self.py_path) }   

    //---------------------
    //  pkg_man()
    //--------------------- 
    pub fn pkg_man(&self) -> String { self.to_str(&self.pkg_man) }       
}


//================
//   Transl
//    matplotlib 
//    example
//================
impl Transl {

    //---------------------
    //  plot()
    //--------------------- 
    pub fn plot(&self) -> String { self.to_str(&self.plot) }     

    //---------------------
    //  legend()
    //--------------------- 
    pub fn legend(&self) -> String { self.to_str(&self.legend) }     

    //---------------------
    //  data()
    //--------------------- 
    pub fn data(&self) -> String { self.to_str(&self.data) }     

    //---------------------
    //  size()
    //--------------------- 
    pub fn size(&self) -> String { self.to_str(&self.size) }     

    //---------------------
    //  label_x()
    //--------------------- 
    pub fn label_x(&self) -> String { self.to_str(&self.label_x) }                     

    //---------------------
    //  label_y()
    //--------------------- 
    pub fn label_y(&self) -> String { self.to_str(&self.label_y) }                     

    //---------------------
    //  title()
    //--------------------- 
    pub fn title(&self) -> String { self.to_str(&self.title) }                             
}

//================
//   Transl
//    pip.seen
//================
impl Transl {

    //---------------------
    //  exec()
    //--------------------- 
    pub fn exec(&self) -> String { self.to_str(&self.exec) }     

    //---------------------
    //  install()
    //--------------------- 
    pub fn install(&self) -> String { self.to_str(&self.install) }         
}

//================
//   Transl
//    translate.seen
//================
impl Transl {

    //---------------------
    //  translate()
    //--------------------- 
    pub fn translate(&self) -> String { self.to_str(&self.translate) }     
}



//================
//   Transl
//    web_server
//================
impl Transl {

    //---------------------
    //  web_server()
    //--------------------- 
    pub fn web_server(&self) -> String { self.to_str(&self.web_server) }     

    //---------------------
    //  settings()
    //--------------------- 
    pub fn settings(&self) -> String { self.to_str(&self.settings) }         

    //---------------------
    //  hostname()
    //--------------------- 
    pub fn hostname(&self) -> String { self.to_str(&self.hostname) }             

    //---------------------
    //  port()
    //--------------------- 
    pub fn port(&self) -> String { self.to_str(&self.port) }          

    //---------------------
    //  localhost()
    //--------------------- 
    pub fn localhost(&self) -> String { self.to_str(&self.localhost) }              

    //---------------------
    //  homepage()
    //--------------------- 
    pub fn homepage(&self) -> String { self.to_str(&self.homepage) }    

    //---------------------
    //  content()
    //--------------------- 
    pub fn content(&self) -> String { self.to_str(&self.content) }            
}


//================
//   Transl
//    html
//================
impl Transl {

    //---------------------
    //  label()
    //--------------------- 
    pub fn label(&self) -> String { self.to_str(&self.label) }     
}
