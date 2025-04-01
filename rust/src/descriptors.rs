use crate::internals::classdef::CLASS;
use crate::internals::dtypedef::{DTYPE, dtype_to_string};
use crate::internals::mdsdescrip::*;
use serde::{Serialize, Deserialize};
use std::fmt;
use ndarray::prelude::*;
use phf::phf_map;

// Descriptor types are 
pub enum Descriptor {
    String(StringDescriptor),
    Ident(IdentDescriptor),
    TreeNID(TreeNIDDescriptor),
    TreePath(TreePathDescriptor),
    UInt8(UInt8Descriptor),
    UInt16(UInt16Descriptor),
    UInt32(UInt32Descriptor),
    UInt64(UInt64Descriptor),
    Int8(Int8Descriptor),
    Int16(Int16Descriptor),
    Int32(Int32Descriptor),
    Int64(Int64Descriptor),
    Float32(Float32Descriptor),
    Float64(Float64Descriptor),
    StringArray(StringArrayDescriptor),
    UInt8Array(UInt8ArrayDescriptor),
    UInt16Array(UInt16ArrayDescriptor),
    UInt32Array(UInt32ArrayDescriptor),
    UInt64Array(UInt64ArrayDescriptor),
    Int8Array(Int8ArrayDescriptor),
    Int16Array(Int16ArrayDescriptor),
    Int32Array(Int32ArrayDescriptor),
    Int64Array(Int64ArrayDescriptor),
    Float32Array(Float32ArrayDescriptor),
    Float64Array(Float64ArrayDescriptor),
    List(ListDescriptor),
    Tuple(TupleDescriptor),
    Dictionary(DictionaryDescriptor),
    Signal(SignalDescriptor),
    Dimension(DimensionDescriptor),
    Window(WindowDescriptor),
    Slope(SlopeDescriptor),
    Function(FunctionDescriptor),
    Conglom(ConglomDescriptor),
    Range(RangeDescriptor),
    Action(ActionDescriptor),
    Dispatch(DispatchDescriptor),
    Program(ProgramDescriptor),
    Routine(RoutineDescriptor),
    Procedure(ProcedureDescriptor),
    Method(MethodDescriptor),
    Dependency(DependencyDescriptor),
    Condition(ConditionDescriptor),
    WithUnits(WithUnitsDescriptor),
    Call(CallDescriptor),
    WithError(WithErrorDescriptor),
    Opaque(OpaqueDescriptor),
}

struct StringDescriptor {
    dsc: mdsdsc_s_t,
    data: String,
}

    pub value: String,
}
pub trait Descriptor {
    fn dtype_id(&self) -> DTYPE {
        self.dsc.dtype_id
    };
    fn class_id(&self) -> CLASS {
        self.dsc.class_id
    };
    fn length(&self) -> u16 {
        self.dsc.length
    };
    fn offset(&self) -> u32 {
        self.dsc.offset
    };
    fn dtype_str(&self) -> String {
        dtype_to_string(self.dsc.dtype_id)
    };
    fn class_str(&self) -> String {
        format!("{:?}", self.dsc.class_id)
    };
    fn pack(&self) -> Vec<u8>;
    // create from a raw buffer, returning a descriptor on the heap
    fn unpack(buffer: &[u8]) -> Option<Box<impl Descriptor>> {
        let dtype_id = DTYPE::try_from(buffer[2]).unwrap_or(DTYPE::MISSING);
        let class_id = CLASS::try_from(buffer[3]).unwrap_or(CLASS::MISSING);
        if class_id in 
        descriptor_from_buffer(buffer, dtype_id, class_id)
    }
    // create a new descriptor with the given data
    fn new<T>(data: T) -> Option<Box<impl Descriptor>> {

        let mut descriptor = Self::default();
        descriptor.dsc.length = 0;
        descriptor.dsc.dtype_id = DTYPE::MISSING;
        descriptor.dsc.class_id = CLASS::MISSING;
        descriptor.dsc.offset = 0;
        Some(Box::new(descriptor))
    }
}

fn dtype_to_num(dtype: DTYPE) -> dyn Num {
    match dtype {
        DTYPE::BU => u8,
        DTYPE::WU => u16,
        DTYPE::LU => u32,
        DTYPE::QU => u64,
        DTYPE::B => i8,
        DTYPE::W => i16,
        DTYPE::L => i32,
        DTYPE::Q => i64,
        DTYPE::FS => f32,
        DTYPE::FT => f64,
        DTYPE::NID => u32,
    }
};


pub trait DescriptorS: Descriptor {
    fn data(&self) -> Option<&[u8]>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BasicDescriptor {
    pub dsc: mdsdsc_t,
    pub data: Option<Vec<u8>>,
}

impl BasicDescriptor {
    pub fn dtype_str(&self) -> String {
        dtype_to_string(self.dsc.dtype_id)
    }

    pub fn class_str(&self) -> String {
        format!("{:?}", self.dsc.class_id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumericDescriptor {
    pub base: BasicDescriptor,
}

impl Numeric for NumericDescriptor {
    fn as_int(&self) -> i64 {
        match &self.base.data {
            Some(data) => i64::from_le_bytes(data[..8].try_into().unwrap_or_default()),
            None => 0,
        }
    }

    fn as_float(&self) -> f64 {
        match &self.base.data {
            Some(data) => f64::from_le_bytes(data[..8].try_into().unwrap_or_default()),
            None => 0.0,
        }
    }
}

impl NumericDescriptor {
    pub fn dtype_str(&self) -> String {
        self.base.dtype_str()
    }

    pub fn class_str(&self) -> String {
        self.base.class_str()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    pub value: Option<Box<Descriptor>>,
    pub raw: Option<Box<Descriptor>>,
    pub dimensions: Vec<Box<Descriptor>>,
}

impl Signal {
    pub fn new(value: Option<Box<Descriptor>>, raw: Option<Box<Descriptor>>, dimensions: Vec<Box<Descriptor>>) -> Self {
        Self {
            value,
            raw,
            dimensions,
        }
    }

    pub fn data(&self) -> Option<&Box<Descriptor>> {
        self.value.as_ref().or(self.raw.as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub opcode: u16,
    pub arguments: Vec<Box<Descriptor>>,
}

impl Function {
    pub fn new(opcode: u16, arguments: Vec<Box<Descriptor>>) -> Self {
        Self { opcode, arguments }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringDescriptor {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Int32Descriptor {
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Float64Descriptor {
    pub value: f64,
}

fn main() {
    let basic = Descriptor::Basic(BasicDescriptor {
        dsc: mdsdsc_t {
            length: 4,
            dtype_id: DTYPE::BU,
            class_id: CLASS::S,
            offset: 0,
        },
        data: None,
    });

    let numeric = Descriptor::Numeric(NumericDescriptor {
        base: BasicDescriptor {
            dsc: mdsdsc_t {
                length: 8,
                dtype_id: DTYPE::Q,
                class_id: CLASS::S,
                offset: 0,
            },
            data: Some(vec![1, 0, 0, 0, 0, 0, 0, 0]),
        },
    });

    if let Descriptor::Numeric(numeric_desc) = &numeric {
        println!("As int: {}", numeric_desc.as_int());
        println!("As float: {}", numeric_desc.as_float());
    }

    let signal = Descriptor::Signal(Signal::new(None, None, vec![]));

    let function = Descriptor::Function(Function::new(38, vec![]));

    println!("{:?}", basic);
    println!("{:?}", numeric);
    println!("{:?}", signal);
    println!("{:?}", function);
}
