pub mod annotation;
pub mod annotation_node;
pub mod brand;
pub mod const_node;
pub mod enum_node;
pub mod field;
pub mod interface;
pub mod struct_node;
pub mod types;
pub mod value;

use crate::codegen_request::node::annotation::Annotation;
use crate::codegen_request::node::annotation_node::AnnotationNode;
use crate::codegen_request::node::const_node::ConstNode;
use crate::codegen_request::node::enum_node::EnumNode;
use crate::codegen_request::node::interface::{InterfaceNode, Parameter};
use crate::codegen_request::node::struct_node::StructNode;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::node)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    /// Name to present to humans to identify this Node. You should not attempt to parse this. Its
    /// format could change. It is not guaranteed to be unique.
    pub display_name: String,
    /// If you want a shorter version of `display_name` (just naming this node, without its
    /// surrounding scope), chop off this many characters from the beginning of `display_name`.
    pub display_name_prefix_length: u32,
    /// ID of the lexical parent node. Typically, the scope node will have a `NestedNode` pointing
    /// back at this node, but robust code should avoid relying on this (and, in fact, group nodes
    /// are not listed in the outer struct's `nested_nodes`, since they are listed in the fields).
    /// `scope_id` is zero if the node has no parent, which is normally only the case with files,
    /// but should be allowed for any kind of node (in order to make runtime type generation
    /// easier).
    pub scope_id: u64,
    /// If this node is parameterized (generic), the list of parameters. Empty for non-generic
    /// types.
    pub parameters: Vec<Parameter>,
    /// True if this node is generic, meaning that it or one of its parent scopes has a non-empty
    /// `parameters`.
    pub is_generic: bool,
    /// List of nodes nested within this node, along with the names under which they were declared.
    pub nested_nodes: Vec<NestedNode>,
    /// Annotations applied to this node.
    pub annotations: Vec<Annotation>,
    /// Info specific to each kind of node.
    #[capnp_conv(type = "unnamed_union")]
    pub kind: NodeKind,
}

#[capnp_conv(schema_capnp::node)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    File(()),
    #[capnp_conv(type = "group")]
    Struct(StructNode),
    #[capnp_conv(type = "group")]
    Enum(EnumNode),
    #[capnp_conv(type = "group")]
    Interface(InterfaceNode),
    #[capnp_conv(type = "group")]
    Const(ConstNode),
    #[capnp_conv(type = "group")]
    Annotation(AnnotationNode),
}

#[capnp_conv(schema_capnp::node::nested_node)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedNode {
    /// Unqualified symbol name. Unlike `Node.display_name`, this *can* be used programmatically.
    pub name: String,
    /// ID of the nested node. Typically, the target node's `scope_id` points back to this node,
    /// but robust code should avoid relying on this.
    pub id: u64,
}
