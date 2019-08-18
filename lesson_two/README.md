# Lesson Two: Variables and Immutability 

## Objectives 


* Rationale for the design
* Understand immutability and borrowing rules 

## Lesson:


Race Conditions and Why Rust Helps:

Let's say we have a Point of Sales system, and there are many run-time configuration options/rules.  Developer A writes some code and then updates those options.  Developer B, in a different module, does the same.  *When* those updating events are called, at runtime, is unknown, as it depends on one (or more users). That's a basic race condition.  


'Borrowing' in Rust has basically two rules:

1) &MyVariable can be borrowed by multiple borrowers.
2) &mut MyVariable can be borrowed by *one* borrower.





### Task:  

```
fn main() {let x = 4;    let equal_to_x = |z| z == x;
    let y = 4;    assert!(equal_to_x(y));}
}
```

```
let x = 5;
let square_x = move || x * x;
assert_eq!(square_x(), 25);
```

[Rust Closures](https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html) 

### Homework 



### Homework Notes 






