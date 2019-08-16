Lesson One

Getting started with Rust

## Objectives 

* Get Rust installed
* Create a new project
* Add a function to the new project that returns a value


## Lesson:

### Task: Installing

[Installing Rust](https://www.rust-lang.org/) - This installs the toolchain (compiler and package manager (cargo) - Windows, OSX, Linux and other OS's are supported.   

* Verify your installation worked by opening a command/terminal prompt and typing:

``rustup```
```rustc```
```cargo```

rustc is the compiler, but it's normally called by running a 'cargo' command (e.g. cargo build)

#### Motivation

When you type rustc, look at the option for 'emit' that specifies the output format, including asm (think web assembly) and llvm-ir (ask Khem)

```
--emit [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
                        Comma separated list of types of output for the
                        compiler to emit
```

### Task: Launch the Doc 

You're going to need it.  ;)  

```rustup doc```

### Task: Review the cargo.toml

Take a look at the format of the cargo.toml.  That's the package.json / .proj  file for Rust.

The most important part is the 'dependencies' section.  There are dependencies and dev-dependencies, similar to javascript-land and many other options  [Rust manifest format and details](https://doc.rust-lang.org/cargo/reference/manifest.html)  

Lesson two will cover adding a dependency.

### Homework 

After you've cloned this project and created your branch (see README at the root of repository), fix the lesson code.

The objective of the homework is to build a rust project that has one function (main) call another function. You will likely hit a problem.  The problem is an important part of what Rust is all about.  For this lesson, just try to get it to work, and don't worry about trying to underst and all the details. (There are many (reference types, borrowing, String vs. &str, etc.)) 

### Notes 

* The println! function lives in std::io [println](https://doc.rust-lang.org/std/macro.println.html) The std library contains lots of useful modules, functions, types, and macros, but is not strictly necessary. 

* Read the compile error messages.  They are helpful.
* The little '''!''' at the end of println is not 'not' - it's means it's a macro.  
* '''eprintln!''' prints errors and progress messages


