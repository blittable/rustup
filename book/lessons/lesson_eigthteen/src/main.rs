//0 -> 0b00000000
//1 -> 0b00000001
//2 -> 0b00000010
//3 -> 0b00000011

// Operators &, |, ^, !, <<, >>

fn enumerate_to_pair() {
    for (i, j) in (0..3).enumerate() {
        println!(
            "Enumerated Iterator - exclusive OR  ^ - value {:?}, loop iteration: {:?}",
            i + 1 ^ 3,
            j
        );
        println!(
            "Enumerated Iterator - inclusive OR | value {:?}, loop iteration: {:?}",
            i | 3,
            j
        );
    }
}

fn main() {
    enumerate_to_pair();
}
