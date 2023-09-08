use super::{
    resolver::ResTab, 
    syntax_tree::ast::ModElement,
    error::{
        Error,
    },    
};

pub struct TypeChecker {
    ast: Option<Vec<ModElement>>,
    restab: Option<ResTab>,
    errors: Option<Vec<Error>>,    
}

impl TypeChecker {
    //---------------------
    //  new()
    //---------------------    
    pub fn new() -> Self {
        Self {
            ast: None,
            restab: None,
            errors: None
        }
    }

    //---------------------
    //  init()
    //---------------------    
    pub fn init(
        &mut self,
        ast: Vec<ModElement>,    
        restab: ResTab
    ) {
        self.ast = Some(ast);
        self.restab = Some(restab);
        self.errors = Some(vec![]);
        
    }    


    //---------------------
    //  infer()
    //---------------------    
    pub fn check(
        &mut self, 
        ast: Vec<ModElement>, 
        restab: ResTab
    ) -> (Vec<ModElement>, ResTab, Vec<Error>)  {
        self.init(ast, restab);

        (
            self.ast.take().unwrap(),
            self.restab.take().unwrap(),
            self.errors.take().unwrap()
        )        
    }
}