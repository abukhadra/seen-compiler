// FIXME: split translations to multiple files: errors, Transl , messages ...etc

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
pub struct Transl {
    _proj_lang: Lang,
    conf: Text,
    seen_ext: Text,
    build: Text,
    src: Text,
    main: Text,
    name: Text,
    missing_conf_err: Text

}

impl Transl {
    //---------------------
    //  new()
    //---------------------     
    pub fn new(_proj_lang: Lang) -> Self {
        Self {
            _proj_lang,
            conf:               Text::new(      CONF_AR,                      CONF_EN                         ),
            seen_ext:           Text::new(      SEEN_EXT_AR,                  SEEN_EXT_EN                     ),
            build:              Text::new(      "بنية",                       "build"                         ),
            src:                Text::new(      "مصدر",                       "src"                           ),
            main:               Text::new(      "رئيسي",                      "main"                          ),
            name:               Text::new(      "الاسم",                       "name"                          ),
            missing_conf_err:   Text::new(      MISSING_CONF_ERR_AR,          MISSING_CONF_ERR_EN             ),
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

    //---------------------
    //  conf_ar()
    //--------------------- 
    pub fn conf_ar() -> String { CONF_AR.to_string() }    

    //---------------------
    //  conf_en()
    //--------------------- 
    pub fn conf_en() -> String { CONF_EN.to_string() }    

    //---------------------
    //  conf()
    //--------------------- 
    pub fn conf(&self) -> String { self.to_str(&self.conf) }    

    //---------------------
    //  seen_ext_ar()
    //--------------------- 
    pub fn seen_ext_ar() -> String { SEEN_EXT_AR.to_string() }        

    //---------------------
    //  seen_ext_en()
    //--------------------- 
    pub fn seen_ext_en() -> String { SEEN_EXT_EN.to_string() }            

    //---------------------
    //  seen_ext()
    //--------------------- 
    pub fn seen_ext(&self) -> String { self.to_str(&self.seen_ext) }        

    //---------------------
    //  build()
    //--------------------- 
    pub fn build(&self) -> String { self.to_str(&self.build) }        

    //---------------------
    //  src()
    //--------------------- 
    pub fn src(&self) -> String { self.to_str(&self.src) }        

    //---------------------
    //  main()
    //--------------------- 
    pub fn main(&self) -> String { self.to_str(&self.main) }        

    //---------------------
    //  name()
    //--------------------- 
    pub fn name(&self) -> String { self.to_str(&self.name) }        

    //---------------------
    //  missing_conf_err_ar()
    //--------------------- 
    pub fn missing_conf_err_ar() -> String { MISSING_CONF_ERR_AR.to_string() }     

    //---------------------
    //  missing_conf_err_en()
    //---------------------     
    pub fn missing_conf_err_en() -> String { MISSING_CONF_ERR_EN.to_string() }     

    //---------------------
    //  missing_conf_err()
    //--------------------- 
    pub fn missing_conf_err(&self) -> String { self.to_str(&self.missing_conf_err) }        

}