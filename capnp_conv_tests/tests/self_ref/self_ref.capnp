@0xe193f15d5ad9869b;

struct Inner {
  value @0 :Int32;
}

struct BoxedField {
  boxed @0 :Inner;
}

struct RecursiveList {
  value @0 :Int32;
  children @1 :List(RecursiveList);
}
