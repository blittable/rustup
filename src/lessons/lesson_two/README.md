# Lesson Two: The Standard Library, Basic Types and Immutability 

## Objectives 

* Understand basic variable assignments, types and fundamental borrowing rules 
* Understand what the standard library is and when it might not be used

## The Foundation We are Standing on (normally): std

Take a look at the Rust standard library [std](https://doc.rust-lang.org/std/) 

Normally, we'll be using the standard library but it's not required. Embedded devices, may or may not have a file system, for example, and sans std lib, we can stillwcode Rust for those devices. 


Libraries or binaries/executables with:
```
#![no_std]
```

instruct the compiler to reject any use of the standard library.

Some good resources on embedded programming with Rust:
* [Discover embedded programming with Rust (good for getting started)] https://docs.rust-embedded.org/discovery/
* [The Embedded Rust Book] (https://docs.rust-embedded.org/book/) 



## Immutability 

By default Rust values are immutable and the compiler enforces compliance.

```rust
let x = 10;
x = 12;
```
produces a compile error.

```rust
let mut x = 10;
x = 12;
```

works because the variable is marked as mutable (changeable)


## Race Conditions, Fearless Concurrency and Why Rust Helps:

Let's say we have a Point of Sales system, and there are many run-time configuration options/rules.  A developer writes some code and then updates those options.  Developer B, in a different module, does the same.  When those updating events are called, at runtime, is unknown, as it depends on the system users. That's a basic race condition.  

Rust adds a bit of information to each memory allocation/variable - ownership.  Race conditions are not all bad (e.g. a multi-player game), but hidden race conditions are frequent source of bugs.  Rust's ownership and compile-time enforcement of ownership rules effectively 'outs' those race conditions.

Ownership is a central concept in Rust and all heap allocated data has an owner.  Changing ownership is called a 'Move'  Sharing of values is allowed.  Sharing a reference is called 'borrowing'.   Mutability comes in to play.  

1) ```MyVariable``` can be borrowed by multiple borrowers.
2) ```mut MyVariable``` can be borrowed by *one* borrower.

The second rule is our guard against unwittingly introducing or exposing a race condition.


## Chapter for from The Book

Chapter 4 from the Rust book has an excellent overview of this critical topic:
[The stack, heap, ownership and borrowing](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)


## General Notes
Borrowing gets us control over the scope and lifetime of memory allocations.  But, there is a price.  Recursive functions, when optimized, become an exercise in handling the borrows.  That's why a fibonacci sequence in a dynamically typed language can look like this:

(Lua)
```lua
function fib(n)
 if n == 0 or n == 1 then
 return n
 else
 return fib(n - 1) + fib(n - 2)
end
```

and in Rust (albeit a complex, optimized form) look like this:

(Rust)
```rust
fn fib(n_dyn: Rc<Any>) -> Rc<Any> {
 let n_static: &i32 =
    n_dyn.downcast_ref::<i32>().unwrap();
    if *n_static == 0 {
        Rc::new(Box::new(*n_static))
    } else {
    let n1 = fib(Rc::new(Box::new(n_static - 1)));
    let n2 = fib(Rc::new(Box::new(n_static - 2)));
    Rc::new(
        n1.downcast_ref::<i32>().unwrap() +
        n2.downcast_ref::<i32>().unwrap())
    }
}
```
- [source](http://cs242.stanford.edu/f18/assets/lectures/09-2-future-of-pl.pdf)








