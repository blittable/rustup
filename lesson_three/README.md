# Lesson Three: Pay for What You Use, Introduction to Traits and Enum Wrappers 

## Objectives 

* Look back at Lesson 2 and to_owned() and clone()
* Understand types and what they implement - Traits 

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

```rust
trait Pay {
  fn pay() -> i32; 
}

struct Company {
  name: String
}

impl Pay for Company {
  fn pay(&self) -> i32 {
    10
  }
}
```

## Homework 

The instructions are in the code (lesson 3) - implement a trait - feel free to do more!







