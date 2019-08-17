# Lesson One: Setup and Calling a Function

## Objectives 

* Get Rust installed
* Look at the toolchain (rustc, cargo, rustup)
* Create a function that returns a value

## Lesson:

### Task: Installing

[Installing Rust](https://www.rust-lang.org/) - This installs the toolchain (compiler and package manager (cargo) - Windows, OSX, Linux and other OS's are supported.   

* Verify your installation worked by opening a command/terminal prompt and typing:

```
rustup
rustc
cargo
```

### Task: Get to Know the toolchain 

rustc is the compiler, but it's normally called by running a 'cargo' command (e.g. cargo build)



This command:
```
rustup show
```
shows you your default / active toolchain.  Something like:

```
installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.37.0 (eae3437df 2019-08-13)
```

You can switch the active toolchain or compile with a specific toolchain.  Look at the 'rustup' command for details. 

#### Cool Things 

Type 'rustc', and look at the option for 'emit' that specifies the output format, including asm (think web assembly) and llvm-ir (ask Khem)

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

The most important part is the 'dependencies' section.  There are dependencies and dev-dependencies, similar to javascript-land and many other options  [Rust manifest format and details](https://rurust.github.io/cargo-docs-ru/manifest.html)


### Homework 

After you've cloned this project and created your branch (see README at the root of repository), fix the lesson code.

The objective of the homework is to build a rust project that has one function (main) call another function. You will likely hit a problem.  The problem is an important part of what Rust is all about.  For this lesson, just try to get it to work, and don't worry about trying to understand all the details. (There are many (reference types, borrowing, String vs. &str, etc.)) 

After you've fixed the code, commit and push to your remote branch.


### Homework Notes 

* We talked about 'rustc' above, but for this lesson, just use cargo build.  Cargo requires a manifest (cargo.toml), rustc does not.  

* The println! function lives in std::io [println](https://doc.rust-lang.org/std/macro.println.html) The std library contains lots of useful modules, functions, types, and macros, but is not strictly necessary. 

* Read the compile error messages.  They are helpful. 'Consider' what it recommends.
* The little '!' at the end of println is not 'not' - it means it's a [macro](https://www.siammakro.co.th/index.php).    
* There are two string types in Rust and there are [many](https://github.com/hoodie/concatenation_benchmarks-rs/blob/master/benches/lib.rs) ways to concatenate strings... stackoverflow is your friend.  Feel free to change the string type between str and String as needed.
* [Rust Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
* This function returns a value
```
fn return_something() -> i32 {
    1
} 
```
* This function does not, and fails to compile
```
fn return_something() -> i32 {
    1;
} 
```






