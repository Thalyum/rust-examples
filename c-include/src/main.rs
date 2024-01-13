use cty;

/* File: cool_bindings.rs */
#[repr(C)]
pub struct CoolStruct {
    pub x: cty::c_int,
    pub y: cty::c_int,
}

extern "C" {
    pub fn cool_function(
        i: cty::c_int,
        c: cty::c_char,
        cs: *mut CoolStruct
    );
}

fn main() {
    let mut c_struct = CoolStruct {
        x: 45,
        y: 27,
    };
    unsafe { cool_function(6, 'g' as i8, &mut c_struct) };
}
