fn sum_from_iter<I>(it: I)
where
    I: IntoIterator,
    I::Item: Weighable,
{
    let sum = it.into_iter().fold(0.0, |acc, x| x.weigh() + acc);
    println!("sum: {}", sum);
}


trait Weighable {
    fn weigh(&self) -> f64;
}

impl<T: Weighable> Weighable for Box<T> {
    fn weigh(&self) -> f64 {
        (**self).weigh()
    }
}

impl<T: Weighable + ?Sized> Weighable for &T {
    fn weigh(&self) -> f64 {
        (**self).weigh()
    }
}

struct InventoryItem {
    weight: f64,
    name: &'static str,
}

impl Weighable for InventoryItem {
    fn weigh(&self) -> f64 {
        self.weight
    }
}

fn main() {
    let mut v1 = Vec::<&dyn Weighable>::new();
    let aaa = &InventoryItem { name: "cart",  weight: 3.33 }; 
    let bbb = &InventoryItem { name: "cart", weight: 3.11 }; 
    let ccc = &Box::new(&InventoryItem { name: "cart", weight: 3.11 }); 
    
    v1.push(aaa);
    v1.push(bbb);
    v1.push(ccc);
    sum_from_iter(v1.iter());

}