use std::sync::Arc;

use capnp_conv2::capnp_conv;

#[allow(unused, clippy::all, clippy::pedantic)]
use super::self_ref_capnp as capnp_types;

#[capnp_conv(capnp_types::inner)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inner {
    pub value: i32,
}

#[capnp_conv(capnp_types::boxed_field)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxedField {
    pub boxed: Box<Inner>,
}

#[capnp_conv(capnp_types::recursive_list)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveList {
    pub value: i32,
    pub children: Vec<RecursiveList>,
}

#[capnp_conv(capnp_types::leaf_node)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafNode {
    #[capnp_conv(type = "unnamed_union")]
    pub parent: LeafNodeParent,
    pub description: String,
}

#[capnp_conv(capnp_types::leaf_node)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafNodeParent {
    Root(()),
    Parent(Box<LeafNode>),
}

#[capnp_conv(capnp_types::leaf_node)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArcLeafNode {
    #[capnp_conv(type = "unnamed_union")]
    pub parent: ArcLeafNodeParent,
    pub description: String,
}

#[capnp_conv(capnp_types::leaf_node)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArcLeafNodeParent {
    Root(()),
    Parent(Arc<ArcLeafNode>),
}

#[capnp_conv(capnp_types::Kind)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    A,
    B,
}

#[capnp_conv(capnp_types::boxed_scalars)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::box_collection)]
pub struct BoxedScalars {
    pub i32_val: Box<i32>,
    pub text_val: Arc<String>,
    #[capnp_conv(type = "data")]
    pub data_val: Box<Vec<u8>>,
    #[capnp_conv(type = "enum")]
    pub enum_val: Box<capnp_types::Kind>,
    #[capnp_conv(type = "enum_remote")]
    pub remote_val: Arc<Kind>,
    pub list_val: Box<Vec<i32>>,
    #[capnp_conv(type = "unnamed_union")]
    pub scalar_union: BoxedScalarsUnion,
}

#[capnp_conv(capnp_types::boxed_scalars)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxedScalarsUnion {
    UnionI32(Box<i32>),
    UnionText(Arc<String>),
}
