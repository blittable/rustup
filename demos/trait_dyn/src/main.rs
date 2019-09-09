#![allow(unused_variables)]
fn main() {
}

trait TaxCalculator {
    fn calcTax(amount: i32) -> i32;
}

const THAILAND_VAT: f32 = 0.07;

struct Thailand {}
struct USA {}

impl TaxCalculator for Thailand {
    fn calcTax(amount: i32) -> i32
    {
        amount * THAILAND_VAT
    }
}

fn GetTaxCalculator() -> Box<dyn TaxCalculator> {
    let calculator = Thailand;
    Box::new(calculator)
}


