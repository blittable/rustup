// //Lesson Five: Introduction to Closures
// //https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html#closures

type Doubledtype = Vec<i32>;

//Fn
fn doubledtype_vec_closure_fn<F>(f: F, array: [i32; 3]) -> Doubledtype
where F: Fn([i32; 3]) -> Doubledtype,
{
    f(array)
}

//FnMut
fn doubledtype_vec_closure_fn_mut<F>(mut f: F)
where F: FnMut(),
{
    f();
    //f();
}

//FnOne
fn doubledtype_vec_closure_fn_mut_one<F>(f: F, array: [i32; 3]) -> Doubledtype
where F: FnOnce([i32; 3]) -> Doubledtype,
{
    f(array)
}

fn main() {
    println!("Hello, Closure!");    
    let a = [1, 2, 3];    

    //Fn
    let func_answer = |num: [i32; 3]| -> Vec<i32> { num.iter().map(|&x| x * 2).collect()};            
    let doubled = doubledtype_vec_closure_fn(func_answer, a);
    assert_eq!(vec![2, 4, 6], doubled);
    println!("Closure implementation by Fn: {:?}", doubled);
        
    //FnMut 
    let mut slice = a.clone();
    let doubled2 = || { slice.iter_mut().for_each(|x| *x *= 2); };
    doubledtype_vec_closure_fn_mut(doubled2);                        
    assert_eq!(vec![2, 4, 6], slice);
    println!("Closure implementation by FnMut: {:?}", slice);

    //FnOne                
    let slice_fn_one = a.clone();
    let doubled3 = doubledtype_vec_closure_fn_mut_one(|num: [i32; 3]| -> Vec<i32> { num.iter().map(|&x| x * 2).collect() }, slice_fn_one,);
    assert_eq!(vec![2, 4, 6], doubled3);
    println!("Closure implementation by FnOnce: {:?}", doubled3);
}