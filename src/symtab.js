const fns = []            // FIXME: workaround, storing fn names here , until name resolution  is complete.
const structs = []          // FIXME: workaround , just like funcs

function get_symtab_fns() { return fns}
function get_symtab_structs() { return structs}

function insert_symtab_fns(fn) { fns.push(fn)}
function insert_symtab_structs(struct) { structs.push(struct) }

export { 
    get_symtab_fns, 
    get_symtab_structs, 
    insert_symtab_fns, 
    insert_symtab_structs
}