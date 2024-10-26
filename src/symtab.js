// FIXME: workaround, storing fn names here , until name resolution  is complete.

export class Symtab {
    /*tmp*/fns 
    /*tmp*/structs      
    /*tmp*/receivers

    root
    stack
    current

    constructor() {
        /*tmp*/this.fns = []    
        /*tmp*/this.structs = []
        /*tmp*/this.receivers = {}
        
        this.root =  new Table(null) 
        this.stack = [] 
        this.stack.push(this.root)
        this.current = this.root
    }

    /*tmp*/insert_fn(fn) { this.fns.push(fn)}
    /*tmp*/insert_struct(struct) { this.structs.push(struct) }
    /*tmp*/insert_receiver(id, fns) { this.receivers[id] = fns}

    begin_scope() {
      const table =  new Table(this.current)
      const entry = new Entry('', 'table', {table} )
      this.current.insert(entry) 
      this.stack.push(table) 
      this.current =  this.stack[this.stack.length - 1] 
      return entry
    }

    end_scope() { this.current = this.stack.pop(); return this.current }
    
    insert_fn (id, arity, rtype){ 
      const entry = new Entry(id, 'fn', { arity , rtype } )
      this.current.insert(entry) 
      return entry
    }
  
    insert_type(id, fields) {
      const entry = new Entry(id, 't', {fields} )
      this.current.insert(entry) 
      return entry
    }  
  
    insert_const(id, t) {
      const entry = new Entry(id, 'const', {t} )
      this.current.insert(entry) 
      return entry
    }
  
    insert_var(id, t) {
      const entry = new Entry(id, 'var', {t} )
      this.current.insert(entry) 
      return entry
    }
  
    insert_ref(id) {
      const entry = new Entry(id, 'ref', {} )
      this.current.insert(entry) 
      return entry
    }
  
}

class Table {
  parent
  entries
  inner_scope
  constructor(parent) { 
    this.parent = parent
    this.entries = []
    this.inner_scope = []
  }  

  insert(entry) { this.entries.push(entry)}
}

class Entry {
    id
    t
    attrs
    constructor(id, t , attrs) { 
      this.id = id
      this.t = t
      this.attrs = attrs

    }
}