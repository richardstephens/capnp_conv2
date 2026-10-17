@0xe3b1c4a9f2d85b71;

struct Foo {
  union {
    val1 @0 :Void;
    val2 @1 :UInt8;
  }
}

struct Bar(T) {
  namedUnion :union {
    none @0 :Void;
    tVal @1 :T;
    text @2 :Text;
    capVal @3 :Void;
  }
}

struct BasicStruct {
  val @0 :Int32;
}
