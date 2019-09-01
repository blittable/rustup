fn main() {
    println!("Hello, world!");
}

mod test {
    use std::mem;

    //MID-TERM!
    #[test]
    fn size_test() {
        let slice = "C";
        let word = "Cat";
        let words = "Cats eat dog food at the sea when the weather is good, but should it turn foul, they turn to the finer indoor delicacies.";

        println!("slice char size: {:?}", &slice_size);

        let slice_size = mem::size_of_val(&slice);
        let word_size = mem::size_of_val(&word);
        let sentence_size = mem::size_of_val(&words);

        assert!((slice_size == word_size) && (word_size == sentence_size));

    }
}
