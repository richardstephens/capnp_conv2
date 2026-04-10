use crate::codegen_request::node::brand::Brand;
use crate::codegen_request::node::value::Value;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

/// Describes an annotation applied to a declaration. Note `AnnotationNode` describes the
/// annotation's declaration, while this describes a use of the annotation.
#[capnp_conv(schema_capnp::annotation)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// ID of the annotation node.
    pub id: u64,
    /// Brand of the annotation.
    ///
    /// Note that the annotation itself is not allowed to be parameterized, but its scope might be.
    pub brand: Brand,
    pub value: Value,
}
