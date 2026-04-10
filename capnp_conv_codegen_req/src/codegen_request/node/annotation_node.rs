use crate::codegen_request::node::types::Type;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::node::annotation)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationNode {
    #[capnp_conv(name = "type")]
    pub type_: Type,
    pub targets_file: bool,
    pub targets_const: bool,
    pub targets_enum: bool,
    pub targets_enumerant: bool,
    pub targets_struct: bool,
    pub targets_field: bool,
    pub targets_union: bool,
    pub targets_group: bool,
    pub targets_interface: bool,
    pub targets_method: bool,
    pub targets_param: bool,
    pub targets_annotation: bool,
}
