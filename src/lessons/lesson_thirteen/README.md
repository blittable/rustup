# Lesson Thirteen: Constructors Patterns 


OO programmers often think of Objects as the launching point of code, particularly, 'newing up' an object.  Although powerful, this approach can lead to monolithic 'engine' classes and interfaces that become black-boxes when the API or surface is exposed to other developers (though it needn't - we are aware). 

Rust programmers often think first in terms of shared behavior or functionality, so it's at least worth asking, "Am I coding shared functionality and is it best abstracted into a trait?"  Decoupling data structures and their behavior lends itself to thinking about that approach. 

Here we'll peek at some of the patterns (mostly from std::lib) used to construct things.   

## Objectives 
- Understand Constructor/Initalization Forms 

(https://doc.rust-lang.org/rust-by-example/fn/methods.html)


### Simple Static Constructor

Note: 
- This is the default impl for the struct 
- There's nothing special about `new` as function name here.
- `Self` or `&self` is never used. 
- The 'count' field might better be generic

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```


Usage:
``` let c = Counter::new();```


### Fingerprint Tuple Struct
Note: 
- The Fingerprint impl contains additional functions so,
so Fingerprint::ZERO hangs together with the impl 

```rust
pub struct Fingerprint(u64, u64);

impl Fingerprint {
    pub const ZERO: Fingerprint = Fingerprint(0, 0);

    ...other methods
}
```

Usage:
```
let zero_zero_tuple_struct = Fingerprint::ZERO;
```
or, equivilantly:
```rust
let zero_zero_tuple_struct = Fingerprint(0, 0);
```


Note: 
- Tuple and unit structs only (1 property) 
- There's nothing special about `new` as function name here.


```
struct AbitSelfishCounter { count: u32 };

impl ABitSelfishCounter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

```

Usage:
```
let a_bit_selfish_counter = AbitSelfishCounter::new();
```







