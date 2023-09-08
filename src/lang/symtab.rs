use std::collections::{
    HashMap,
    BTreeSet
};

use crate::error;

use crate::lang::{
    token::Token,
    syntax_tree::ast::*,
    error::{
        Error,
    },    
};

//================
//   Aliases
//================
pub type ScopeId = usize;
pub type EntryId = usize;
pub type RefId = (ScopeId, EntryId);
pub type RefedId = (ScopeId, EntryId);

//================
//   SymTab
//================
#[derive(Debug)]
pub struct SymTab {
    pub scopes: Vec<Scope>,
    stack: Vec<ScopeId>
}

impl SymTab {
    //---------------------
    //  new()
    //--------------------- 
    pub fn new() -> Self{
        let id = 0;        
        Self {
            scopes: vec![Scope::new(id, None)],
            stack: vec![id]
        }
    }

    //---------------------
    //  new_scope()
    //--------------------- 
    pub fn new_scope(
        &mut self,
    ) -> ScopeId {
        let id = self.scopes.len();
        let parent_id = self.stack
                            .last()
                            .unwrap()
                            .clone();
        let scope = Scope::new(
            id, 
            Some(parent_id),
        );
        self.scopes.push(scope);
        self.enter_scope(id);

        id
    }

    //---------------------
    //  module_scope()
    //--------------------- 
    pub fn module_scope(&self) -> &Scope {
        &self.scopes[0]     
    }

    //---------------------
    //  current_scope()
    //--------------------- 
    fn current_scope(
        &mut self,
    ) -> &mut Scope {
        let id = self.stack
                    .last()
                    .unwrap()
                    .clone();

        &mut self.scopes[id]
    }

    //---------------------
    //  enter_scope()
    //--------------------- 
    pub fn enter_scope(
        &mut self,
        id: ScopeId
    ) {
        self.stack.push(id);
        self.current_scope()
            .insert_scope(id);
    }

    //---------------------
    //  exit_scope()
    //--------------------- 
    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    //---------------------
    //  insert_ref()
    //--------------------- 
    pub fn insert_ref(
        &mut self,
        t: &Token
    ) {
        self.current_scope()
            .insert_ref(t)
    }        


    //---------------------
    //  insert_id_pattern()
    //--------------------- 
    pub fn insert_id_pattern(
        &mut self,        
        t: &Token
    ) {
        self.current_scope()
            .insert_id_pattern(t)
    }    

    //---------------------
    //  insert_decl()
    //--------------------- 
    pub fn insert_decl(
        &mut self,        
        decl: &Decl
    ) -> Result<(), Error> {
        self.current_scope()
            .insert_decl(decl)
    }    

    //---------------------
    //  insert_fn()
    //--------------------- 
    pub fn insert_fn(
        &mut self,        
        _fn: &Fn
    ) -> Result<(), Error> {
        self.current_scope()
            .insert_fn(_fn)
    }        

    //---------------------
    //  insert_struct()
    //--------------------- 
    pub fn insert_struct(
        &mut self,        
        _struct: Struct
    ) -> Result<(), Error> {
        self.current_scope()
            .insert_struct(_struct)
    }            

    //---------------------
    //  insert_trait()
    //--------------------- 
    pub fn insert_trait(
        &mut self,        
        _trait: Trait
    ) -> Result<(), Error> {
        self.current_scope()
            .insert_trait(_trait)
    }            

    //---------------------
    //  insert_enum()
    //--------------------- 
    pub fn insert_enum(
        &mut self,        
        _enum: Enum
    ) -> Result<(), Error> {
        self.current_scope()
            .insert_enum(_enum)
    }                

}

//================
//   Scope
//================
#[derive(Debug)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    pub set: BTreeSet<String>,
    pub defs: HashMap<String, EntryId>,
    pub entries: Vec<Entry>
}    

impl Scope {
    //---------------------
    //  new()
    //---------------------        
    pub fn new(
        id: ScopeId, 
        parent_id: Option<ScopeId>
    ) -> Self {
        Self {
            id,
            parent_id,
            set: BTreeSet::new(),
            defs: HashMap::new(),
            entries: vec![]
        }
    }   

    //---------------------
    //  deref()
    //---------------------        
    pub fn deref(
        &self,
        id: EntryId,
        symtab: &SymTab
    ) -> Result<RefedId, Error> {
        let mut res = None;
        let entry = &self.entries[id];
        let mut index = id.clone();
        while index > 0 {
            index  -= 1;
            let second = &self.entries[index];
            if self.match_decl(&entry, &second) {
                res = Some((self.id, index));
            }
        }

        match res {
            None => {
                match self.parent_id {
                    None => Err(
                        error!(
                            format!("could not resolve: {}", entry.sym()),
                            entry.token()
                        )
                    ),
                    Some(parent_id) => {
                        let parent = &symtab.scopes[parent_id];
                        parent.deref(id , symtab)
                    }
                }
            },
            Some(res) => Ok(res)
        }

        
    }   

    //---------------------
    //  match_decl()
    //---------------------        
    pub fn match_decl(
        &self,
        first: &Entry,
        second:&Entry
    ) -> bool {    
        if let Entry::Decl(_) = second {
            first.sym() == second.sym()
        } else {
            false
        }
        

    }



    //---------------------
    //  check_unique()
    //--------------------- 
    fn check_unique(
        &mut self,
        token: &Token
    ) -> Result<(), Error> {
        let id = token.value.to_string();
        if !self.set.insert(id.clone()) {
            Err(
                error!(
                    format!("duplicate identifier: {}", id),
                    token
                )
            )
        } else {
            Ok(())
        }
    }

    //---------------------
    //  insert_ref()
    //--------------------- 
    pub fn insert_ref(
        &mut self,
        t: &Token
    ) {
        self.entries.push(
            Entry::Ref(
                RefInfo { token: t.clone() }
            )
        );
    }         

    //---------------------
    //  insert_id_pattern()
    //--------------------- 
    pub fn insert_id_pattern(
        &mut self,
        t: &Token
    ) {

        self.entries.push(
            Entry::IdPattern(
                IdPatternInfo { token: t.clone() }
            )
        );
    }         


    //---------------------
    //  insert_decl()
    //--------------------- 
    // FIXME, is this needed? we only need to insert ID Pattern
    pub fn insert_decl(
        &mut self,
        decl: &Decl
    ) -> Result<(), Error>{
        match &decl.pattern {
            Pattern::Id(id_pat) => self.check_unique(&id_pat.id)?,
            _ => todo!()
        }

        self.entries.push(
            Entry::Decl(
                DeclInfo { 
                    pattern: decl.pattern.clone()
                }
            )
        );
        Ok(())        
    }         

    //---------------------
    //  insert_fn()
    //--------------------- 
    pub fn insert_fn(
        &mut self,
        _fn: &Fn
    ) -> Result<(), Error>{
        match &_fn.name {
            None => Ok(()),
            Some(token) => {
                self.check_unique(&token)?;
                let id = self.insert_entry(
                    Entry::Fn(
                        FnInfo { 
                            name: _fn.name.clone(),
                            params: _fn.params.clone()
                        }                            
                    )
                );
                self.defs.insert(
                    token.value.to_string(),
                    id
                );
                Ok(())  
            }
        } 
    }

    //---------------------
    //  insert_struct()
    //--------------------- 
    pub fn insert_struct(
        &mut self,
        _struct: Struct
    ) -> Result<(), Error>{
        self.check_unique(&_struct.name)?;
        let id = self.insert_entry(
            Entry::Struct(
                StructInfo { 
                    name: _struct.name.clone(),
                }
            )
        );

        self.defs.insert(
            _struct.name.value.to_string(),
            id
        );
        Ok(())  
    }         

    //---------------------
    //  insert_trait()
    //--------------------- 
    pub fn insert_trait(
        &mut self,
        _trait: Trait
    ) -> Result<(), Error>{
        self.check_unique(&_trait.name)?;
        let id = self.insert_entry(
            Entry::Trait(
                TraitInfo { 
                    name: _trait.name.clone(),
                }
            )
        );        
        self.defs.insert(
            _trait.name.value.to_string(),
            id
        );
        Ok(())        
    }         


    //---------------------
    //  insert_enum()
    //--------------------- 
    pub fn insert_enum(
        &mut self,
        _enum: Enum
    ) -> Result<(), Error>{
        self.check_unique(&_enum.name)?;
        let id = self.insert_entry( 
            Entry::Enum(
                EnumInfo { 
                    name: _enum.name.clone(),
                }
            )
        );
        self.defs.insert(
            _enum.name.value.to_string(),
            id
        );
        Ok(())           
    }         


    //---------------------
    //  insert_scope()
    //--------------------- 
    pub fn insert_scope(
        &mut self,
        id: ScopeId
    ) {
        self.insert_entry(Entry::Scope(id));
    }                   

    //---------------------
    //  insert_entry()
    //--------------------- 
    pub fn insert_entry(
        &mut self,
        entry: Entry
    ) -> EntryId {
        let id = self.entries.len();
        self.entries.push(entry);
        id
        
    }                     

}

//================
//   Entry
//================
#[derive(Debug)]
pub enum Entry {
    Ref(RefInfo),    
    IdPattern(IdPatternInfo),
    Decl(DeclInfo),
    Fn(FnInfo),
    Struct(StructInfo),
    Trait(TraitInfo),
    Enum(EnumInfo),
    Scope(ScopeId)
}

impl Entry {
    //---------------------
    //  token()
    //---------------------     
    pub fn token(&self) -> &Token {
        match self {
            Self::Ref(info) => &info.token,
            Self::IdPattern(info) => &info.token,
            Self::Decl(info) => {
                match &info.pattern {
                    Pattern::Id(id_pat) => &id_pat.id,
                    _ => todo!()
                }
            },
            Self::Fn(info) => info.name.as_ref().unwrap(),   // FIXME , this will fail for lambda
            Self::Struct(info) => &info.name,
            Self::Trait(info) => &info.name,
            Self::Enum(info) => &info.name,
            _ => panic!()
        }
    }

    //---------------------
    //  sym()
    //---------------------     
    pub fn sym(&self) -> String {
        match self {
            Self::Ref(info) => info.token.to_string(),
            Self::IdPattern(info) => info.token.to_string(),
            Self::Decl(info) => {
                match &info.pattern {
                    Pattern::Id(id_pat) => id_pat.id.to_string(),
                    _ => todo!()
                }
            },
            Self::Fn(info) => info.name.as_ref().unwrap().to_string(),  // FIXME , this will fail for lambda
            Self::Struct(info) => info.name.to_string(),
            Self::Trait(info) => info.name.to_string(),
            Self::Enum(info) => info.name.to_string(),
            _ => panic!()
        }
    }
}


//================
//   RefInfo
//================
#[derive(Debug)]
pub struct RefInfo {
    pub token: Token,
}

//================
//   IdPatternInfo
//================
#[derive(Debug)]
pub struct IdPatternInfo {
    pub token: Token,
}

//================
//   DeclInfo
//================
#[derive(Debug)]
pub struct DeclInfo {
    pub pattern: Pattern
}

//================
//   FnInfo
//================
#[derive(Debug)]
pub struct FnInfo {
    pub name: Option<Token>,
    pub params: Vec<Param>
}

//================
//   StructInfo
//================
#[derive(Debug)]
pub struct StructInfo {
    pub name: Token
}

//================
//   TraitInfo
//================
#[derive(Debug)]
pub struct TraitInfo {
    pub name: Token
}

//================
//   EnumInfo
//================
#[derive(Debug)]
pub struct EnumInfo {
    pub name: Token
}
