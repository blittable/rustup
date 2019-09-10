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
