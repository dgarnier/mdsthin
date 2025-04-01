#[allow(non_camel_case_types)]
use crate::internals::classdef::{CLASS, class_to_string};
use crate::internals::dtypedef::{DTYPE, dtype_to_string};
use bitfield::bitfield;


// Macro to generate a struct
macro_rules! generate_dsc_struct {
    ($name:ident { $($field:ident : $type:ty,)* }) => {
        #[repr(C)]
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            pub length: u16,
            pub dtype_id: DTYPE,
            pub class_id: CLASS,
            pub offset: u32,
            $(
                pub $field: $type,
            )*
        }
    };
}

bitfield! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    pub struct aflags_t(u8);
    //impl Debug;
    pub fill, set_fill: 2, 0;       // 3 bits
    pub binscale, set_binscale: 3;  // 1 bit
    pub redim, set_redim: 4;        // 1 bit
    pub column, set_column: 5;      // 1 bit
    pub coeff, set_coeff: 6;        // 1 bit
    pub bounds, set_bounds: 7;      // 1 bit
}


// Base descriptor struct
generate_dsc_struct!( mdsdsc_t {});

generate_dsc_struct!( mdsdsc_s_t {});
generate_dsc_struct!( mdsdsc_d_t {});
generate_dsc_struct!( mdsdsc_a_t {
    scale: i8,
    digits: u8,
    aflags: aflags_t,
    dimct: u8,
    arsize: u32,
});
generate_dsc_struct!( mdsdsc_r_t {
    ndesc: u8,
    fill: [u8; 3],
});


fn main() {
    // Example usage
    let descriptor = mdsdsc_t {
        length: 10,
        dtype_id: DTYPE::BU,
        class_id: CLASS::A,
        offset: 100,
    };

    let mut aflags = aflags_t(0);
    aflags.set_binscale(true);
    aflags.set_redim(false);

    println!("Class string: {}", descriptor.class_str());
    println!("Dtype string: {}", descriptor.dtype_str());
    println!("Aflags: {:?}", aflags);
}
