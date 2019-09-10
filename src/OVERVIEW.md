# The OO Perspective 

For the OO programmer, the primary differences (and therefore challenges) are the following:

- Functionality (often traits), is decoupled from data structures (structs or enums). 
- "You get what you get" - there's very little in the way of 'inherited' functionality.
- NULL is replaced by wrapping `Option<T>`
- You're accountable for the memory life of your allocations, though Rust, through compile-time rules, does most of the heavy lifting. 
- Iteration and closures are first-class citizens in code composition. 
- Error handling wraps a structure that may be of type error or not.
- Self-referential types (Parent A has a child B and they have references to one-another) require special handling

There are exceptions to most of the above.  If it sounds challenges, keep in mind that it buys us:
- A garbage-collection free lightweight runtime that is memory safe.
- Incredible performance.
- The ability to go small (embedded devices).
- Granular control over memory allocation.
- Generics
- Fearless concurrency
