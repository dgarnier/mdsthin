use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[derive(Debug, TryFromPrimitive, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DTYPE {
    MISSING = 0, // No data
    V = 1, // (deprecated) Aligned bit string
    BU = 2, // Byte Unsigned, 8-bit unsigned integer
    WU = 3, // Word Unsigned, 16-bit unsigned integer
    LU = 4, // Long Unsigned, 32-bit integer
    QU = 5, // Quadword Unsigned, 64-bit integer
    B = 6, // Byte, 8-bit signed integer
    W = 7, // Word, 16-bit signed integer
    L = 8, // Long, 32-bit signed integer
    Q = 9, // Quadword, 64-bit signed integer
    F = 10, // (deprecated) VMS F_Floating 32-bit single-precision floating point
    D = 11, // (deprecated) VMS D_Floating 64-bit single-precision floating point
    FC = 12, // (deprecated) DTYPE_F Complex
    DC = 13, // (deprecated) DTYPE_D Complex
    T = 14, // Text, ASCII 8-bit character string
    DSC = 24, // Descriptor
    OU = 25, // Octaword Unsigned, 128-bit unsigned integer
    O = 26, // Unsigned Octaword, 128-bit signed integer
    G = 27, // (deprecated) VMS G_floating 64-bit double-precision floating point
    H = 28, // (deprecated) VMS H_Floating 128-bit quadruple-precision floating point
    GC = 29, // (deprecated) DTYPE_G complex
    HC = 30, // (deprecated) DTYPE_H complex
    POINTER = 51, // TODO:
    FS = 52, // IEEE 32-bit single-precision floating point
    FT = 53, // IEEE 64-bit double-precision floating point
    FSC = 54, // DTYPE_FS complex
    FTC = 55, // DTYPE_FT complex
    C = 56, // Used for char* returned by TdiCall() that must be freed
    IDENT = 191, // TODO:
    NID = 192, // Node identification number, see NID
    PATH = 193, // Path to node as a string
    PARAM = 194, // TODO:
    SIGNAL = 195, // Signal()
    DIMENSION = 196, // Dimension()
    WINDOW = 197, // Window()
    SLOPE = 198, // Slope()
    FUNCTION = 199, // TODO:
    CONGLOM = 200, // TODO:
    RANGE = 201, // Range()
    ACTION = 202, // Action()
    DISPATCH = 203, // Dispatch()
    PROGRAM = 204, // (deprecated) TODO:
    ROUTINE = 205, // (deprecated) TODO:
    PROCEDURE = 206, // (deprecated) TODO:
    METHOD = 207, // Method()
    DEPENDENCY = 208, // TODO:
    CONDITION = 209, // TODO:
    EVENT = 210, // TODO:
    WITH_UNITS = 211, // WithUnits()
    CALL = 212, // TODO:
    WITH_ERROR = 213, // WithError()
    LIST = 214, // List()
    TUPLE = 215, // TODO:
    DICTIONARY = 216, // TODO:
    OPAQUE = 217, // TODO:
}


const NUMERIC_DTYPES: [DTYPE; 23] = [
    DTYPE::BU, DTYPE::WU, DTYPE::LU, DTYPE::QU,
    DTYPE::B, DTYPE::W, DTYPE::L, DTYPE::Q,
    DTYPE::F, DTYPE::D, DTYPE::FC, DTYPE::DC,
    DTYPE::OU, DTYPE::O,
    DTYPE::G, DTYPE::H, DTYPE::GC, DTYPE::HC,
    DTYPE::FS, DTYPE::FT, DTYPE::FSC, DTYPE::FTC,
    DTYPE::DSC,
];

pub fn dtype_is_numeric(dtype: DTYPE) -> bool {
    NUMERIC_DTYPES.contains(&dtype)
}

const EXPRESSION_DTYPES: [DTYPE; 5] = [
    DTYPE::FUNCTION,
    DTYPE::NID,
    DTYPE::PATH,
    DTYPE::IDENT,
    DTYPE::CALL,
];

pub fn dtype_is_expression(dtype: DTYPE) -> bool {
    EXPRESSION_DTYPES.contains(&dtype)
}



pub fn dtype_to_string(dtype: DTYPE) -> String {
    match dtype {
        DTYPE::MISSING => "DTYPE_MISSING",
        DTYPE::V => "DTYPE_V",
        DTYPE::BU => "DTYPE_BU",
        DTYPE::WU => "DTYPE_WU",
        DTYPE::LU => "DTYPE_LU",
        DTYPE::QU => "DTYPE_QU",
        DTYPE::B => "DTYPE_B",
        DTYPE::W => "DTYPE_W",
        DTYPE::L => "DTYPE_L",
        DTYPE::Q => "DTYPE_Q",
        DTYPE::F => "DTYPE_F",
        DTYPE::D => "DTYPE_D",
        DTYPE::FC => "DTYPE_FC",
        DTYPE::DC => "DTYPE_DC",
        DTYPE::T => "DTYPE_T",
        DTYPE::DSC => "DTYPE_DSC",
        DTYPE::OU => "DTYPE_OU",
        DTYPE::O => "DTYPE_O",
        DTYPE::G => "DTYPE_G",
        DTYPE::H => "DTYPE_H",
        DTYPE::GC => "DTYPE_GC",
        DTYPE::HC => "DTYPE_HC",
        DTYPE::POINTER => "DTYPE_POINTER",
        DTYPE::FS => "DTYPE_FS",
        DTYPE::FT => "DTYPE_FT",
        DTYPE::FSC => "DTYPE_FSC",
        DTYPE::FTC => "DTYPE_FTC",
        DTYPE::C => "DTYPE_C",
        DTYPE::IDENT => "DTYPE_IDENT",
        DTYPE::NID => "DTYPE_NID",
        DTYPE::PATH => "DTYPE_PATH",
        DTYPE::PARAM => "DTYPE_PARAM",
        DTYPE::SIGNAL => "DTYPE_SIGNAL",
        DTYPE::DIMENSION => "DTYPE_DIMENSION",
        DTYPE::WINDOW => "DTYPE_WINDOW",
        DTYPE::SLOPE => "DTYPE_SLOPE",
        DTYPE::FUNCTION => "DTYPE_FUNCTION",
        DTYPE::CONGLOM => "DTYPE_CONGLOM",
        DTYPE::RANGE => "DTYPE_RANGE",
        DTYPE::ACTION => "DTYPE_ACTION",
        DTYPE::DISPATCH => "DTYPE_DISPATCH",
        DTYPE::PROGRAM => "DTYPE_PROGRAM",
        DTYPE::ROUTINE => "DTYPE_ROUTINE",
        DTYPE::PROCEDURE => "DTYPE_PROCEDURE",
        DTYPE::METHOD => "DTYPE_METHOD",
        DTYPE::DEPENDENCY => "DTYPE_DEPENDENCY",
        DTYPE::CONDITION => "DTYPE_CONDITION",
        DTYPE::EVENT => "DTYPE_EVENT",
        DTYPE::WITH_UNITS => "DTYPE_WITH_UNITS",
        DTYPE::CALL => "DTYPE_CALL",
        DTYPE::WITH_ERROR => "DTYPE_WITH_ERROR",
        DTYPE::LIST => "DTYPE_LIST",
        DTYPE::TUPLE => "DTYPE_TUPLE",
        DTYPE::DICTIONARY => "DTYPE_DICTIONARY",
        DTYPE::OPAQUE => "DTYPE_OPAQUE",
        _ => "DTYPE_UNKNOWN",
    }
    .to_string()
}

pub fn get_dtype_size(dtype: DTYPE) -> usize {
    match dtype {
        DTYPE::BU | DTYPE::B => 1,
        DTYPE::WU | DTYPE::W => 2,
        DTYPE::LU | DTYPE::L | DTYPE::F | DTYPE::FS => 4,
        DTYPE::QU | DTYPE::Q | DTYPE::D | DTYPE::G | DTYPE::FT => 8,
        DTYPE::OU | DTYPE::O => 16,
        _ => 0,
    }
}

fn main() {
    // Example usage
    let dtype = DTYPE::BU;
    println!("Is numeric: {}", dtype_is_numeric(dtype));
    println!("Is expression: {}", dtype_is_expression(dtype));
    println!("Dtype string: {}", dtype_to_string(dtype));
    println!("Dtype size: {}", get_dtype_size(dtype));
}