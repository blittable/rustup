# Lesson Three: Pay for What You Use, Introduction to Traits and Enum Wrappers 

## Objectives 

* Understand types and what they implement - Traits 
* Understand working without NULL (you won't miss it) and the std enum wrappers Option<T> Result<Err, T> 

## Pay for What you Use 

One of the philosophies of Rust is 'Pay only for what you use'.  That means means many types have 'limited' functionality
to keep them efficient. Some of the 'basic' functions you might expect to be there, will not be! 

The functionality that is on a type is normally available from a ```trait```.

In an OO language like .NET, the root object ```Object``` implements ```Object.MemberwiseClone``` and is accessible to all classes/objects.

Not so in Rust:

```rust
  let name: &'static str = "Mycos";

    let mut name_borrower = String::default();
    let name: String = String::from("Mycos");

    name_borrower = name.to_owned();    <--- Our String can do to_owned 
    name_borrower = name.clone();       <--- AND clone() 

    //BUT, not all types can be .clone() 'd
    //Not all types implement the trait
    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();
    let vv: Vec<i32> = v.clone()        <---  Compile Error, the clone trait is not implemented
```

## Simple Traits (demos / traits_101)







```


## Homework 
Playtime!  Lesson 2 src is empty.  Try working with stack and heap assigned variables. Try to break the rules and review the compiler messages.  






