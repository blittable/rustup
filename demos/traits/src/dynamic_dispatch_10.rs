#[allow(dead_code)]

//Dynamic dispatch
fn main() {
    
    let mut v1 = Vec::<&dyn Get>::new();

    let aaa = &Foo(1234);
    let bbb = &Foo(4321);
    v1.push(aaa);
    v1.push(bbb);
    count_inventory(v1.iter());

    let ccc = &Bar(1, 1, 1);
    v1.push(ccc);
    count_inventory(v1.iter());

    let mut v2 = Vec::<Box<dyn Get>>::new();
    v2.push(Box::new(Foo(123)));
    v2.push(Box::new(Foo(321)));
    count_inventory(v2.iter());

    v2.push(Box::new(Bar(1, 1, 1)));
    count_inventory(v2.iter());
}

fn count_inventory<I>(it: I)
where
    I: IntoIterator,
    I::Item: Get,
{
    let sum = it.into_iter().fold(0, |acc, get| get.get() + acc);
    println!("sum: {}", sum);
}

trait Get {
    fn get(&self) -> u32;
}

impl<T: Get + ?Sized> Get for Box<T> {
    fn get(&self) -> u32 {
        (**self).get()
    }
}

impl<T: Get + ?Sized> Get for &T {
    fn get(&self) -> u32 {
        (**self).get()
    }
}

struct Foo(u32);
struct Bar(u16, u16, u16);

impl Get for Foo {
    fn get(&self) -> u32 {
        self.0
    }
}

impl Get for Bar {
    fn get(&self) -> u32 {
        self.0 as u32 + self.1 as u32 + self.2 as u32
    }
}
