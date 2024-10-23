import {Compiler} from '../src/main.js' 

let compiler = new Compiler()                      
compiler.init_ar(`دل بدء {اطبع_سطر(«السلام عليكم!»)}`)
compiler.get_code().then(
    code => eval(code)
)