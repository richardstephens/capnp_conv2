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
