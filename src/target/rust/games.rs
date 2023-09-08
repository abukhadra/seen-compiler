pub mod bevy;

use crate::{
    target::rust::rs_gen::Rust, 
    lang::syntax_tree::ast::{
        BlockElement,
        Attr,
    }
};

//================
//   game_2d_main()
//================
// FIXME: hardcoded for demo
impl <'a> Rust<'a> {
    pub fn game_2d_main(
        &mut self,
        els: &Vec<BlockElement>,
        attrs: &Option<Vec<Attr>>
    ) {
        if let  Some(BlockElement::AutoImpl) = els.get(0)  {
            self.bevy();
        } else {
            todo!();
        }

    }
}