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

trait InventoryItem {
    fn id(&self) -> u64;
}

enum Measurement {
    weight,
    volume,
}

struct Count<T> {
    quantity: T,
    uom: Measurement,
}

fn CountHandler() {}

trait Countable {
    fn count<T>(&self) -> Count<T>;
}

trait Item {
    fn id(&self) -> u64;
}

// impl Item {
//     fn id(&self) -> u64 {
//         2
//     }
// }

struct Dog {
    id: u64,
    name: String,
}

impl Item for Dog {
    fn id(&self) -> u64 {
        self.id
    }
}

trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

// struct printer {
//     who: dyn Item,
//     name: String,
// }

fn main() {

    //let p = printer { who: Item, name: "john".to_string()  };
    //print(Box::new(10) as Box<dyn Printable>);

    let d = Dog {
       name: "Fred".to_string(), 
    }; 

    println!("Dog id is: {:?}", d.id());
}

// struct PhysicalInventoryItem<T> {
//     id: u64,
//     quantity: T,
//     measure: Measurement,
// }

// impl<T, U> Item for PhysicalInventoryItem<T> where U:  {
//     fn id(&self) -> u64 {
//         self.id
//     }
//     fn count<U>(&self) -> Count<U> {
//         let x = 10;
//         let count = Count { quantity: &self.quantity,  uom: self.measure };
//         count
//     }
// }

// // impl Weighable for InventoryItem {
// //     fn weigh(&self) -> f64 {
// //         self.weight
// //     }
// // }

// //Count and Weigh Inventory
// //The inventory consists of weighable and non-weighable items
// //(e.g. hours of training and apples)

// fn main() {
//     // let mut v1 = Vec::<&dyn Weighable>::new();
//     // let aaa = &InventoryItem {
//     //     name: "cart",
//     //     weight: 3.33,
//     // };
//     // let bbb = &InventoryItem {
//     //     name: "cart",
//     //     weight: 3.11,
//     // };
//     // let ccc = &Box::new(&InventoryItem {
//     //     name: "cart",
//     //     weight: 3.11,
//     // });
//     // v1.push(aaa);
//     // v1.push(bbb);
//     // v1.push(ccc);
//     // sum_from_iter(v1.iter());
// }
