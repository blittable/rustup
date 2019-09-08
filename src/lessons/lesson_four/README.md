# Lesson Four: Enum Wrappers and Some Type Basics 

## Objectives 

* Understand working without NULL (you won't miss it) and the std lib enum wrappers Option<T> Result<Err, T> 
* Understand how to import a library/crate and reference it in a source file 

## NULL, std::option and Enum Wrappers

But, we still have uninitialized values, right?   Our data structures need a placeholder for runtime initialized values without arbitrary results.

There's no NULL in Rust. 

```
if (x == null) ... is gone.
```

The std::option crate has an std::option::Option type and it is used *very* often in Rust. In fact, we do not need to import it. 

```rust
pub enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}

```

So, typically, if it is optional we define it as such:

```
struct Dieter {
    name: String,
    daily_calories: <Option<u8>>   // <- quite a strict diet
}
```


While not required, another built-in enum in the std library is the Result<T, E>  which is used for returning values which can then be 'unwrapped' for errors OR thrown further up the call stack.

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

*** See demo match_101 and match_102 for examples 

