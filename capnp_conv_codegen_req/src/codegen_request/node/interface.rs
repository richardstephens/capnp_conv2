use crate::codegen_request::node::annotation::Annotation;
use crate::codegen_request::node::brand::Brand;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::node::interface)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceNode {
    /// Methods ordered by ordinal.
    pub methods: Vec<Method>,
    /// Superclasses of this interface.
    pub superclasses: Vec<Superclass>,
}

/// Schema for method of an interface.
#[capnp_conv(schema_capnp::method)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    /// Specifies order in which the methods were declared in the code.
    /// Like `Field.code_order`.
    pub code_order: u16,
    /// The parameters listed in `[]` (typically, type / generic parameters), whose bindings are
    /// intended to be inferred rather than specified explicitly, although not all languages
    /// support this.
    pub implicit_parameters: Vec<Parameter>,
    /// ID of the parameter struct type. If a named parameter list was specified in the method
    /// declaration (rather than a single struct parameter type) then a corresponding struct type
    /// is auto-generated. Such an auto-generated type will not be listed in the interface's
    /// `nested_nodes` and its `scope_id` will be zero -- it is completely detached from the
    /// namespace.
    pub param_struct_type: u64,
    /// Brand of param struct type.
    pub param_brand: Brand,
    /// ID of the return struct type; similar to `param_struct_type`.
    pub result_struct_type: u64,
    /// Brand of result struct type.
    pub result_brand: Brand,
    pub annotations: Vec<Annotation>,
}

#[capnp_conv(schema_capnp::superclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Superclass {
    pub id: u64,
    pub brand: Brand,
}

/// Information about one of the node's parameters.
#[capnp_conv(schema_capnp::node::parameter)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
}
