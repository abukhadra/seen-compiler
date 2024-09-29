import {JSGen} from '../lib/sgen-js.js'

import {
    to_lowercase,
    panic,    
} from '../lib/sutils.js'

export default class Gen {
    ast
    symtab
    main_args
    target
    target_opts

    init(ast, symtab, main_args, target, target_opts) {
        this.ast = ast
        this.symtab = symtab
        this.main_args = main_args
        this.target = target
        this.target_opts = target_opts
    }

    run() {
        let gen
        switch(to_lowercase(this.target)) {
            case "js": gen = new JSGen(); break
            default: panic("target \"" + this.target + "\" is not supported") ; break
        }
        gen.init(this.ast, this.symtab, this.main_args, this.target_opts)
        return gen.run()
    }
}

