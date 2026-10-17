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

struct LeafNode {
  union {
    root @0 :Void;
    parent @1 :LeafNode;
  }
  description @2 :Text;
}

enum Kind {
  a @0;
  b @1;
}

struct BoxedScalars {
  i32Val    @0 :Int32;
  textVal   @1 :Text;
  dataVal   @2 :Data;
  enumVal   @3 :Kind;
  remoteVal @4 :Kind;
  listVal   @5 :List(Int32);
  union {
    unionI32  @6 :Int32;
    unionText @7 :Text;
  }
}
