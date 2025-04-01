use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Class identifiers
#[allow(non_camel_case_types)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CLASS {
    /// Represents an invalid class
    MISSING = 0,
    /// Static fixed-length descriptor
    S = 1,
    /// Dynamic string descriptor
    D = 2,
    /// Deprecated: VMS variable buffer descriptor
    V = 3,
    /// Array descriptor
    A = 4,
    /// Deprecated: Procedure descriptor
    P = 5,
    /// Deprecated: Procedure incarnation descriptor
    PI = 6,
    /// Deprecated: VMS debugger label descriptor
    J = 7,
    /// Deprecated: VMS debugger label incarnation descriptor
    JI = 8,
    /// Deprecated: Decimal string descriptor
    SD = 9,
    /// Deprecated: Noncontiguous array descriptor
    NCA = 10,
    /// Deprecated: Varying string descriptor
    VS = 11,
    /// Deprecated: Varying string array descriptor
    VSA = 12,
    /// Deprecated: Unaligned bit string descriptor
    UBS = 13,
    /// Deprecated: Unaligned bit array descriptor
    UBA = 14,
    /// Deprecated: String with bounds descriptor
    SB = 15,
    /// Deprecated: Unaligned bit string with bounds descriptor
    UBSB = 16,
    /// Extended dynamic descriptor
    XD = 192,
    /// Extended static descriptor
    XS = 193,
    /// Record descriptor
    R = 194,
    /// Compressed array descriptor
    CA = 195,
    /// Array of pointers to data descriptor
    APD = 196,
}

const ARRAY_CLASSES: [CLASS; 3] = [CLASS::A, CLASS::CA, CLASS::APD];

pub fn class_is_array(class: CLASS) -> bool {
    ARRAY_CLASSES.contains(&class)
}

pub fn class_to_string(class: CLASS) -> String {
    match class {
        CLASS::MISSING => "CLASS_MISSING",
        CLASS::S => "CLASS_S",
        CLASS::D => "CLASS_D",
        CLASS::V => "CLASS_V",
        CLASS::A => "CLASS_A",
        CLASS::P => "CLASS_P",
        CLASS::PI => "CLASS_PI",
        CLASS::J => "CLASS_J",
        CLASS::JI => "CLASS_JI",
        CLASS::SD => "CLASS_SD",
        CLASS::NCA => "CLASS_NCA",
        CLASS::VS => "CLASS_VS",
        CLASS::VSA => "CLASS_VSA",
        CLASS::UBS => "CLASS_UBS",
        CLASS::UBA => "CLASS_UBA",
        CLASS::SB => "CLASS_SB",
        CLASS::UBSB => "CLASS_UBSB",
        CLASS::XD => "CLASS_XD",
        CLASS::XS => "CLASS_XS",
        CLASS::R => "CLASS_R",
        CLASS::CA => "CLASS_CA",
        CLASS::APD => "CLASS_APD",
        _ => "CLASS_MISSING",
    }.to_string()
}

fn main() {
    // Example usage
    let class = CLASS::A;
    println!("Is array: {}", class_is_array(class));
    println!("Class string: {}", class_to_string(class));
}