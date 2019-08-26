pub fn get_max_score(children: Vec<(i32, String, f32)>) -> (i32, String, f32){
    let mut max_value: f32 = 0.0;
    for i in children.iter() {
        if i.2 > max_value {
            max_value = i.2
        }
    }
    for i in children.iter() {
        if i.2 == max_value {
            return i.clone();
        }
    }
    (0, "".to_string(), 0.0)
}