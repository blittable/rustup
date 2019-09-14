
# Lesson Sixteen: Rc

## Objectives 

- Look at Rc, and RefCell 

Consider the following scenario:

A family of five has two parents and three grown-children.  
The grown-children all drive. 

The family has 3 cars: a Subaru, a Ferrari, and a Daihatsu Mira.

All members of the family can drive the Mira.
All members of the family can drive the Subaru.
Only the parents can drive the Ferrari.


| Option                            | Details       | Issue             | 
| --------------------------------- | ------------- | ----------------- |
| Clone                             | Makes a Copy  | Lose shared state |
| Borrow with Lifetime Annotations  | &'a structs   | None              |
| Rc<T>                             |               |                   |



## Solution One: Lifetime Annoations


## Solution Two: Rc<T> Reference Counting  


The type Rc<T> provides shared ownership of a value of type T

{#playpen src/main.rs}}


Exercise

- Create a sample that mutates the contents of something behind a Rc<T>