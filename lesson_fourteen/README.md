# Lesson Thirteen: Iterator Patterns 

Idomatic Rust uses 
is different.

Here we'll review some of the main forms.

## Objectives 
- Understand Constructor Forms 

(https://doc.rust-lang.org/rust-by-example/fn/methods.html)


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

