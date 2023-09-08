//================
//  replace_ext()
//================
pub fn replace_ext( 
    path: &mut String, 
    ext: &str,
    new_ext: String
) -> String{
    let pos = path.rfind(ext).unwrap();  // FIXME unwrap()
    path.drain(pos..);
    format!("{path}{new_ext}")  
}