use crate::{
    assembly::{Assembly, Comp, Dest},
    babel::Translation,
};

pub fn push_pointer(translator: &mut Translation, idx: i32) {
    let pointee = {
        match idx {
            0 => Assembly::this(),
            1 => Assembly::that(),
            _ => panic!("Invalid command: push pointer {idx}"),
        }
    };
    translator.with_asm([
        // @THIS/That
        pointee,
        Assembly::assign(Dest::D, Comp::M),
        // Push onto stack
        Assembly::sp(),
        Assembly::assign(Dest::A, Comp::M),
        Assembly::assign(Dest::M, Comp::D),
        // Increment SP
        Assembly::sp(),
        Assembly::assign(Dest::M, Comp::Mplus1),
    ]);
}

pub fn pop_pointer(translator: &mut Translation, idx: i32) {
    let pointee = {
        match idx {
            0 => Assembly::this(),
            1 => Assembly::that(),
            _ => panic!("Invalid command: pop pointer {idx}"),
        }
    };
    translator.decrement_sp();
    translator.store_sp_to_dreg();
    translator.with_asm([
        // Set THIS/THAT to D register
        pointee,
        Assembly::assign(Dest::M, Comp::D),
    ]);
}
