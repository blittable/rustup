mod reader;
mod checker;
mod writer;

fn main() {
    let my_vec = reader::from_string("10,13,169,1337,2497,100003");
    let answer = checker::prime_checker(&my_vec);
    writer::print_answer(&answer);
}
