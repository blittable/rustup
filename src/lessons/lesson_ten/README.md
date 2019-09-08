# Lesson Nine: Unit Testing 

## Objectives 
- Unit Testing
- Tests directory, benches, and examples 


## Testing

By convention, integration tests are added in the /tests directory.  They import the crate(s) they are testing.

Standard unit tests are normally added in the source files, sometimes inside a separate ```mod tests```

The #[cfg(test)] macro attribute gives some control over how they are run and exluded from releases.

### Unit Tests are normally created with:

```#[test]```

```rust
assert!(expression)
assert_eq!(left, right)
assert_ne!(left, right)
```

And run with ```cargo test```

### Running Unit Tests with Output

If you use output in unit tests, like println!(), the output is hidden unless you add --no-capture

```
cargo test -- --nocapture
```


## Examples

It's common in rust code, especially on github, to put examples in an ```/examples``` directory.  Cargo/Rust supports that
convention:

```cargo run --example [example_name]```

### TEST!!!

This is a refresher on borrowing and mutablity.
- In the /demos/test_lesson_10 you will find a set of functions calls commented out.
- Review the functions and ask yourself, "does this compile?"

### Homework

This is homework is a short one:
Create a simple function, and write a unit test for that function
