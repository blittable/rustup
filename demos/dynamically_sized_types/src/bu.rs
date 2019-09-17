///The Sized Trait and dynamically sized types

fn main() {
    // Our slice contains
    let s1: &str = "Hello there!";
}

struct Foo<T>(T);
struct Bar<T: ?Sized>(T); // struct Foo

struct FooUse(Foo<[i32]>);

// error: Sized is not implemented for [i32]struct BarUse(Bar<[i32]>);
