//================
//  eprintln_red
//================
pub fn eprintln_red(v: &str) {
    eprintln!("\x1b[31m{}\x1b[0m", v);
}