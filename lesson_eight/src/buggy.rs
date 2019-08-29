fn main() {
    // Covers both ends of the i32 range; from 0 to std::i32::MAX (2147483647) including 
    // values just below (1073741823) and above (1073741824) half of std::i32::MAX.
    let nums: [i32; 6] = [0, 1, 2, 1073741823, 1073741824, std::i32::MAX] ;

    // Option 1: Result is Vec<i64>
    let result_guaranteed = double_some_values_guaranteed(&nums);
    if !result_guaranteed.is_empty() {
        println!("Here are your guaranteed doubled numbers: {:?}", result_guaranteed);
    }
    else {
        println!("No results were returned.");
    }

    // Option 2: Result is Vec<Option<i32>>
    let result_with_option = double_some_values_with_option(&nums);
    if !result_with_option.is_empty() {
        print!("Here are your possibly doubled numbers: [");
        for i in 0..result_with_option.len() {
            match result_with_option[i] {
                None => print!("TOO_BIG"),
                Some(v) => print!("{}", v),
            }
            if i < result_with_option.len() - 1 {
                print!(", ");
            }
        }
        print!("]\n\r");
    }
    else {
        println!("No results were returned.");
    }

    // Option 3: Return and empty vector.
    let result_empty = double_some_values_empty_result(&nums);
    if !result_empty.is_empty() {
        println!("Here are your doubled numbers: {:?}", result_empty);
    }
    else {
        println!("No results were returned.");
    }
}

// Returns Vec<i64> instead of Vec<i32>, to ensure that the 'attempt to multiply with overflow' error 
// does not occur - as doubling any number within the range of i32 will not overflow i64.
fn double_some_values_guaranteed(nums: &[i32; 6]) -> Vec<i64> {
    // Cast each value to i64 before doubling it.
    nums.iter().map(|&n| n as i64 * 2).collect()
}

// Returns Vec<Option<i32>>, where any values that would overflow i32 return None (to be 
// handled by the calling code.) This makes use of i32's checked_mul method. 
// See https://doc.rust-lang.org/std/primitive.i32.html#method.checked_mul
fn double_some_values_with_option(nums: &[i32; 6]) -> Vec<Option<i32>> {
    nums.iter().map(|&n| n.checked_mul(2)).collect()
}

// Returns an empty Vec<i32> if any of the input values were to cause an overflow of i32 if doubled.
fn double_some_values_empty_result(nums: &[i32; 6]) -> Vec<i32> {
    for num in nums {
        // Return en empty Vec<i32> since a value that is greater than half of std::i32::MAX will cause 
        // an overflow of i32 when doubled.
        if num > &(std::i32::MAX / 2) {
            return Vec::new()
        }
    }

    nums.iter().map(|&n| n * 2).collect()
}
