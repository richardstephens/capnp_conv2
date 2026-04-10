use crate::codegen_request::node::field::Field;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::node::struct_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructNode {
    /// Size of the data section, in words.
    pub data_word_count: u16,
    /// Size of the pointer section, in pointers (which are one word each).
    pub pointer_count: u16,
    /// The preferred element size to use when encoding a list of this struct. If this is anything
    /// other than `InlineComposite` then the struct is one word or less in size and is a candidate
    /// for list packing optimization.
    #[capnp_conv(type = "enum_remote")]
    pub preferred_list_encoding: ElementSize,
    /// If true, then this "struct" node is actually not an independent node, but merely represents
    /// some named union or group within a particular parent struct. This node's `scope_id` refers
    /// to the parent struct, which may itself be a union/group in yet another struct.
    ///
    /// All group nodes share the same `data_word_count` and `pointer_count` as the top-level
    /// struct, and their fields live in the same ordinal and offset spaces as all other fields in
    /// the struct.
    ///
    /// Note that a named union is considered a special kind of group -- in fact, a named union
    /// is exactly equivalent to a group that contains nothing but an unnamed union.
    pub is_group: bool,
    /// Number of fields in this struct which are members of an anonymous union, and thus may
    /// overlap. If this is non-zero, then a 16-bit discriminant is present indicating which
    /// of the overlapping fields is active. This can never be 1 -- if it is non-zero, it must be
    /// two or more.
    ///
    /// Note that the fields of an unnamed union are considered fields of the scope containing the
    /// union -- an unnamed union is not its own group. So, a top-level struct may contain a
    /// non-zero discriminant count. Named unions, on the other hand, are equivalent to groups
    /// containing unnamed unions. So, a named union has its own independent schema node, with
    /// `is_group` = true.
    pub discriminant_count: u16,
    /// If `discriminant_count` is non-zero, this is the offset of the union discriminant, in
    /// multiples of 16 bits.
    pub discriminant_offset: u32,
    /// Fields defined within this scope (either the struct's top-level fields, or the fields of
    /// a particular group; see `is_group`).
    ///
    /// The fields are sorted by ordinal number, but note that because groups share the same
    /// ordinal space, the field's index in this list is not necessarily exactly its ordinal.
    /// On the other hand, the field's position in this list does remain the same even as the
    /// protocol evolves, since it is not possible to insert or remove an earlier ordinal.
    /// Therefore, for most use cases, if you want to identify a field by number, it may make the
    /// most sense to use the field's index in this list rather than its ordinal.
    pub fields: Vec<Field>,
}

/// Possible element sizes for encoded lists. These correspond exactly to the possible values of
/// the 3-bit element size component of a list pointer.
#[capnp_conv(schema_capnp::ElementSize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementSize {
    /// aka "void", but that's a keyword.
    Empty,
    Bit,
    Byte,
    TwoBytes,
    FourBytes,
    EightBytes,
    Pointer,
    InlineComposite,
}
