# Lesson Four: Enums and Type Basics 

## Objectives 

* Understand working without NULL (you won't miss it) 
* Get familar with the standard library's enum wrappers `Option<T>` `Result<Err, T>` 
* Understand how to import a library/crate and reference it in a source file 

## NULL, std::option and Enum Wrappers

There's no NULL type in Rust. 

```
if (x == null) ... is gone.
```

The `std::option::Option` type (no need to import) is used *very* often in Rust, typically to handle our NULL scenario - indicated the presence or absence of data. 

```rust,no_run
pub enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}

```

So, typically, if it is optional we define it as such:

```rust,no_run
struct Dieter {
    name: String,
    daily_calories: <Option<u8>>   // <- tough diet, u8 is an unsigned 8 bit byte
}
```

The dieter object may or may not have a defined amount of `daily_calories` and access to the value must be preceded by a check for its presence (Some or None).  

Another built-in enum in the std library is the `Result<T, E>` which is used for returning values which can then be 'unwrapped' for errors OR propograted/thrown further up the call stack.

The definition, likewise, is simple:

```
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

And the typical usage:

```rust

use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}

```

REF: match_101 and match_102 

