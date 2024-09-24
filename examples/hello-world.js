import Project from '../scompiler.js' 

let project = new Project()
project.init(`fn main { println('hello world!')}`)
let code = project.get_code()
eval(code); 