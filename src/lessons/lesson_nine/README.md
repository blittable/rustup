# Lesson Nine: Lifetime Annotations 

## Objectives 
- Understand the basics of Lifetime Annotations

## The problem

Java, .NET, Go and others use garbage collection to manage the 'generations' of object references and release them.  

Rust does not use a garbage collection process. Consequently, memory utilization is extremely efficient.  One of the trade-off's, however, is that we sometimes need to explictly provide instructions regarding how long objects are intended to live.   

Look at the code below.  We pass our &str's to a function, that does an evaluation, and then returns one of the str's
After the function returns, the parameter &str's, go out of scope.  

The borrow-checker efficiently wants to drop the references at the end of the scoped block, *but* the returned value is referencing one of them.  

The compiler needs explicit instructions.  This is done with a special 'marker' called the 'lifetime annotation.'  

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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // <- without 'a this fails.
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

So, 'they' dropped the cases where it was obvious and gave the heruistics a French name, 'lifetime elision rules' so no one would ask anymore questions. 

## Exercise 

Create a binary tree datastructure.  

Each node may have a parent, a node to the left, and a node to the right.  

Add one function/method that allows insertion of a node.

You will almost certainly need to use lifetime annotations and possibly Box<T>.  If you cannot complete the exercise, have no fear and keep moving forward.

#### Related Fun:

- A pretty node-free Graph implementation
[Contest Algorithms, Graph:](https://github.com/EbTech/rust-algorithms/blob/master/src/graph/mod.rs)