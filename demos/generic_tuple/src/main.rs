pub trait Runner {}

pub struct DecathlonEntrant<A>(pub A)
where
    A: Runner;

fn main() {
    let entrant_number_one = DecathlonEntrant("Time");
}
