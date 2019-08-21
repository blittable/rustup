# Lesson Four: Enum Wrappers and Import References

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
    daily_calories: <Option<u8>>
}
```

## Homework 






