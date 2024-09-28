const fns = []            // FIXME: workaround, storing fn names here , until name resolution  is complete.
const structs = []          // FIXME: workaround , just like funcs

function symtab_has_fn() { return fns}
function symtab_has_struct() { return structs}

function insert_symtab_fns(fn) { fns.push(fn)}
function insert_symtab_structs(struct) { structs.push(struct) }

export { 
    symtab_has_fn, 
    symtab_has_struct, 
    insert_symtab_fns, 
    insert_symtab_structs
}