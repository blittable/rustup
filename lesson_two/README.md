# Lesson Two: Variables and Immutability 

## Objectives 


* Rationale for the design
* Understand immutability and basic borrowing rules 

## Lesson:


Race Conditions and Why Rust Helps:

Let's say we have a Point of Sales system, and there are many run-time configuration options/rules.  Developer A writes some code and then updates those options.  Developer B, in a different module, does the same.  *When* those updating events are called, at runtime, is unknown, as it depends on one (or more users). That's a basic race condition.  


'Borrowing' in Rust has basically two rules:

1) &MyVariable can be borrowed by multiple borrowers.
2) &mut MyVariable can be borrowed by *one* borrower.



### Task:  


### Homework 



### Homework Notes 






