# Lesson Five: Introduction to Closures 

## Objectives 

* Understand when and how to use a closure 
* Look at the traits Fn, FnOnce, and FnMut 

## Lesson:


Comparing Funtions and Closures:

```
	fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
	let plus_one_v2 = |x: i32| -> i32 { x + 1 };
	let plus_one_v3 = |x: i32|  x + 1; };
```



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






