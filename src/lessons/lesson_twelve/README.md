# Lesson Twelve:  Trait Bounds 

## Objectives 
- Trait Bounds Basics

### Trait Bounds

As an OO programmer, you already know generics (hopefully!).  It's quite similar in Rust. 


```c#
  class Program
    {
        static void Main(string[] args)
        {
            //Covariant 
            IEnumerable<Base> d = new List<Derive>();
        }
    }

    interface Base { };
    class Derive : Base {
        public Derive(){}
    }
```


But, what about 'traits'?

You can pass a trait `impl` as a parameter:

```rust, no_run
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

But, what about traits as *generics*?  Perhaps:


```rust, no_run
pub fn notify<T>(item: T) {
    println!("Breaking news! {}", item.summarize()); //<- what's a summarize?
}
```

In the code below, the Rust toolchain has *no* information on the types' functions, etc. and will not compile. It needs more information. 


To solve the issue in the context of generics, Rust uses `trait bounds` 

```rust, no_run
pub fn notify<T: Summary>(item: T) {
    ...
}
```

which is the same as:
```rust, no_run
pub fn notify(item: impl Summary) {
}
```

For better clarity, this can also be written as:

```rust, no_run
pub fn notify<T>(item: T) 
where T: Summary {
    ...
}
```

And, applied across multiple traits with:

```rust,no_run
pub fn notify<T: Summary + Display>(item: T) 
where T: Summary + Display {
    ...
}
```

Read the above, "the notify function is bound by the behavior of the Summary and Display traits."

