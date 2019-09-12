
# Lesson Sixteen: Serde, Rc, RefCell 

## Objectives 

- Look at Serde, Rc, and RefCell 

[Serde Sample](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=72755f28f99afc95e01d63174b28c1f5)


| Option                            | Details       | Issue             | 
| --------------------------------- | ------------- | ----------------- |
| Clone                             | Makes a Copy  | Lose shared state |
| Borrow with Lifetime Annotations  | &'a structs   | None              |
| Rc<T>                             |               |                   |


### Rc<T>

```rust
  --> src/main.rs:47:26
   |
36 |     let house_d = House::new(2, "Douglas".to_string());
   |         ------- move occurs because `house_d` has type `House`, which does not implement the `Copy` trait
...
44 |     street_0.houses.push(house_d);
   |                          ------- value moved here
...
47 |     street_1.houses.push(house_d);
   |                          ^^^^^^^ value used here after move

```

## Solution One: Lifetime Annoations


## Solution Two: Rc<T> Reference Counting  


The type Rc<T> provides shared ownership of a value of type T

{#playpen src/main.rs}}


Exercise

- Create a sample that mutates the contents of something behind a Rc<T>