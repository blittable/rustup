# Lesson Seven: HouseKeeping 

## Objectives 

We've covered quite a bit of the fundamentals, so a few notes on building projects with Rust and a few other Helpful Details
## Import/Use syntax

Rust 2018 (a major release from, surprise, 2018), does not require 'extern crate in most cases' If the dependency is 
listed in the Cargo.toml, only 'use crate_name' is needed.

```rust
 extern crate colored; // not needed in Rust 2018
    
use colored::*;
``` 

## Syntax variations for crates and modules

For  somecrate::somemodule::somesubmodule::somefunction

```rust
use somecrate::*;
```
Syntax will be:

```rust
somemodule::somesubmodule::somefunction()
```

The invocation starts where the the use ends.

We can also: 

```rust
use somecrate::{SomeStruct, somefunction};
```

Or:
```rust
use somecrate::{SomeStruct, somefunction};
```

Or:
```rust
use somecrate::somesubmodule::{SomeStruct, somefunction};
```

## Project Structure

There is a 'top level' to any library or binary (executable)
and it is named in the cargo.toml

The 'top level' is also responsible for exporting public members (using the ```pub``` keyword)

Typical Project Structure:


      main.rs    or   lib.rs               
                  |      
  subdirectory_01  subdirectory_02         
         |              |                  
       mod.rs         mod.rs            


- You can use `mod` to create modules anywhere
- The *parent* controls the visibility of the children who are public by 'reexposing' them, e.g.:
```pub mychildmod```


### Importing Macros

This needs to happen at the top level of the crate
```
#[macro_use]
extern crate criterion;
```

### Tuples - Multiple Types - One datastructure

```rust
let (a, b, c, d) = ("I'm".to_string(), "a".to_string(), "tuple".to_string(), 32);
```

### Vectors - A Common Rust Datastructure 

For vectors, we get some 'standard' operations: push, pop, index (e.g. our_vector[0])

```rust
let mut collected_iterator: Vec<i32> = (0..10).collect();
println!("Collected (0..10) into: {:?}", collected_iterator);

// The `vec!` macro can be used to initialize a vector
let mut xs = vec![1i32, 2, 3];
println!("Initial vector: {:?}", xs);

// Thanks to `iter_mut`, mutable `Vector`s can also be iterated
// over in a way that allows modifying each value, note the derference operator
for x in xs.iter_mut() {
    *x *= 3;
    println!("Updated vector: {:?}", xs);
}
```
### Cargo.toml 

The [Cargo.toml manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) is extremely powerful and flexible
At a high-level, you can:
- Split 'features' within your crate to make optional dependencies
- Split the workspace for multiple crates joined in a single toml.lock
- Override dependencies
- Use files or git repositories as the source of dependencies
- Support multiple libraries or binaries

one example:

```rust
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

Homework:

1) Create a project that has multiple directories (more than 2!) with separate modules.
2) Use a Vec in the project
3) Use a tuple in the project




