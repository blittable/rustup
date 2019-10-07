# Rust Top-Down 

This is an overview/tutorial of the Rust programming language for progrmamers coming from 'higher' level languages.

Rust is best groked from a C++/C perspective, so some content that attends to the background of programmers with Java/.NET/Python backgrounds lets us skip a few topics, but more importantly post warning signs where OO thinking may introduce obstacles to success. 

[The Rust Programming Language - aka 'The Book'](https://doc.rust-lang.org/book/) is a *great* resource and should certainly be used in conjuction with the content if you are new to Rust programming.  

Rust code and libraries are also easily 'self-documented', and one of the best examples is the `std` crate.  (a `crate` being the equivilant of an npm or nuget package).  [std](https://doc.rust-lang.org/std/)  For core concepts (e.g. references, types, etc.), the api documentation often provides conceptual context that is very helpful. 

A personal favorite, though sparse in parts, is the [reference book].(https://doc.rust-lang.org/stable/reference/) It's readable and not strictly a formal reference.

Keep in mind that post-install, you have the core documentation installed locally.  Launch with ```rustup doc```

## How to Use 

* The content/progress is fairly linear.

* The README for each chapter explains concepts and then typically presents an exercise.

* If you fork or clone this repository, most of the exercises are in the /src folder.  

* You know how you learn best, but taking the examples, tweeking them, and playing with the problem/topics works well for most. 

* The original content was developed as part of an in-house course on Rust, and is a WIP, so some oddities may appear. 


## Objectives
* Provide an introduction to Rust that will get you moving, quickly
* Keep the lessons small and focused, one or two topics, max
* Build a solid foundation in Rust for further development
* Provide contextual information for OO programmers looking for a frame of reference

## Other Notes and Tips

* Resist the temptation to skim the Rust documention, jump over to github, pull a random project and start compiling.  A good foundation will get you moving faster.
* Rust code can be a bit intimidating at first glance (and even after a few more glances). However you're not obliged to use the advanced features of the language to get moving.  There are solid applications and libraries that are light on generics, inter-thread communication, macros, unsafe code, etc.
