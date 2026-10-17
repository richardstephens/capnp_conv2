use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

/// Additional information about a node which is not needed at runtime, but may be useful for
/// documentation or debugging purposes. This is kept in a separate struct to make sure it
/// doesn't accidentally get included in contexts where it is not needed. The
/// `CodeGeneratorRequest` includes this information in a separate array.
#[capnp_conv(schema_capnp::node::source_info)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// ID of the Node which this info describes.
    pub id: u64,
    /// The top-level doc comment for the Node.
    pub doc_comment: String,
    /// Information about each member -- i.e. fields (for structs), enumerants (for enums), or
    /// methods (for interfaces).
    ///
    /// This list is the same length and order as the corresponding list in the Node, i.e.
    /// `Node.struct.fields`, `Node.enum.enumerants`, or `Node.interface.methods`.
    pub members: Vec<SourceInfoMember>,
}

#[capnp_conv(schema_capnp::node::source_info::member)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfoMember {
    /// Doc comment on the member.
    pub doc_comment: String,
}
