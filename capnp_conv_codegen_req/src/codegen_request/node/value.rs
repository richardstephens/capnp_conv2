use capnp::schema_capnp;
use serde::{Deserialize, Serialize};

/// Represents a value, e.g. a field default value, constant value, or annotation value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Void,
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    Text(String),
    Data(Vec<u8>),
    List,
    Enum(u16),
    Struct,
    /// The only interface value that can be represented statically is "null", whose methods
    /// always throw exceptions.
    Interface,
    AnyPointer,
}

impl capnp_conv2::Readable for Value {
    type OwnedType = schema_capnp::value::Owned;

    fn read(reader: schema_capnp::value::Reader<'_>) -> capnp::Result<Self> {
        use schema_capnp::value;
        Ok(match reader.which()? {
            value::Void(()) => Value::Void,
            value::Bool(v) => Value::Bool(v),
            value::Int8(v) => Value::Int8(v),
            value::Int16(v) => Value::Int16(v),
            value::Int32(v) => Value::Int32(v),
            value::Int64(v) => Value::Int64(v),
            value::Uint8(v) => Value::Uint8(v),
            value::Uint16(v) => Value::Uint16(v),
            value::Uint32(v) => Value::Uint32(v),
            value::Uint64(v) => Value::Uint64(v),
            value::Float32(v) => Value::Float32(v),
            value::Float64(v) => Value::Float64(v),
            value::Text(t) => Value::Text(t?.to_string()?),
            value::Data(d) => Value::Data(d?.to_vec()),
            value::List(_) => Value::List,
            value::Enum(v) => Value::Enum(v),
            value::Struct(_) => Value::Struct,
            value::Interface(()) => Value::Interface,
            value::AnyPointer(_) => Value::AnyPointer,
        })
    }
}

impl capnp_conv2::Writable for Value {
    type OwnedType = schema_capnp::value::Owned;

    fn write(&self, mut builder: schema_capnp::value::Builder<'_>) {
        match self {
            Value::Void => builder.set_void(()),
            Value::Bool(v) => builder.set_bool(*v),
            Value::Int8(v) => builder.set_int8(*v),
            Value::Int16(v) => builder.set_int16(*v),
            Value::Int32(v) => builder.set_int32(*v),
            Value::Int64(v) => builder.set_int64(*v),
            Value::Uint8(v) => builder.set_uint8(*v),
            Value::Uint16(v) => builder.set_uint16(*v),
            Value::Uint32(v) => builder.set_uint32(*v),
            Value::Uint64(v) => builder.set_uint64(*v),
            Value::Float32(v) => builder.set_float32(*v),
            Value::Float64(v) => builder.set_float64(*v),
            Value::Text(v) => builder.set_text(v.as_str()),
            Value::Data(v) => builder.set_data(v),
            Value::List => {}
            Value::Enum(v) => builder.set_enum(*v),
            Value::Struct => {}
            Value::Interface => builder.set_interface(()),
            Value::AnyPointer => {}
        }
    }
}
