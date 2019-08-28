#![allow(unused_variables)]

fn main() {
    {
        let nums: [i32; 3] = [1, 2, 3];

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

    {
        let nums: [i32; 3] = [1, 2, 2147483647];

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
    
}

//TODO: This function has a *serious* bug
//The requirement is to take the array of 3 nums
//and return the array with all of the values doubled 
//So, input => [1,2,3] returns [2,4,6]
fn double_some_values(nums: &[i32; 3]) -> Vec<i32>  {
    let max_value_at_half :i32 = 1073741823;
    let min_value_at_half :i32 = -1073741824;
    
    let filtered : Vec<i32> = nums.iter()
        .filter(|&x| x >= &min_value_at_half && x <= &max_value_at_half)
        .map(|&x| x)
     .collect();
    
    match filtered.iter().count(){
        3 =>
        {
            return filtered.iter().map(|&n| n * 2).collect();
        }
        _ =>
        {
            return vec![];
        }
    }
}
