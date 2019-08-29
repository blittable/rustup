#![allow(unused_variables)]

fn main() {

    let nums: [i32; 3] = [1, 2, 3];
    let nums: [i32; 3] = [1, 2, 340404404044];

    let result = double_some_values(&nums);

    if !result.is_empty()
    {
        println!("Here are your doubled numbers : {:?}", result)
    }
    else
    {
        println!("no results were returned")
    }
}

//TODO: This function has a *serious* bug
//The requirement is to take the array of 3 nums
//and return the array with all of the values doubled 
//So, input => [1,2,3] returns [2,4,6]
fn double_some_values(nums: &[i32; 3]) -> Vec<i32>  {
    nums.iter().map(|&n| n).collect()
}
