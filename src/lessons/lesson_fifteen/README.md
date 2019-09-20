# Lesson Fifteen: Copy, Clone, Drop and Sized Traits, Problem Preview 

## Objectives 
- Understand The Copy, Clone, Drop and Sized Traits 

### Lesson 12 Assignment Review/Discussion

[One Solution](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=183d5e20941f9d96071260c036c12e10)

## Three Special Traits: Copy, Clone, Size

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

## Three Special Traits: Copy, Clone, Size
<!-- 
https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait
let x: Display = ...;
In this case, the compiler does not know which type is actually used here, it is erased, therefore it does not know the size of values of these types. The above line is not valid - you can't make a local variable without knowing its size (to allocate enough bytes on the stack), and you can't pass the value of an unsized type into a function as an argument or return it from one.

Unsized types can be used through a pointer, however, which can carry additional information - the length of available data for slices (&[u32]) or a pointer to a virtual table (Box<SomeTrait>). Because pointers always have a fixed and known size, they can be stored in local variables and be passed into or returned from functions.

Given any concrete type you can always say whether it is sized or unsized. With generics, however, a question arises - is some type parameter sized or not?

fn generic_fn<T>(x: T) -> T { ... }
If T is unsized, then such a function definition is incorrect, as you can't pass unsized values around directly. If it is sized, then all is OK.

In Rust all generic type parameters are sized by default everywhere - in functions, in structs and in traits. They have an implicit Sized bound; Sized is a trait for marking sized types:

fn generic_fn<T: Sized>(x: T) -> T { ... }
This is because in the overwhelming number of times you want your generic parameters to be sized. Sometimes, however, you'd want to opt-out of sizedness, and this can be done with ?Sized bound:

fn generic_fn<T: ?Sized>(x: &T) -> u32 { ... } -->

### Problem Preview (Box, Ref, RefCell)

The exercise/assigment is to review the problem presented in this code.  

{{#playpen src/main.rs}}



