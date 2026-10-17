#[allow(unused, clippy::all, clippy::pedantic)]
#[rustfmt::skip]
mod self_ref_capnp;
mod self_ref_rust;

use std::sync::Arc;

use self_ref_rust::{BoxedField, BoxedScalars, BoxedScalarsUnion, Inner, Kind, RecursiveList};

use crate::{
    assert_identical,
    self_ref::self_ref_rust::{ArcLeafNode, ArcLeafNodeParent, LeafNode, LeafNodeParent},
};

#[test]
pub fn keyword_rename_and_box_field() {
    let input = BoxedField {
        boxed: Box::new(Inner { value: 99 }),
    };
    assert_identical(&input);
}

#[test]
pub fn recursive_list() {
    let input = RecursiveList {
        value: 1,
        children: vec![
            RecursiveList {
                value: 2,
                children: vec![],
            },
            RecursiveList {
                value: 3,
                children: vec![RecursiveList {
                    value: 4,
                    children: vec![],
                }],
            },
        ],
    };
    assert_identical(&input);
}

#[test]
pub fn basic_tree() {
    let input = LeafNode {
        parent: LeafNodeParent::Parent(Box::new(LeafNode {
            parent: LeafNodeParent::Parent(Box::new(LeafNode {
                parent: LeafNodeParent::Root(()),
                description: "grandparent".to_owned(),
            })),
            description: "parent".to_owned(),
        })),
        description: "child".to_owned(),
    };
    assert_identical(&input);
}

#[test]
pub fn arc_tree() {
    let input = ArcLeafNode {
        parent: ArcLeafNodeParent::Parent(Arc::new(ArcLeafNode {
            parent: ArcLeafNodeParent::Parent(Arc::new(ArcLeafNode {
                parent: ArcLeafNodeParent::Root(()),
                description: "grandparent".to_owned(),
            })),
            description: "parent".to_owned(),
        })),
        description: "child".to_owned(),
    };
    assert_identical(&input);
}

#[test]
pub fn boxed_scalars() {
    let mut input = BoxedScalars {
        i32_val: Box::new(5),
        text_val: Arc::new("text".to_owned()),
        data_val: Box::new(vec![1, 2, 3]),
        enum_val: Box::new(self_ref_capnp::Kind::B),
        remote_val: Arc::new(Kind::B),
        list_val: Box::new(vec![4, 5, 6]),
        scalar_union: BoxedScalarsUnion::UnionI32(Box::new(7)),
    };
    assert_identical(&input);
    input.scalar_union = BoxedScalarsUnion::UnionText(Arc::new("union".to_owned()));
    assert_identical(&input);
}
