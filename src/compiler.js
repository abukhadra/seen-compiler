import Lexer from './lexer.js'
import Parser from './parser.js'
import Semantic from './semantic.js'
import Gen from './generate.js'
import { is_empty, pprint, panic }  from '../lib/sutils.js'

export default class Compiler {
    src
    main_args
    target
    target_opts
    lang
    tokens
    ast
    symtab
    gen_code
    
    init(src, main_args, lang, target_opts) {
        this.src = src
        this.main_args = main_args
        this.target = target_opts && target_opts.target || "js"
        this.target_opts = target_opts || {}        
        this.lang = lang || "en"
    }

    init_ar(src, main_args , target_opts) {
        return this.init(src, main_args, "ar", target_opts)
    }

    async get_code() {
        if(!this.gen_code) {
            await this.run()
        }
        return this.gen_code
    }

    async run() {        
        this.scan(true)        
        this.parse()
        // this.semantic()
        await this.generate(this.target)
    }

    scan(ignore_cmts_ws) {        
        const lexer = new Lexer()
        lexer.init(this.lang, this.src, ignore_cmts_ws)
        lexer.run()
        this.tokens = lexer.tokens
        if(!is_empty(lexer.errs)) {
            pprint(lexer.errs)
            panic("")
        }        
    }

    parse() {        
        const parser = new Parser()
        parser.init(this.tokens)
        parser.run()
        this.ast = parser.ast
        this.symtab = parser.symtab
        if(!is_empty(parser.errs)) {
            pprint(parser.errs)
            panic("")
        }        
    }

    semantic() { 
        const semantic = new Semantic(this.ast, this.symtab)
        semantic.run() 
        if(!is_empty(semantic.errs)) {
            pprint(semantic.errs)
            panic("")
        }        
    }

    async generate(target) {
        const gen = new Gen()
        gen.init(this.ast, this.symtab, this.main_args, target, this.target_opts)
        gen.set_lang(this.lang)
        this.gen_code = await gen.run()
    }    
}
