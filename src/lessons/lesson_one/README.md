# Lesson One: Setup and returning a String

## Objectives 

* Get Rust installed
* Look at the toolchain (rustc, cargo, rustup)
* Create a function that returns a value


### Installing

[Installing Rust](https://www.rust-lang.org/) - This installs the toolchain (compiler and package manager (cargo) - Windows, OSX, Linux and other OS's are supported.   

* Verify your installation worked by opening a command/terminal prompt and typing:

```
rustup
rustc
cargo
```

### Get to Know the toolchain 

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

#### Cool Note  

Type ``rustc``, and look at the option for ```emit``` that specifies the output format, including asm (assembly) and llvm-ir 

```
--emit [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
                        Comma separated list of types of output for the
                        compiler to emit
```

### Tool Summary
So, there are three tools to get straight:

| Tool            | Purpose| 
| -------------   | --------------------------------------------------- | 
| ```rustup```     | Doc and toolchain related                           | 
| ```cargo```     | Rust project builder, runner, invoker sort of like ~npm    | 
| ```rustc```   | The rust compiler, though you will likely invoke it via ```cargo```|      

## Launch the Local Doc 

You're going to need it.  ;)  

```rustup doc```

## Review the Cargo.toml

Take a look at the format of the Cargo.toml.  That's the package.json / .proj  manifest file for Rust.  

The most important part is the 'dependencies' section.  There are dependencies and dev-dependencies and many other options  [Rust manifest](https://rurust.github.io/cargo-docs-ru/manifest.html) 


## Exercises 

You have some choices on runnning the exercises:
- Run inside of the doc via playpen (look for the play button)
- Clone or fork this repository and navigate to the lesson directory and run with `cargo run` 
- Brew your own repo, optionally copying-pasting code


| Exercise| Purpose| 
| -------------   | --------------------------------------------------- | 
| [Calling a Function](ex_function_parameter_return.md)     | Kicking the Tires | 
