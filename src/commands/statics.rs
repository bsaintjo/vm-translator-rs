use crate::{
    assembly::{Assembly, Comp, Dest},
    babel::Translation,
};

fn var_symbol(index: u32, basename: &str) -> Assembly {
    Assembly::VariableSymbol(std::borrow::Cow::Owned(format!("{basename}.{index}")))
}

pub fn push_static(translator: &mut Translation, index: u32, basename: &str) {
    let symbol = var_symbol(index, basename);
    translator.with_asm([symbol, Assembly::assign(Dest::D, Comp::M)]);
    translator.store_dreg_to_sp();
    translator.increment_sp();
}

pub fn pop_static(translator: &mut Translation, index: u32, basename: &str) {
    let symbol = var_symbol(index, basename);
    translator.decrement_sp();
    translator.store_sp_to_dreg();
    translator.with_asm([symbol, Assembly::assign(Dest::M, Comp::D)]);
}
