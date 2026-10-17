use std::marker::PhantomData;

use capnp_conv2::capnp_conv;

use super::unit_variant_capnp::{bar, basic_struct, foo};

#[capnp_conv(foo)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Foo {
    Val1,
    Val2(u8),
}

#[capnp_conv(foo)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Foo2 {
    Val1(()),
    Val2(u8),
}

#[capnp_conv(bar)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bar<T> {
    #[capnp_conv(type = "union")]
    pub named_union: BarUnion<T>,
}

#[capnp_conv(bar::named_union)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarUnion<T> {
    None,
    TVal(T),
    Text(String, PhantomData<*const T>),
    #[capnp_conv(name = "capVal")]
    Renamed,
}

#[capnp_conv(basic_struct)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicStruct {
    pub val: i32,
}
