#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassType {
    Missing = 0,
    Scalar = 1,
    Array = 2,
    Pointer = 3,
    Recursive = 4,
    CompoundArray = 5,
    ArrayPointer = 6,
    BoundedArray = 7,
    WithUnits = 8,
    WithError = 9,
    List = 10,
    Dictionary = 11,
    Tuple = 12,
    Opaque = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    Missing = 0,
    Byte = 1,
    Word = 2,
    Long = 3,
    Float = 4,
    Double = 5,
    Complex = 6,
    String = 7,
    Ident = 8,
    Nid = 9,
    Path = 10,
    Signal = 11,
    Dimension = 12,
    Window = 13,
    Slope = 14,
    Function = 15,
    Conglom = 16,
    Range = 17,
    Action = 18,
    Dispatch = 19,
    Program = 20,
    Routine = 21,
    Procedure = 22,
    Method = 23,
    Dependency = 24,
    Condition = 25,
    Event = 26,
    WithUnits = 27,
    Call = 28,
    WithError = 29,
    Parameter = 30,
    UByte = 31,
    UWord = 32,
    ULong = 33,
    Int64 = 34,
    UInt64 = 35,
    FSingle = 36,
    GComplex = 37,
    FTDouble = 38,
    FSComplex = 39,
    FTComplex = 40,
    Opaque = 41,
}

/// Maximum number of dimensions supported in array descriptors
pub const MAX_DIMS: usize = 8;

/// Status code helpers
pub fn status_ok(status: i32) -> bool {
    status & 1 != 0
}

pub fn status_not_ok(status: i32) -> bool {
    !status_ok(status)
}

pub fn status_facility(status: i32) -> i32 {
    status >> 16
}

pub fn status_message(status: i32) -> i32 {
    (status >> 3) & 0b1111111111111
}

pub fn status_severity(status: i32) -> i32 {
    status & 0b111
}

#[derive(Debug, Clone)]
pub struct Descriptor {
    /// Length in bytes of data
    pub length: u32,
    /// Class type (scalar, array, etc)
    pub class_id: ClassType,
    /// Data type 
    pub dtype_id: DataType,
    /// Byte offset to data
    pub offset: u32,
}

impl Descriptor {
    pub fn new() -> Self {
        Self {
            length: 0,
            class_id: ClassType::Missing,
            dtype_id: DataType::Missing,
            offset: 0,
        }
    }
}

/// Errors that can occur in MDSThin operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid descriptor class")]
    InvalidClass,
    
    #[error("Invalid data type")]
    InvalidDataType,
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("MDSplus error: {0}")]
    MdsError(String),
    
    #[error("Parse error: {0}")] 
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// MDSplus network message header
#[repr(C, packed(4))]
pub struct MessageHeader {
    pub msglen: i32,
    pub status: i32,
    pub length: i16,
    pub nargs: u8,
    pub descriptor_idx: u8,
    pub message_id: u8,
    pub dtype: DataType,
    pub client_type: i8,
    pub ndims: u8,
    pub dims: [i32; MAX_DIMS],
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            msglen: 0,
            status: 0,
            length: 0,
            nargs: 0,
            descriptor_idx: 0,
            message_id: 0,
            dtype: DataType::Missing,
            client_type: 0,
            ndims: 0,
            dims: [0; MAX_DIMS],
        }
    }
}

/// Network client type constants
pub const INVALID_CLIENT: i8 = 0;
pub const IEEE_CLIENT: i8 = 2;

/// Message compression flags
pub const COMPRESSED: i8 = 0x20;
pub const SENDCAPABILITIES: i8 = 0xF;
