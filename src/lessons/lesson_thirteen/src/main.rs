fn main() {}

//Basic struct
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//Exactly the same. Basic struct, but with 'Self' as return type
struct SelfishCounter {
    count: u32,
}

impl SelfishCounter {
    fn new() -> Self {
        SelfishCounter { count: 0 }
    }
}

struct ABitSelfishCounter {
    count: u32,
}

//Using the 'Self' constructor
impl ABitSelfishCounter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

#[test]
fn counter_test() {
    let counter = Counter::new();
    assert_eq!(counter.count, 0);
}

#[test]
fn selfish_counter_test() {
    let selfish_counter = SelfishCounter::new();
    assert_eq!(selfish_counter.count, 0);
}

#[test]
fn a_bit_selfish_counter_test() {
    let a_bit_selfish_counter = ABitSelfishCounter::new();
    assert_eq!(a_bit_selfish_counter.count, 0);
}

///Fingerprint tuple struct
pub struct Fingerprint(u64, u64);

impl Fingerprint {
    pub const ZERO: Fingerprint = Fingerprint(0, 0);
    pub fn hi() {}
}

#[test]
fn fingerprint_test() {
    let fp = Fingerprint(7, 7);
    assert_eq!(fp.0, 7);

    let zeroed = Fingerprint::ZERO;
    assert_ne!(zeroed.0, fp.0);
}
