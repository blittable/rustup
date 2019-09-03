pub fn to_pascal_str(input: &'static str) -> String {
    if input.len() == 0 {
        return input.to_string();
    }

    let mut i: i32 = 0;
    let mut result_string: String = String::new();
    let result_vec: Vec<String> = input
        .split_whitespace()
        .map(|s| {
            i = 0;
            s.to_string()
                .chars()
                .map(|c| {
                    i += 1;
                    if i == 1 {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_lowercase().to_string()
                    }
                })
                .collect()
        })
        .collect();

    result_vec.iter().for_each(|r| result_string.push_str(r));
    return result_string;
}
