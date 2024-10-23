import {Compiler} from '../src/main.js' 

let compiler = new Compiler()
compiler.init(`fn main { println('hello world!')}`)
compiler.get_code().then(
    code => eval(code)
)