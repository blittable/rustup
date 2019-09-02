# Lesson Twelve:  Midterm!, Trait Bounds, and ซื่อๆ DB

## Objectives 
- Debug with output
- Trait Bounds
- Week's Project


### Running Unit Tests with Output

If you use output in unit tests, like println!(), the output is hidden unless you add --no-capture

```
cargo test -- --nocapture
```

### Trait Bounds

As an OO programmer, you already know generics.  It's quite similar in Rust. But, what about Traits?

You can pass a trait `impl` as a parameter:

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

But, what about traits as *generics*?


In the code below, the Rust toolchain has *no* information on the types functions, etc. and will not compile. 

```rust
pub fn notify<T>(item: T) {
    println!("Breaking news! {}", item.summarize()); //<- what's a summarize?
}
```

To solve the issue in the context of generics, Rust uses `trait bounds` 
```rust
pub fn notify<T: Summary>(item: T) {
    ...
}
```

which is the same as:
```rust
pub fn notify(item: impl Summary) {
    ...
}
```

which is the same as:
```rust
pub fn notify<T>(item: T) 
where T: Summary {
    ...
}
```

which can be extended, applying other traits with:

```rust
pub fn notify<T: Summary + Display>(item: T) 
where T: Summary + Display {
    ...
}
```



## This Week's Project

There will be no homework this week.  Classtime will be used to for the project.


We will split into teams and create a database.

Mate, Nook, Aong => Clap Team
Kanoon, Xenirio, Kim => Architecture Team
Toom, Neng, Khem => Persistence Team
Kevin, Uthen => Performance / Criterion Team
John, Vincent => Marketing
```

# Requirements
- Implement a cli-based database that supports, "Upsert" and "Get" operations.
- The database should store the values on the file system

These operations (or something close) need to be supported:

```
susu add key="susan_salary" value="1200"
susu get key="susan_salary"
```

# Clap Team
Look at Clap. Clap can get complicated.  Keep it simple. [Clap Crate on crates.io](https://crates.io/crates/clap)   

# Persistence Team
How do we store and retrieve the data?

# Architecture Team
Keep it simple.

# CI/CD
Keep it simple, we need a shared repository


# Marketing

# **"ซื่อๆ DB, Saving Your Data... Usually"** 

![DB Logo](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSpXHfXT9o3HfiKSg8n4tZxvEgLhsI8mbK3R7aVkVpnNKZaUojP)



