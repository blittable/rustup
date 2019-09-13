Clear Challenges:

---
* borrowing and the Copy trait - fundamental tests rely on String, but first attempts likely to be basic types, e.g. i32
* disambiguation of generic and traits in type parameters
* ```&'static str``` isn't too friendly 

let x: str = "foo";
let y: mut str = "foo";
---
'hidden' iterator return types in chaining
---


WT*: Snippets for Exploring

ID ME:


--- 
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

---
 value: 4f32

 ---

## Static 

static N: i32 = 5;

Unlike let bindings, you must annotate the type of a static.
Statics live for the entire lifetime of a program, and therefore any reference stored in a constant has a 'static lifetime:

static NAME: &'static str = "Steve";
---

# Representations

https://doc.rust-lang.org/reference/type-layout.html#representations

#[repr(C)]
struct ThreeInts {
    first: i16,
    second: i8,
    third: i32
}

Type Alias and _: identifier
let _: F = E::A;