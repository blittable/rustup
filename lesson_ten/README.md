# Lesson Nine: Unit Testing and bit of Housekeeping 

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

## Examples

It's common in rust code, especially on github, to put examples in an ```/examples``` directory.  Cargo/Rust supports that
convention:

```cargo run --example [example_name]```


### TEST!!!



### Homework

This is homework is a short one:
Create a simple function, and writing a failing unit test
