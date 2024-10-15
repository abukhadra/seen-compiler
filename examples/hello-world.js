import {Compiler} from '../src/main.js' 

let compiler = new Compiler()
compiler.init(`fn main { println('hello world!')}`)
let code = compiler.get_code()
eval(code); 