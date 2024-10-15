export default class Semantic {
    ast 
    symtab 

    constructor(ast, symtab) {
        this.ast = ast 
        this.symtab = symtab
    }

    run() {
        console.log(JSON.stringify(this.symtab.root))
    }

}