import {Compiler} from '../src/main.js' 

let compiler = new Compiler()                      
compiler.init_ar(`دل بدء {اطبع_سطر(«السلام عليكم!»)}`)
let code = compiler.get_code()
eval(code); 