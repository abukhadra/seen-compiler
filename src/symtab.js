// FIXME: workaround, storing fn names here , until name resolution  is complete.

class Symtab {
    fns
    structs
    constructor() {
        this.fns = []
        this.structs = []
    }

    insert_fn(fn) { this.fns.push(fn)}
    insert_struct(struct) { this.structs.push(struct) }

}

export { 
    Symtab
}