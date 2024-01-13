#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let mut c_struct = CoolStruct { x: 45, y: 27 };
    let a = unsafe {
        cool_function(6, 'g' as i8, &mut c_struct);
        sum(8, 10)
    };
    println!("sum = {a}");
}
