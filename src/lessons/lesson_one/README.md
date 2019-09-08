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

Type ``rustc``, and look at the option for ```emit``` that specifies the output format, including asm (think web assembly) and llvm-ir 

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
| [Calling a Function](./ex_function_parameter_return.md)     | Doc and toolchain related                           | 
1) [Calling A Function](ex_function_parameter_return)

### A Function, A Parameter, and a Return Type 

{{#playpen src/main.rs}}
Or, if you need a bit more real-estate: [Playpen 1](https://play.rust-lang.org/?gist=417001759b501f8058ebb10c7404101d)

#### Code Dissection  

```rust
println!("{}", function_result);
```
* The println! function lives in [std::io::println](https://doc.rust-lang.org/std/macro.println.html) The std library contains lots of useful modules, functions, types, and macros, but is not strictly necessary. 


```rust
fn greetings(name: &str) -> String 
```

{{#playpen src/macro_syntax.rs}}

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






