fn main() {
    let usa = Country {
        name: "USA",
        taxer: Taxer::CompoundTaxer {
            rate: 2.3,
            compound_rate: 3.3,
        },
    };

    println!("{}", usa.taxer.calculate_tax(333.0));
}

struct Country {
    pub name: &'static str,
    pub taxer: Taxer,
}

enum Taxer {
    Simple { rate: f32 },
    CompoundTaxer { rate: f32, compound_rate: f32 },
}

impl Taxer {
    fn calculate_tax(&self, amount: f32) -> f32 {
        match *self {
            Self::Simple { rate } => amount * rate,
            Self::CompoundTaxer {
                rate,
                compound_rate,
            } => amount * rate + amount * compound_rate,
        }
    }
}
