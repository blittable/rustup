# Lesson Two: Variables and Immutability 

## Objectives 

* Understand basic variable assignments, types and fundamental borrowing rules 

## Lesson:

Race Conditions and Why Rust Helps:

Let's say we have a Point of Sales system, and there are many run-time configuration options/rules.  A developer writes some code and then updates those options.  Developer B, in a different module, does the same.  When those updating events are called, at runtime, is unknown, as it depends on the system users. That's a basic race condition.  

Rust has an important rule:

1) ```MyVariable``` can be borrowed by multiple borrowers.
2) ```mut MyVariable``` can be borrowed by *one* borrower.




## Task:

This section of chapter 4 is great.  Highly recommended:
[The stack, heap, ownership and borrowing](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)









### Task:  


### Homework 



### Homework Notes 






