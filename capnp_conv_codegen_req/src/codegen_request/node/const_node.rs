use crate::codegen_request::node::types::Type;
use crate::codegen_request::node::value::Value;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::node::const_)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstNode {
    #[capnp_conv(name = "type")]
    pub type_: Type,
    pub value: Value,
}
