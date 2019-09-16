pub fn from_string(input: &str) -> Vec<u32> {
    return input.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
}