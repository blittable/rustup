* borrowing and the Copy trait - fundamental tests rely on String, but first attempts likely to be basic types, e.g. i32
* disambiguation of generic and traits in type parameters
* ```&'static str``` isn't too friendly 


let x: str = "foo";
let y: mut str = "foo";


Nice Things:

- Package management

WT:

trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
