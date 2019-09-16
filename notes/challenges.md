
WT*: Opaque Snippets - Not transparent in passing 

 
[ ] 'hidden' iterator return types in chaining

```rust
foo.map()  
```

[ ] 'RHS' right hand side 

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

[ ] Precision prefix and numeric type hints 

```
 value: 4f32
```

[ ] Use of annotation for static

Unlike let bindings, you must annotate the type of a static.
Statics live for the entire lifetime of a program, and therefore any reference stored in a constant has a 'static lifetime:

```rust
static N: i32 = 5;
static NAME: &'static str = "Steve";
```


Representations [ ]

https://doc.rust-lang.org/reference/type-layout.html#representations

```
#[repr(C)]
struct ThreeInts {
    first: i16,
    second: i8,
    third: i32
}

