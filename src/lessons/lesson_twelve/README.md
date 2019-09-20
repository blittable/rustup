# Lesson Twelve:  Trait Bounds 

## Objectives 
- Trait Bounds Basics

________
### Trait Bounds

As an OO programmer, you are likely already familiar with generics. 

In Rust there are similarities and differences.



First, it's important to get clear on some terminology.

```rust
fn apply<F, T>(f: F, t: T)
where
    F: FnOnce(),
{
    f();

    do_something(t);
}

fn do_something<T>(t: T) {
    let z: Box<T> = Box::new(t);
}
```

In the code above, 

`fn apply<F, T>(f: F, t: T) where F: FnOnce()`

is the entire type signature.

`<F, T>` specifies two type parameters.  We know nothing about them, other than they are types.

`where F: FnOnce()` is a trait bound.  It's a constrait that requires any generic type used in the function to implement the FnOnce() trait.



Additionally:


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

In the code above, the Rust toolchain has *no* information on the types' functions, etc. and will not compile. It needs more information. 


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

### Key Points on Traits
* Trait objects are dynamically sized and are behind a reference of some type.  
* Consequently, and unlike generics and plain functions/methods, they are dynamically dispatched at runtime. 

### Function Overloading-ish

As an OO programmer, note that no-inhertiance != no-indirection.  The ```SimpleTaxer``` resolves the appropriate calculation method.

```rust
fn main() {
    let usa = USA {
        tax_calculator: Box::new(SimpleTaxer {}),
    };

    let thailand = Thailand {
        tax_calculator: Box::new(SimpleTaxer {}),
    };

    println!(
        "USA tax: {}",
        usa.tax_calculator.compound_tax(333.0, 2.3, 3.3)
    );
    println!(
        "Thailand tax: {}",
        thailand.tax_calculator.calculate_tax(333.0, 2.3)
    );
}

trait SimpleTax {
    fn calculate_tax(&self, amount: f32, rate: f32) -> f32;
}

trait CompoundTax: SimpleTax {
    fn compound_tax(&self, amount: f32, base_rate: f32, rate: f32) -> f32;
}

struct USA {
    pub tax_calculator: Box<dyn CompoundTax>,
}

struct Thailand {
    pub tax_calculator: Box<dyn SimpleTax>,
}

struct SimpleTaxer {}

impl SimpleTax for SimpleTaxer {
    fn calculate_tax(&self, amount: f32, rate: f32) -> f32 {
        amount * rate
    }
}
impl CompoundTax for SimpleTaxer {
    fn compound_tax(&self, amount: f32, base_rate: f32, compound_rate: f32) -> f32 {
        let base_tax = &self.calculate_tax(amount, base_rate);
        base_tax + amount * compound_rate
    }
}
```

* As a side note, if the content here, the API doc, and 'The Book' still leave you unclear on a topic,
try [The Reference](https://doc.rust-lang.org/stable/reference/) Some of the explanations are excellent.  Remarkably
similar to some of the content here.  ;)


### Exercise

In the below below, using a trait, create a converter between Kilos, Pounds, and Stone

{{#playpen src/main.rs}}


