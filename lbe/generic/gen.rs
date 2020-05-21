struct Foo;
impl Foo {
    fn foo(&self) { println!("Foo::foo()"); }
}

struct GenVal<T> {
    val: T
}

impl <U> GenVal<U> {
    fn value(&self) -> &U { &self.val }
    //似乎天生自带concepts，不能这么写
    //fn bar(&self) { self.val.foo(); }
}

fn main() {
    let v1 = GenVal { val: 2.3f32 };
    println!("{}", v1.value());
    let v2 = GenVal { val: Foo{} };
}