
struct Counter {
    count: u32,
}


// 3 identical ways to 'new up' a Counter
impl Counter {
    fn new() -> Counter {
        Counter { count: 8 }
    }
    fn new_self() -> Self {
        Self { count: 8 }
    }
    fn new_self_named_return() -> Counter {
        Self { count: 8 }
    }
}

// Creating a generic counter
struct CounterGeneric<T> {
    count: T,
}

impl<T> CounterGeneric<T> {
    fn new(t: T) -> CounterGeneric<T> {
        CounterGeneric { count: t }
    }
}

fn main() {
    let counter_0 = Counter::new();
    assert_eq!(counter_0.count, 8);

    let counter_1 = Counter::new_self();
    assert_eq!(counter_1.count, 8);

    let counter_2 = Counter::new_self_named_return();
    assert_eq!(counter_2.count, 8);

    let counter_3 = CounterGeneric::new(8);
    assert_eq!(counter_3.count, 8);

    let counter_4_u32 = CounterGeneric::new(8_u32);
    assert_eq!(counter_4_u32.count, 8);

    let counter_5_u64 = CounterGeneric::new(8_u64);
    assert_eq!(counter_5_u64.count, 8);


    //in-line sample 
    // struct InventoryItem {
    //     name: &'static str,
    // }

    // let aaa = &InventoryItem { name: "cart" }; 

    println!("If we ran to here without an error in the assets, it was successful.");
}
