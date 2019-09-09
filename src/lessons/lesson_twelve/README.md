# Lesson Twelve:  Trait Bounds 

## Objectives 
- Trait Bounds Basics

### Trait Bounds

As an OO programmer, you already know generics (hopefully!).  It's quite similar in Rust. 


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


### Function Overloading -ish

As an OO programmer, note that no-inhertiance != no-indirection:

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

### dyn Keword

* As a side note, if the content here, the API doc, and 'The Book' still leave you unclear on a topic,
try [The Reference](https://doc.rust-lang.org/stable/reference/) Some of the explanations are excellent.  Remarkably
similar to some of the content here.  ;)

Trait objects are dynamically sized and are behind a reference of some type.  

Traits used as return types need to be marked with ```dyn```


### Exercise

In the below code, using a trait, create a converter between Kilos, Pounds, and Stone

{{#playpen src/main.rs}}


