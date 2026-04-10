use crate::codegen_request::node::brand::Brand;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

/// Represents a type expression.
#[capnp_conv(schema_capnp::type_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Void(()),
    Bool(()),
    Int8(()),
    Int16(()),
    Int32(()),
    Int64(()),
    Uint8(()),
    Uint16(()),
    Uint32(()),
    Uint64(()),
    Float32(()),
    Float64(()),
    Text(()),
    Data(()),
    #[capnp_conv(type = "group")]
    List(TypeList),
    #[capnp_conv(type = "group")]
    Enum(TypeEnum),
    #[capnp_conv(type = "group")]
    Struct(TypeStruct),
    #[capnp_conv(type = "group")]
    Interface(TypeInterface),
    #[capnp_conv(type = "group")]
    AnyPointer(TypeAnyPointer),
}

#[capnp_conv(schema_capnp::type_::list)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeList {
    pub element_type: Box<Type>,
}

#[capnp_conv(schema_capnp::type_::enum_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeEnum {
    pub type_id: u64,
    pub brand: Brand,
}

#[capnp_conv(schema_capnp::type_::struct_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStruct {
    pub type_id: u64,
    pub brand: Brand,
}

#[capnp_conv(schema_capnp::type_::interface)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInterface {
    pub type_id: u64,
    pub brand: Brand,
}

#[capnp_conv(schema_capnp::type_::any_pointer)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeAnyPointer {
    /// A regular AnyPointer.
    ///
    /// The name "unconstrained" means as opposed to constraining it to match a type parameter.
    /// In retrospect this name is probably a poor choice given that it may still be constrained
    /// to be a struct, list, or capability.
    #[capnp_conv(type = "group")]
    Unconstrained(TypeAnyPointerUnconstrained),
    /// This is actually a reference to a type parameter defined within this scope.
    #[capnp_conv(type = "group")]
    Parameter(TypeAnyPointerParameter),
    /// This is actually a reference to an implicit (generic) parameter of a method. The only
    /// legal context for this type to appear is inside `Method.param_brand` or
    /// `Method.result_brand`.
    #[capnp_conv(type = "group")]
    ImplicitMethodParameter(TypeAnyPointerImplicitMethodParameter),
}

#[capnp_conv(schema_capnp::type_::any_pointer::unconstrained)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeAnyPointerUnconstrained {
    /// Truly AnyPointer.
    AnyKind(()),
    /// AnyStruct.
    Struct(()),
    /// AnyList.
    List(()),
    /// Capability.
    Capability(()),
}

#[capnp_conv(schema_capnp::type_::any_pointer::parameter)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAnyPointerParameter {
    /// ID of the generic type whose parameter we're referencing. This is always either the
    /// current scope's type ID or one of its ancestors' IDs.
    pub scope_id: u64,
    /// Index of the parameter within the generic type's parameter list.
    pub parameter_index: u16,
}

#[capnp_conv(schema_capnp::type_::any_pointer::implicit_method_parameter)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAnyPointerImplicitMethodParameter {
    pub parameter_index: u16,
}
