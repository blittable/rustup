## Exercise: Calling a Function


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






