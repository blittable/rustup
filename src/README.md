# Rust for OO Programmers

This is an overview/tutorial of the Rust programming targeting object oriented programmers.  

Rust is best groked from a C++/C perspective, so some content that attends to the background of OO programmers lets us skip a few topics, but more importantly post warning signs 
where OO thinking may introduce obstacles in the process.  By OO, we mean a statically-typed language where 'Object' sits at the top
of the type heirarchy, such as any .NET language or Java.

[The Rust Programming Language - aka 'The Book'](https://doc.rust-lang.org/book/) is a *great* resource and should certainly be used in conjuction with the content if you are new to Rust programming.  

Rust code and libraries are also easily 'self-documented', and one of the best examples is the std crate.  (a `crate` being the equivilant of an npm or nuget package).  [std](https://doc.rust-lang.org/std/)

A personal favorite, though spare in parts, is the [reference book](https://doc.rust-lang.org/stable/reference/) It's readable and not strictly a formal reference.

Lest you find a broken link while searching doc, try the local documentation installed with the toolchain ```rustup doc```

## How to Use 

* The content/progress is fairly linear, but an attempt has been made to minimize the prerequisite topics where possible. 

* If you clone this repository, most of the exercises are in the /src folder of the lesson.  The README for the individual lesson will detail those.

* You know how you learn best, but taking the examples, tweeking them, and playing with the problem/topic works well for most. 

* The original content was developed as part of an in-house course on Rust, so you may see some remnants that don't make sense. 


## Objectives
* Provide an introduction to Rust that will get you moving, quickly
* Keep the lessons small and focused, one or two topics, max
* Build a solid foundation in Rust for further development
* Provide contextual information for OO programmers looking for a frame of reference
* Provide some motiviation by looking at some interesting Rust projects (mostly during discussions) 

## Other Notes and Tips

* Resist the temptation to skim the Rust documention, jump over to github, pull a random project and start compiling.  A good foundation will get you moving faster.
* Rust code can be a bit intimidating at first glance (and even after a few more glances). However you're not obliged to use the advanced features of the language to get moving.  There are solid applications and libraries that are light on generics, inter-thread communication, macros, unsafe code, etc.