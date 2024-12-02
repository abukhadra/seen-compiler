class Node          { id; t; v                          ;  constructor(id,t,v)         { this.id = id, this.t = t, this.v = v}                 }
class Mod           {                                   ;  constructor()               {}                                                      }
class Type          { t; o                              ;  constructor(t,o)            { this.t = t ; this.o = o }                             }
class TypeDef       { name; fields ; children           ;  constructor(name,fields,children)    { 
                                                                this.name = name 
                                                                this.fields = fields
                                                                this.children = children 
                                                            }                                  
}
class Value         { name; t ; body                    ; cosntructor(name,t,body) { this.name = name; this.t = t; this.body = body }          }
class Alias         { name; t                           ; constructor(name,t) { this.name = name; this.t = t }                                 }

class Struct       { name; fields ;                     ;  constructor(name,fields)    { 
                                                            this.name = name 
                                                            this.fields = fields
                                                        }                                  
}

class Enum         { name; variants ;                   ;  constructor(name,variants)    { 
                                                            this.name = name 
                                                            this.variants = variants
                                                        }                                  
}

class TypeDynamic   { fields; o                         ;  constructor(fields,o)       { this.fields = fields ; this.o = o }                   }
class TypeTempl     { t; ts; o                          ;  constructor(t, ts ,o)       { this.t = t; this.ts = ts ; this.o = o }               }
class EnumPat       { name; variant                     ;  constructor(name, variant)  { this.name = name ; this.variant = variant}            }
class Pair          { k; v                              ;  constructor(k,v)            { this.k = k ; this.v = v }                             }
class Uni           { opr; op                           ;  constructor(opr,op)         { this.opr = opr ; this.op = op }                       }
class Bin           { lopr; op; ropr                    ;  constructor(lopr,op,ropr)   { this.lopr = lopr ; this.op = op ; this.ropr = ropr }  }
class Method        { trait; instance_type; fn          ;  constructor(trait, instance_type,fn) { 
                                                                this.trait = trait;
                                                                this.instance_type = instance_type;
                                                                this.fn = fn
                                                            }                            
}
class Trait         { id; fns; sigs                     ;  constructor(id, fns, sigs)  { this.id = id; this.fns = fns; this.sigs = sigs }      }
class TraitImpl     { id; t; fns                        ;  constructor(id, t, fns)     { this.id = id; this.t = t; this.fns = fns }            }
class Fn            { attrs; typesig; modifier; name; params ;body  ;  constructor(attrs, typesig, modifier , name, params ,body)   {     
                                                                this.attrs = attrs ;     
                                                                this.typesig = typesig;
                                                                this.modifier = modifier;
                                                                this.name = name ; 
                                                                this.params = params ; 
                                                                this.body = body
                                                            }  
}
class TypeSig       { params; ret_type                  ; constructor(params,ret_type) { this.params = params ; this.ret_type = ret_type}     }
class FnSig         { name; params; ret_types           ;  constructor(name, params, ret_types) { 
                                                                this.name = name; 
                                                                this.params = params; 
                                                                this.ret_types = ret_types
                                                          }
}
class FnParam        { _pat; t                          ;   constructor(_pat,t)         { this._pat = _pat; this.t = t }                        }
class TrailingClosure{ label; fn                        ;   constructor(label,fn)        { this.label = label; this.fn = fn }                   }
class FnCall         { id; args; trailing               ;   constructor(id, args, trailing)     { 
                                                                this.id = id; 
                                                                this.args = args; 
                                                                this.trailing = trailing 
                                                            }
}
class Field         {id; t                             ;  constructor(id,t)            { this.id = id; this.t = t; }                           }
class FieldAsgmt    { id; expr                         ;  constructor(id,expr)         { this.id = id; this.expr = expr }                      }
class AnonymousMethod{ id; expr                        ;  constructor(stmts)           { this.stmts = stmts }                                  }
class StructLEl     { k; v                             ;  constructor(k,v)             { this.k = k; this.v = v }                              }
class Asgmt         { lhs; t; rhs                      ;  constructor(lhs, t ,rhs)     { this.lhs = lhs; this.t = t ; this.rhs = rhs }         }
class While         { expr; body                       ;  constructor(expr,body)       { this.expr = expr; this.body = body }                  }
class ForIn         { pat; expr; body                  ;  constructor(pat, expr, body) { this.pat = pat; this.expr = expr; this.body = body }  }                  
class When          { expr; arms                       ;  constructor(expr,arms)       { this.expr = expr; this.arms = arms }                  }
class WhenArm       { pats; expr                       ;  constructor(pats,expr)       { this.pats = pats; this.expr = expr }                  }

export {
    Node,
    Mod,
    Pair,
    Uni,
    Bin,
    Method,
    Trait,
    TraitImpl,
    Fn,
    TypeSig,
    FnSig,
    FnParam,
    TrailingClosure,
    FnCall,
    Field,
    FieldAsgmt,
    AnonymousMethod,
    TypeDef,
    Value,
    Alias,
    Struct,
    Enum,
    StructLEl,
    Asgmt,
    While,
    ForIn,
    When,
    WhenArm,
    Type,
    TypeDynamic,
    TypeTempl,
    EnumPat,
}