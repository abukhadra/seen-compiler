use std::collections::HashMap;


use crate::lang::{
    symtab::*,
    error::Error
};


//================
// ResTab
//================
#[derive(Debug)]
pub struct ResTab {
    entries: HashMap<RefId, RefedId>
}

impl ResTab {
    //---------------------
    //  new()
    //---------------------         
    pub fn new() -> Self {
        Self {
            entries: HashMap::new()
        }
    }

    //---------------------
    //  insert()
    //---------------------     
    pub fn insert(
        &mut self,
        ref_id: RefId, 
        refed_id: RefedId
    ) {
        self.entries.insert(ref_id, refed_id);
    }
}


//================
// Resolver
//================
pub struct Resolver {
    // symtab: Option<SymTab>,
    restab: Option<ResTab>,
    errors: Option<Vec<Error>>,
}

impl Resolver {
    //---------------------
    //  new()
    //---------------------     
    pub fn new() -> Self {
        Self{
            restab: Some(ResTab{ entries: HashMap::new()}),
            errors: None
        }
    }

    //---------------------
    //  init()
    //---------------------    
    pub fn init(
        &mut self,
    ) {
        // self.symtab = Some(symtab);
        self.errors = Some(vec![]);
        
    }        

    //---------------------
    //  resolve()
    //---------------------    
    pub fn resolve(
        &mut self,
        symtab: SymTab
    ) -> (SymTab, ResTab, Vec<Error>) {
        self.init();

        let module_scope = symtab.module_scope();

        self.search_scopes(&module_scope, &symtab);

        (
            symtab,
            self.restab.take().unwrap(),
            self.errors.take().unwrap()
        )
    }

    //---------------------
    //  insert_err()
    //---------------------        
    fn insert_err(
        &mut self, 
        error: Error
    ) {
        self.errors.as_mut().unwrap().push(error);
    }    

    //---------------------
    //  search_scopes()
    //---------------------    
    pub fn search_scopes(
        &mut self,
        scope: &Scope,
        symtab: &SymTab
    ) {
        for (i, entry) in scope.entries
                                .iter()
                                .enumerate() {
            match entry {
                Entry::Ref(_ref) => {
                    let ref_id = (scope.id, i);
                    let refed_id = scope.deref(i, symtab);
                    match refed_id {
                        Err(err) =>  self.insert_err(err),
                        Ok(refed_id) => self.restab.as_mut().unwrap().insert(ref_id, refed_id)
                    }
                    
                },
                Entry::Scope(id) => {
                }
                _ => ()

            }
        }

    }
}