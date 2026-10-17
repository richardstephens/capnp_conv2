#[allow(unused, clippy::all, clippy::pedantic)]
#[rustfmt::skip]
mod unit_variant_capnp;
mod unit_variant_rust;

use std::marker::PhantomData;

use unit_variant_rust::{Bar, BarUnion, BasicStruct, Foo};

use crate::{assert_decodes_as, assert_identical, unit_variant::unit_variant_rust::Foo2};

#[test]
fn unnamed_union_unit_variant() {
    assert_identical(&Foo::Val1);
    assert_identical(&Foo::Val2(7));
}

#[test]
fn unnamed_union_unit_variant_roundtrips() {
    assert_decodes_as(&Foo::Val1, &Foo2::Val1(()));
    assert_decodes_as(&Foo2::Val1(()), &Foo::Val1);
    assert_decodes_as(&Foo::Val2(7), &Foo2::Val2(7));
    assert_decodes_as(&Foo2::Val2(7), &Foo::Val2(7));
}

#[test]
fn named_generic_union_unit_variant() {
    assert_identical(&Bar {
        named_union: BarUnion::<BasicStruct>::None,
    });
    assert_identical(&Bar {
        named_union: BarUnion::TVal(BasicStruct { val: 3 }),
    });
    assert_identical(&Bar {
        named_union: BarUnion::<BasicStruct>::Text("hi".to_owned(), PhantomData),
    });
    assert_identical(&Bar {
        named_union: BarUnion::<BasicStruct>::Renamed,
    });
}
