# The OO Perspective 

For the OO programmer:

- Functionality (often traits), is decoupled from data structures (structs or enums). 
- "You get what you get" - there's very little in the way of 'inherited' functionality.
- There is no NULL.  It is replaced, by convention, by wrapping `None` in an `Option<T>`
- You're accountable for the memory life of your allocations, though Rust, through compile-time rules, assists, vigorously. 
- Iteration and closures are first-class citizens in code composition. 
- The allocation-related rules above have significant consequences, especially for recursive-types (e.g. graph nodes), associated types, and thread-safe types. 


- A garbage-collection free, lightweight runtime that is memory safe.
- Incredible performance.
- Generics
- Fearless concurrency
