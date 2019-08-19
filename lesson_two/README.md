# Lesson Two: The Standard Library, Basic Types and Immutability 

## Objectives 

* Understand basic variable assignments, types and fundamental borrowing rules 
* Understand what the standard library is and when it might not be used


## The Foundation We are Standing on (normally): std

Take a look at the Rust standard library [std](https://doc.rust-lang.org/std/) 

Normally, we'll be using the standard library and it's not required to import.

However, not all Rust code requires or can use the standard library.  Embedded devices, may or may not have a file system, for example.  


Libraries or binaries/executables with:
```
#![no_std]
```

instruct the compiler to reject any use of the standard library.

Some good resources on embedded programming with Rust:
* [Discover embedded programming with Rust (good for getting started)] https://docs.rust-embedded.org/discovery/
* [The Embedded Rust Book] (https://docs.rust-embedded.org/book/) 



## Immutability 

By default Rust values are immutable.

```
let x = 10;
x = 12;
```
produces a compile error.

```
let mut x = 10;
x = 12;
```

works because the variable is marked as mutable (changeable)


## Race Conditions, Fearless Concurrency and Why Rust Helps:

Let's say we have a Point of Sales system, and there are many run-time configuration options/rules.  A developer writes some code and then updates those options.  Developer B, in a different module, does the same.  When those updating events are called, at runtime, is unknown, as it depends on the system users. That's a basic race condition.  


Rust has an important rule:

1) ```MyVariable``` can be borrowed by multiple borrowers.
2) ```mut MyVariable``` can be borrowed by *one* borrower.


## Task:

This section of chapter 4 from the Rust book is great.  Highly recommended:
[The stack, heap, ownership and borrowing](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)


## General Notes
Borrowing gets us control over the scope and lifetime of memory allocations.  But, there is a price.  Recursive functions become an exercise in handling the borrows.  That's why a fibonacci sequence in a dynamically typed language can look like this:

(Lua)
```
function fib(n)
 if n == 0 or n == 1 then
 return n
 else
 return fib(n - 1) + fib(n - 2)
end
```

and in Rust, look like this:

(Rust)
```
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
- http://cs242.stanford.edu/f18/assets/lectures/09-2-future-of-pl.pdf


## Homework 
Playtime!  Lesson 2 src is empty.  Try working with stack and heap assigned variables. Try to break the rules and review the compiler messages.  






