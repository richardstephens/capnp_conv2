#[allow(unused, clippy::all, clippy::pedantic)]
#[rustfmt::skip]
mod self_ref_capnp;
mod self_ref_rust;

use self_ref_rust::{BoxedField, Inner, RecursiveList};

use crate::assert_identical;
use crate::self_ref::self_ref_rust::{LeafNode, LeafNodeParent};

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
