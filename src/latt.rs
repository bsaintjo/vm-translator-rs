use crate::{
    assembly::{Assembly, Comp, Dest},
    babel::Translation,
    segment::LATT,
};

pub fn push_latt(translator: &mut Translation, latt: LATT, index: u32) {
    let base = latt.as_asm();
    translator.with_asm([
        // store base (RAM[segment addr]) + index into dreg
        base,
        Assembly::assign(Dest::D, Comp::M),
        Assembly::Address(index),
        Assembly::assign(Dest::A, Comp::DplusA),
        Assembly::assign(Dest::D, Comp::M),
    ]);
    translator.store_dreg_to_sp();
    translator.increment_sp();
}

pub fn pop_latt(translator: &mut Translation, latt: LATT, index: u32) {
    let base = latt.as_asm();
    // Store Address of base + i into dreg
    translator.with_asm([
        base,
        Assembly::assign(Dest::D, Comp::M),
        Assembly::Address(index),
        Assembly::assign(Dest::D, Comp::DplusA),
    ]);
    translator.store_dreg_in_reg13();
    translator.decrement_sp();
    translator.store_sp_to_dreg();
    translator.with_asm([
        Assembly::reg13(),
        Assembly::assign(Dest::A, Comp::M),
        Assembly::assign(Dest::M, Comp::D),
    ]);
}
