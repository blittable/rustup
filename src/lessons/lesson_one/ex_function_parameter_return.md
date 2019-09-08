## Calling a Function

{{#playpen src/main.rs}}

Try running this with the play button above.

#### Code Dissection  

Two elements jump out here: ```println!``` and ```&str```:

One-by-one: 

The `println!`
```rust, no_run
println!(function_result);
```

The println! 'function' lives in [std::io::println](https://doc.rust-lang.org/std/macro.println.html) But, it's not a function, it's a macro. 

The little '!' at the end of println is not 'not.' It means it's a macro. 

A macro is a meta-code generation feature that lets macro authors expand simple syntax (hopefully), into something that would otherwise be unduly complex or repetitive. 

A macro author writes the rules (the macro) for the expansion.  

You needn't author macros to code in Rust, but you will use them.

<br>

```rust, no_run
fn greetings(name: &str) -> String 
```

The `&str`
* There are two string types in Rust. [Rust Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
* The ```&str``` type is a string slice.  It's a pointer to some UTF-8 bytes, so we know it's length, precisely.
* The ```String``` type is dynamically sized, and is conceptually close to a Java or .NET String class.

The challenge is that we frequently move back and forth between the two.  

The secret-sauce of Rust, compile-time checks, requires that all types allocated have a known-size.  Consequently, many of the operations on strings are done on the ```&str``` type and there are reasons (forthcoming) to prefer the &str as a parameter. 

So, in the code:
- ```name``` is actually a string slice `&str` and is passed accordingly to the ```greetings``` function
- The `to_string` function makes our `&str` a String.  (Think to_String() with a big S). 
- Our concatenation operator `+` takes a `&str` and a `String` and returns a `String`.
- That String is returned and used as the parameter to the println! macro. 

Again:

{{#playpen src/main.rs}}


### Implicit Return
The `return` keyword is not specified and `;` is not required in this case. 

This works: 
```rust, no_run
fn return_something() -> i32 {
    1
} 
```
But, this fails to compile (but would with a `return 1;` statement)
```rust, no_run
fn return_something() -> i32 {
    1;
} 
```

#### Task

Play around with the code block.  Don't worry if you hit errors at this point.








