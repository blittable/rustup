# Lesson Seven: HouseKeeping 

## Objectives 

* A few helpful notes and crate/project structure 


We've covered quite a bit of the fundamentals, so a few notes on building projects with Rust

** Import/Use syntax

Rust 2018 (a major release from, surprise, 2018), does not require 'extern crate in most cases' If the dependency is 
listed in the Cargo.toml, only 'use crate_name' is needed.

```Rust
 extern crate colored; // not needed in Rust 2018
    
    use colored::*;
``` 

** Syntax variations for crates and modules

For  ```somecrate::somemodule::somesubmodule::somefunction```

```
use somecrate::*;
```
Syntax will be:

```rust 
somemodule::somesubmodule::somefunction()
```

The invocation starts where the the use ends.

We can also: 

```
use somecrate::{SomeStruct, somefunction};
```

Or:
```
use somecrate::{SomeStruct, somefunction};
```

Or:
```
use somecrate::somesubmodule::{SomeStruct, somefunction};
```

One note on crate => module => submodule structure - Rust is not OO oriented

```
use somecrate::somemod;
```

** Use syntax variations


