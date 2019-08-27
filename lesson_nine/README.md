# Lesson Nine: Lifetime Annotations 

## Objectives 
- Understand and use lifetime annotations

## The problem - For Discussion 

```rust

fn main() {

    let target: &str;

    //Narrow our scope
    {
        let string1 = "abcd";
        let string2 = "xyzwkrp";

        target = longest(string1, string2);

        println!("The longest string is {}", target);
    } // <- goodbye string1, string2... I think...

    println!("The longest string is {}", target);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```
Actually, Rust requires lifetimes on all parameter and return value references, but it became a pain to type:
```
fn longest(x: &'a str, y: &'a str) -> &'a str ... 
```

So, they dropped the cases where it was obvious and gave the heruistics a French name, 'lifetime elision rules' so no one would ask anymore questions. 

## Homework 


