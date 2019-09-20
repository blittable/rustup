# Lesson Seven: Import Syntax and Project Structure 

## Objectives 

We've covered quite a few of the fundamentals. Housekeeping lessons are quick visit on a series of topics.

## Import/Use syntax

Rust 2018 (a major release from, surprise, 2018), does not require 'extern crate' in most cases. If the dependency is 
listed in the Cargo.toml, only 'use crate_name' is needed.  Older sample code may indicate otherwise.

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

The invocation starts where the the use statement ends.

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

There is a 'top level' to any library or binary (executable) and it is named in the cargo.toml

The 'top level' is also responsible for exporting public members (using the ```pub``` keyword)

Typical Project Structure:


      main.rs    or   lib.rs               
                  |      
  subdirectory_01  subdirectory_02         
         |              |                  
       mod.rs         mod.rs            


- You can use `mod` to create modules anywhere

- The *parent* controls the visibility of the children who are public by 're-exposing' them, e.g.:
```pub mychildmod```

- This likewise applies to the top-level crate which selectively makes other crates and functions public


### Importing Macros

This needs to happen at the top level of the crate
```
#[macro_use]
extern crate criterion;
```

### Tuples: Multiple Types - One Data Structure

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
// over in a way that allows modifying each value, note the dereference operator
for x in xs.iter_mut() {
    *x *= 3;
    println!("Updated vector: {:?}", xs);
}
```
### Cargo.toml 

The [Cargo.toml manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) is extremely powerful and flexible.

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
