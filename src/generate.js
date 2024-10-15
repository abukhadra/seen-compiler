import {HtmlCssJSGen} from '../lib/sgen-html-css-js.js'
import { SUPPORTED_GEN } from './constants.js'
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
    lang

    init(ast, symtab, main_args, target, target_opts) {
        this.ast = ast
        this.symtab = symtab
        this.main_args = main_args
        this.target = target
        this.target_opts = target_opts
    }

    // FIXME: setting lang separately , to avoid breaking the code for current release ,
    //              need to refactor later.
    set_lang(lang) { this.lang = lang }

    async run() {
        let gen
        const target = to_lowercase(this.target)
        if(target === 'js') {  gen = new HtmlCssJSGen()  } 
        else if (SUPPORTED_GEN.includes(target)) { 
            const {default: Gen} = await import(this.target_opts.deps.path)
            gen = new Gen()      
        } else { panic("target \"" + this.target + "\" is not supported") }
        return gen.run(
            this.lang || en,
            this.ast, 
            this.symtab, 
            this.main_args, 
            this.target_opts
        )
    }
}