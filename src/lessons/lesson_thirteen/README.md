# Lesson Thirteen: Constructors Patterns 


OO programmers often think of Objects as the launching point of code, particularly, 'newing up' an object.  

Although powerful, this approach can lead to monolithic 'engine' classes and that become black-boxes when the API or surface is exposed to other developers (though it needn't - we are aware). 

Rust programmers often think first in terms of shared behavior or functionality, so it's at least worth asking, "Is this shared functionality and is it best abstracted into a trait?"  

Here we'll peek at some of the patterns (mostly from std::lib) used to construct things.   

## Objectives 
- Understand Constructor/Initalization Forms 


### Constructor Patterns (Comments in code) 


Notes: 

```rust
//PATTERN ONE: static, default impl 
//This is the default impl for the struct 
//There's nothing special about `new` as function name here.
//`Self` or `&self` is never used. 
#[derive(Debug, PartialEq)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//PATTERN TWO: static, Self
//Exactly the same. Basic struct, but with 'Self' as return type
struct SelfishCounter {
    count: u32,
}

impl SelfishCounter {
    fn new() -> Self {
        SelfishCounter { count: 0 }
    }
}

//PATTERN THREE: Self -> Self
//Exactly the same as above, but `Self` is used for the 
//return value and return type, rather than the struct name 
struct ABitSelfishCounter {
    count: u32,
}

//Using the 'Self' constructor
impl ABitSelfishCounter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

fn main() {
    //PATTERN ONE
    let counter = Counter::new();
    assert_eq!(counter.count, 0);

    //PATTERN TWO 
    let selfish_counter = SelfishCounter::new();
    assert_eq!(selfish_counter.count, 0);

    //PATTERN THREE 
    let a_bit_selfish_counter = ABitSelfishCounter::new();
    assert_eq!(a_bit_selfish_counter.count, 0);

    #println!("If we ran to here without an error in the asserts, it was successful.");

}
```

### Builder Pattern


https://docs.rs/derive_builder/0.7.2/derive_builder/

```rust,no_run
 let person = PersonBuilder::default().name("krishnab").build();
 ```

### Tuple Struct

```rust,no_run
//PATTERN FOUR: Tuple Struct
pub struct Fingerprint(u64, u64);

impl Fingerprint {
    pub const ZERO: Fingerprint = Fingerprint(0, 0);
    pub fn hi() {} //... more code
}


```
Usage:
```
let zero_zero_tuple_struct = Fingerprint::ZERO;
```
or, equivilantly:
```rust,no_run
let zero_zero_tuple_struct = Fingerprint(0, 0);
```

```rust

#[derive(Debug, PartialEq)]
pub struct Fingerprint(u64, u64);

impl Fingerprint {
    pub const ZERO: Fingerprint = Fingerprint(0, 0);
    pub fn hi() {} //... more code
}

fn main() {
    let fp = Fingerprint(7, 7);
    assert_eq!(fp.0, 7);

    let zeroed = Fingerprint::ZERO;
    assert_ne!(zeroed.0, fp.0);

    let zeroed_two = Fingerprint(0, 0);
    assert_eq!(zeroed, zeroed_two);

    #println!("If we ran to here without an error in the asserts, it was successful.");
}
```