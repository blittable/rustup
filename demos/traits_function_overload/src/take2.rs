fn main() {
    let usa = Country {
        name: "USA",
        taxer: Box::new(CompoundTaxer {
            base: SimpleTaxer {
                rate: 2.3,
            },
            compound_rate: 3.3,
        }),
    };

    println!("{}", usa.taxer.calculate_tax(333.0));
}

struct Country {
    pub name: &'static str,
    pub taxer: Box<dyn Taxer>,
}

trait Taxer {
    fn calculate_tax(&self, amount: f32) -> f32;
}

struct SimpleTaxer {
    rate: f32,
}

impl Taxer for SimpleTaxer {
    fn calculate_tax(&self, amount: f32) -> f32 {
        amount * self.rate
    }
}

struct CompoundTaxer {
    base: SimpleTaxer,
    compound_rate: f32,
}

impl Taxer for CompoundTaxer {
    fn calculate_tax(&self, amount: f32) -> f32 {
        let base_tax = &self.base.calculate_tax(amount);
        base_tax + amount * self.compound_rate
    }
}