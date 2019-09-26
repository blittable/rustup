# Lesson Five: Error Handling 

## Objectives 

* Understand the common error handling patterns 
* Review matching in the context of error handling

## Errors:

This is an excellent article on error handling in Rust here:  [error handling](https://blog.burntsushi.net/rust-error-handling/)

The Rust Book and Rust By Example are a bit thin in this category, so the article is highly recommended.

Frankly, handling errors in Rust requires more effort than programmers from dynmaically typed languages are accustomed to. 

There are efforts like the [Failure](https://github.com/rust-lang-nursery/failure) library which attempt to make it more ergonomic/friendly to use.

But, Here are the basics using only Rust's built-in types: 

## Error Propogation 

Like `Options`, `Results` can use `.unwrap()`  As of Rust 2018, a nicer way to handle errors is to propogate via the `?` operator.

It's analogous to throwing an Exception in an OO language. If it unwraps to an `Err`, throw it up the chain. 

## The Flow of Options and Results 

```Result<T, E>```, for OO programmers, *looks like* like a 'parent' return type.  OO programmers can use parent types to easily return the more specific errors in sub-class instances.  ```Result<T, E>``` doesn't do this.   

1) Exactly, like ```Option<T>```, Results are unwrapped, typically with a ```match``` statement.
2) But, the reasoning process for Results vs. Options is not the not the same and introduces one additional step 

With Options:

```
                    Option<T>                                                       
                       |                                                                  
          Some         or               None                                  
            |                             | 
We know our T and get to work       Missing, handle accordingly 
```


But, with Results:

```
                    Result<T, E>                                                       
                       |                                                                  
           Ok          or                Err                                  
            |                             | 
We know our T and get to work       We *might* have an Error (a trait)
                                          |
                                    We find the specific error for return type size
                                          |
                                    We return Result handling the content behind Err
                                    with full knowledge of its underlying type
```


In OO programming we have the luxury of handling errors without knowing all the details, because if an error is thrown or I throw an error, we at least know it derives from an Error (typically an Exception class).

In Rust, however, we *must* know the exact content of the Result's Err in order to handle it completely.  In fact the `E` in `Result<T, E>` needn't be an `Error` (deriving from the Error trait) at all (a String here for illustrative purpose):

```rust

fn main() -> Result<(), Box<String>> {
    let function_result = is_positive(-2);
    println!("{:?}", function_result?);
    Ok(())
}

fn is_positive(number: i64) -> Result<(), String> {
    if number < 0 {
        return Err("emergency failure".to_string());
    }
    Ok(())
}
```

An additional layer of digging is often required because, aside from the above, types including
`Result<T, E>` are frequently aliased, e.g.: 
```rust,no_run
type AliasedResult<T> = Result<T, String>;
```


```rust

fn main() -> AliasedResult<()> {
    let function_result = is_positive(-2);
    println!("{:?}", function_result?);
    Ok(())
}


type AliasedResult<T> = Result<T, String>;

fn is_positive(number: i64) -> AliasedResult<()> {
    if number < 0 {
        return Err("emergency failure".to_string());
    }
    Ok(())
}

```

## Using dyn Error to wrap errors

The next challenge in Rust error handling is dealing with errors thrown by *other* libraries.  Althoug the specifics of the underlying error are unknown, if we know that the error is an Error of std::error::Error type, we can still `Box<dyn Error>` the error as shown below:


```rust, no_run
pub fn build_config(args: Vec<String>) -> Result<SuiteConfig, Box<dyn std::error::Error>> {

//.. ommitted code

            Err(e) => {
                println!("Error opening the file: {:?}", e.description());
                return Err(Box::new(e));
            }
}
```





## Summary:

1) Using the `?` makes errors easier to read and makes higher-level code flow more sensibly.
2) If you are using a Result (your own or from a function you invoke), first determine the type unwrapped by Err.
3) Make no assumptions that the Err wraps a type implementing the Error trait.
4) If you are handling other libraries' errors, you can use `Box` to wrap the libraries errors. 




