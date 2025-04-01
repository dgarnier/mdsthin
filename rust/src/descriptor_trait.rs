use std::convert::TryFrom;
use bytes::{Bytes, BytesMut, BufMut};
use crate::types::*;

/// Core trait for all MDSplus descriptors
pub trait MdsDescriptor: std::fmt::Debug {
    /// Get the base descriptor metadata
    fn descriptor(&self) -> &Descriptor;
    
    /// Get mutable reference to base descriptor
    fn descriptor_mut(&mut self) -> &mut Descriptor;
    
    /// Pack the descriptor into bytes for network transmission
    fn pack(&self) -> Result<Bytes>;
    
    /// Unpack bytes into this descriptor type
    fn unpack(bytes: &[u8]) -> Result<Self> where Self: Sized;
    
    /// Convert to string representation (TDI decompile)
    fn decompile(&self) -> String;
}

/// Scalar descriptor containing basic data types
#[derive(Debug, Clone)]
pub struct ScalarDescriptor<T> {
    descriptor: Descriptor,
    data: T,
}

impl<T> ScalarDescriptor<T> 
where T: Clone + std::fmt::Debug
{
    pub fn new(data: T, dtype: DataType) -> Self {
        let mut descriptor = Descriptor::new();
        descriptor.class_id = ClassType::Scalar;
        descriptor.dtype_id = dtype;
        Self {
            descriptor,
            data,
        }
    }
    
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> MdsDescriptor for ScalarDescriptor<T> 
where T: Clone + std::fmt::Debug
{
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
    
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
    
    fn pack(&self) -> Result<Bytes> {
        // Pack header
        let mut bytes = BytesMut::with_capacity(std::mem::size_of::<Descriptor>());
        bytes.put_u32_le(self.descriptor.length);
        bytes.put_u8(self.descriptor.class_id as u8);
        bytes.put_u8(self.descriptor.dtype_id as u8);
        bytes.put_u32_le(self.descriptor.offset);
        
        // Pack data - this needs to be implemented per type
        Ok(bytes.freeze())
    }
    
    fn unpack(_bytes: &[u8]) -> Result<Self> 
    where Self: Sized {
        // Unpack needs to be implemented per type
        Err(Error::Parse("Unpack not implemented".into()))
    }
    
    fn decompile(&self) -> String {
        format!("{:?}", self.data)
    }
}

/// Array descriptor containing array data
#[derive(Debug, Clone)]
pub struct ArrayDescriptor<T> {
    descriptor: Descriptor,
    data: Vec<T>,
    dims: Vec<usize>,
}

impl<T> ArrayDescriptor<T>
where T: Clone + std::fmt::Debug
{
    pub fn new(data: Vec<T>, dims: Vec<usize>, dtype: DataType) -> Self {
        let mut descriptor = Descriptor::new();
        descriptor.class_id = ClassType::Array;
        descriptor.dtype_id = dtype;
        Self {
            descriptor,
            data,
            dims,
        }
    }
    
    pub fn data(&self) -> &[T] {
        &self.data
    }
    
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }
}

impl<T> MdsDescriptor for ArrayDescriptor<T>
where T: Clone + std::fmt::Debug
{
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
    
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
    
    fn pack(&self) -> Result<Bytes> {
        let mut bytes = BytesMut::new();
        // Pack header
        bytes.put_u32_le(self.descriptor.length);
        bytes.put_u8(self.descriptor.class_id as u8);
        bytes.put_u8(self.descriptor.dtype_id as u8);
        bytes.put_u32_le(self.descriptor.offset);
        
        // Pack dimensions
        bytes.put_u8(self.dims.len() as u8);
        for &dim in &self.dims {
            bytes.put_u32_le(dim as u32);
        }
        
        // Pack data - needs implementation per type
        Ok(bytes.freeze())
    }
    
    fn unpack(_bytes: &[u8]) -> Result<Self> {
        // Unpack needs to be implemented per type
        Err(Error::Parse("Unpack not implemented".into()))
    }
    
    fn decompile(&self) -> String {
        format!("{:?}", self.data)
    }
}

/// String descriptor
#[derive(Debug, Clone)]
pub struct StringDescriptor {
    descriptor: Descriptor,
    data: String,
}

impl StringDescriptor {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let data = s.into();
        let mut descriptor = Descriptor::new();
        descriptor.class_id = ClassType::Scalar;
        descriptor.dtype_id = DataType::String;
        descriptor.length = data.len() as u32;
        Self {
            descriptor,
            data,
        }
    }
}

impl MdsDescriptor for StringDescriptor {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
    
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
    
    fn pack(&self) -> Result<Bytes> {
        let mut bytes = BytesMut::new();
        // Pack header
        bytes.put_u32_le(self.descriptor.length);
        bytes.put_u8(self.descriptor.class_id as u8);
        bytes.put_u8(self.descriptor.dtype_id as u8);
        bytes.put_u32_le(self.descriptor.offset);
        
        // Pack string data
        bytes.put_slice(self.data.as_bytes());
        Ok(bytes.freeze())
    }
    
    fn unpack(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < std::mem::size_of::<Descriptor>() {
            return Err(Error::Parse("Buffer too small".into()));
        }
        
        let descriptor = Descriptor {
            length: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            class_id: ClassType::try_from(bytes[4]).map_err(|_| Error::InvalidClass)?,
            dtype_id: DataType::try_from(bytes[5]).map_err(|_| Error::InvalidDataType)?,
            offset: u32::from_le_bytes(bytes[6..10].try_into().unwrap()),
        };
        
        let data = String::from_utf8(bytes[10..].to_vec())
            .map_err(|e| Error::Parse(e.to_string()))?;
            
        Ok(Self {
            descriptor,
            data,
        })
    }
    
    fn decompile(&self) -> String {
        format!("\"{}\"", self.data)
    }
}

// Additional descriptor implementations will be needed for:
// - Numeric types (implement ScalarDescriptor for i8,i16,i32,i64,f32,f64 etc)
// - Array types
// - Signal
// - Function
// - etc
