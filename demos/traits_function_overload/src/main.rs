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
