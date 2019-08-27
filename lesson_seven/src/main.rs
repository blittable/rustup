mod vecs;
mod tuples;

fn main() {
    let my_vecs: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    vecs::print_vec(my_vecs);

    let my_tuple = (1,"ASDF".to_string());
    tuples::print_tuple(my_tuple);
}