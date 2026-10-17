use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

use crate::codegen_request::node::annotation::Annotation;

#[capnp_conv(schema_capnp::node::enum_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumNode {
    /// Enumerants ordered by numeric value (ordinal).
    pub enumerants: Vec<Enumerant>,
}

/// Schema for member of an enum.
#[capnp_conv(schema_capnp::enumerant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enumerant {
    pub name: String,
    /// Specifies order in which the enumerants were declared in the code.
    /// Like `Field.code_order`.
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
}
