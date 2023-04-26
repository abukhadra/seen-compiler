pub mod actix_files;
pub mod actix_web;

pub trait Crate {
    fn id(&self) -> &String;
    fn version(&self) -> &String;
}