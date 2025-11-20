use crate::{
    assembly::{Assembly, Comp, Dest},
    babel::Translation,
};

pub fn push_temp(translator: &mut Translation, index: u32) {
    let temp_loc = 5 + index;
    translator.with_asm([
        // Store TEMP[i] into D
        Assembly::Address(temp_loc),
        Assembly::assign(Dest::D, Comp::M),
    ]);
    translator.store_dreg_to_sp();
    translator.increment_sp();
}

pub fn pop_temp(translator: &mut Translation, index: u32) {
    let temp_loc = 5 + index;
    translator.decrement_sp();
    translator.store_sp_to_dreg();
    translator.with_asm([
        Assembly::Address(temp_loc),
        Assembly::assign(Dest::M, Comp::D),
    ]);
}
