# Lesson Fifteen: Copy and Clone Traits, Problem Preview 

## Objectives 
- Understand The Copy and Clone Traits 

### Lesson 12 Assignment Review/Discussion

[One Solution](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=183d5e20941f9d96071260c036c12e10)

### Two Special Traits: Copy, Clone

The ```Copy``` trait uses mem::copy underneath and is implicit (you can use = operator),  ```Clone``` is explicit (.clone()) 

```rust
#[derive(Copy, Clone)]
struct Motocy;
```

```rust
struct Motorcy;

impl Copy for Motorcy { }

impl Clone for Motorcy {
    fn clone(&self) -> Motorcy {
        *self
    }
}
```

[Copy/Clone Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=291d8a0f3d018fd84909a45475b1af57)


### Problem Preview (Box, Ref, RefCell)

The exercise/assigment is to review the problem presented in this code.  

{{#playpen src/main.rs}}



