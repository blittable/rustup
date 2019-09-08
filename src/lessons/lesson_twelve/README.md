# Lesson Twelve:  Trait Bounds 

## Objectives 
- Trait Bounds Basics

### Trait Bounds

As an OO programmer, you already know generics (hopefully!).  It's quite similar in Rust. But, what about 'traits'?

You can pass a trait `impl` as a parameter:

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

But, what about traits as *generics*?  Perhaps:


```rust
pub fn notify<T>(item: T) {
    println!("Breaking news! {}", item.summarize()); //<- what's a summarize?
}
```

In the code below, the Rust toolchain has *no* information on the types' functions, etc. and will not compile. It needs more information. 


To solve the issue in the context of generics, Rust uses `trait bounds` 

```rust
pub fn notify<T: Summary>(item: T) {
    ...
}
```

which is the same as:
```rust
pub fn notify(item: impl Summary) {
}
```

For better clarity, this can also be written as:

```rust
pub fn notify<T>(item: T) 
where T: Summary {
    ...
}
```

And, applied across multiple traits with:

```rust
pub fn notify<T: Summary + Display>(item: T) 
where T: Summary + Display {
    ...
}
```

Read the above, "the notify function is bound by the behavior of the Summary and Display traits."


## This Week's Project

There will be no homework this week.  Class with cover one quick topic (5 minutes) and the remainder will be focused on the project effort.


We will split into teams and create a database.

Mate, Nook, Aong => Clap Team
Kanoon, Xenirio, Kim => Architecture Team
Toom, Neng, Khem => Persistence Team
Kevin, Uthen, Khwan => Performance / Criterion Team
John, Vincent => Marketing
```

### Project Structure

For larger projects, a parent Cargo.toml can reference N other projects via the 'members' 
syntax.  See the following project as an example:
[members cargo syntax](https://github.com/crypto-com/chain)



### Requirements
- Implement a cli-based database that supports, "Upsert" and "Get" operations.
- The database should store the values on the file system

These operations (or something close) need to be supported:

```
susu add key="susan_salary" value="1200"
susu get key="susan_salary"
```

# Clap Team (This is not a cheerleading team: clap: Command Line Argument Parser)
Look at Clap. Clap can get complicated.  Keep it simple. [Clap Crate on crates.io](https://crates.io/crates/clap)   

# Persistence Team
How do we store and retrieve the data?

# Architecture Team
Keep it simple.

# Criterion Team 
Compare ซื่อๆ DB with something (RocksDB?) 


