# Lesson Eleven: Box<T> and Option<T> Result<T, Err> (Again!) 

## Objectives 
- Drive home Option and Result Usage 
- How to use dyn and boxing 

## Result<T, E> 

```Result<T, E>``` For OO programmers, looks like like a 'parent' return type.  OO programmers can use parent objects to easily return the more specific errors in sub-class instances.  In Rust, you can get this behaviour, kind of (see below). 

But best to think about it in these terms:

1) Exactly, like Option<T>, Results are unwrapped.
2) But, reasoning through the Option tree vs. the Result tree, when we *return* an error

With Options:

                    Option<T>                                                       
                       |                                                                  
          Some         or               None                                  
            |                             | 
We know our T and get to work       Missing, handle accordingly 


But, with Results:

```
                    Result<T, E>                                                       
                       |                                                                  
           Ok          or                Err                                  
            |                             | 
We know our T and get to work       We *probably* have an Error (a trait)
                                          |
                                    We have to find the specific error because we need the Size
                                          |
                                    We return the appropriate error type and have access to the error data
```

But Wait!

I can also do this:

```rust
let x: Result<u32, &str> = Err("emergency failure");
```

and Result is frequently aliased as:

```rust
Result<T> = Result<T, SomeErrorIcannotSee>;
```


So, returning Results with errors of a known size normally triggers a hunt, when all we want to do is throw and 
handle.

## Homework:
Catchup day 2.  No new assignment.

