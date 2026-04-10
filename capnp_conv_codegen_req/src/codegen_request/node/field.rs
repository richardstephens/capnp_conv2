use crate::codegen_request::node::annotation::Annotation;
use crate::codegen_request::node::types::Type;
use crate::codegen_request::node::value::Value;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

/// Schema for a field of a struct.
#[capnp_conv(schema_capnp::field)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    /// Indicates where this member appeared in the code, relative to other members.
    /// Code ordering may have semantic relevance -- programmers tend to place related fields
    /// together. So, using code ordering makes sense in human-readable formats where ordering is
    /// otherwise irrelevant, like JSON. The values of `code_order` are tightly-packed, so the
    /// maximum value is `count(members) - 1`. Fields that are members of a union are only ordered
    /// relative to the other members of that union, so the maximum value there is
    /// `count(union.members)`.
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
    /// If the field is in a union, this is the value which the union's discriminant should take
    /// when the field is active. If the field is not in a union, this is 0xffff.
    pub discriminant_value: u16,
    #[capnp_conv(type = "unnamed_union")]
    pub kind: FieldKind,
    #[capnp_conv(type = "union")]
    pub ordinal: Ordinal,
}

#[capnp_conv(schema_capnp::field)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldKind {
    /// A regular, non-group, non-fixed-list field.
    #[capnp_conv(type = "group")]
    Slot(SlotField),
    /// A group.
    #[capnp_conv(type = "group")]
    Group(GroupField),
}

#[capnp_conv(schema_capnp::field::slot)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotField {
    /// Offset, in units of the field's size, from the beginning of the section in which the field
    /// resides. E.g. for a UInt32 field, multiply this by 4 to get the byte offset from the
    /// beginning of the data section.
    pub offset: u32,
    #[capnp_conv(name = "type")]
    pub type_: Type,
    pub default_value: Value,
    /// Whether the default value was specified explicitly. Non-explicit default values are always
    /// zero or empty values. Usually, whether the default value was explicit shouldn't matter.
    /// The main use case for this flag is for structs representing method parameters:
    /// explicitly-defaulted parameters may be allowed to be omitted when calling the method.
    pub had_explicit_default: bool,
}

#[capnp_conv(schema_capnp::field::group)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupField {
    /// The ID of the group's node.
    pub type_id: u64,
}

#[capnp_conv(schema_capnp::field::ordinal)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ordinal {
    Implicit(()),
    /// The original ordinal number given to the field. You probably should NOT use this; if you
    /// need a numeric identifier for a field, use its position within the field array for its
    /// scope. The ordinal is given here mainly just so that the original schema text can be
    /// reproduced given the compiled version -- i.e. so that `capnp compile -ocapnp` can do its
    /// job.
    Explicit(u16),
}
