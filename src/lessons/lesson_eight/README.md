# Lesson Eight: Closures and Lifetime Preview 

## Objectives 

- Understand the basics of closures
- Preview lifetime annotations 

### Closure Essentials   

- Closures look *a lot* like functions

```rust
fn  plus_one_v1   (x: i32) -> i32 { x + 1 };
let plus_one_v2 = |x: i32| -> i32 { x + 1 };
let plus_one_v3 = |x: i32|          x + 1  ;
``` 
- Closures can be parameters to functions, return values from functions, 'in-line' as lambda expressions,
and assigned to a variable and 'invoked' later.  They are `lazy` initialized. 

- Closures that mutate values must be annotated as ```mut``` 
```rust
  let mut a_closure = |t| t = t + 1; 
```

- The ```move``` keyword makes the closure take ownership of the referenced values.  (x in this case) 

```rust
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

- A closure with an explicit type annotation

```rust
    let haystack = vec![1, 2, 3];

    let contains = |needle: &i32| haystack.contains(needle);

    println!("{}", contains(&1));
```

The closure above is called *exactly* as a method/function is called.

The ```iter()``` trait exposes some excellent functions detailed [here](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods)
and summarized with *impressive fonts* [here](https://danielkeep.github.io/itercheat_baked.html)


### Introduction to Lifetimes

We know Rust is good at throwing away things.  Out of scope, out of memory.  That, however, introduces a problem.  We don't always want to throw away associated values.  We want them to *live* longer.  The compiler tracks the 'lifetime' of values (all of them actually), but when it sees code that uses values beyond the obvious scope (lifetime), it requies an explicit annotation of the lifetime.  

For now - just read ```'a``` as 'lifetime annotation' and note (obviously?), that type parameters (like 'a) have to be used. 
```rust
struct Foo<'a> {
    x: &'a i32,
}
```

### Homework

The homework task touches on a few topics: Vectors, iterators, and closures.  

The ```iter``` trait has many functions like ```map``` and ```filter``` that take closures as arguments. 
They do this by implementing FnMut, which is one way, together with ```FnOnce``` and ```Fn`` to control how closures capture their surrounding values.  

Practically, this let's us pipe the ```iter`` results to other functions in a nice, functional style.  

Note: this homework contains *2* binaries.  See the Crate.toml to see how that is done.  ```cargo run`` will not work, however.  You need to specify
which binary you want to run.  In our case, either:

```cargo run --bin buggy```
or
```cargo run --bin iterclosures``

The first exercise is to fix a bug in the ```buggy``` app.  See the source file for TODO and details. (extra credit with prize!: there's a bug in the simple fix, handle it.)

The second exercise is to chain some ```iter()``` results and ```collect()``` them.  Play with some of the operators [here](https://danielkeep.github.io/itercheat_baked.html)
But, make sure you use at least *two closures* in your solution.


