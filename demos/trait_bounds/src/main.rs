fn main() {
    println!("Good luck!");
}

#[allow(unused_imports)]
mod test {
    use std::mem;

    //MID-TERM!
    #[test]
    fn size_test() {
        let slice = "C";
        let word = "Cat";
        let words = "Cats eat dog food at the sea when the weather is good, but should it turn foul, they turn to the finer indoor delicacies.";

        let slice_size_demo = mem::size_of_val(&slice);
        println!("slice for \"C\" size: {:?}", &slice_size_demo);

        //mem::size_of_val
        //signature: pub fn size_of_val<T: ?Sized>(val: &T) -> usize
        //doc: Returns the size of the pointed-to value in bytes.
        let slice_size = mem::size_of_val(&slice);
        let word_size = mem::size_of_val(&word);
        let sentence_size = mem::size_of_val(&words);

        //Question One: Does this assert pass?
        assert!(slice_size == word_size); 
        //Question Two: Does this assert pass?
        assert!(word_size == sentence_size);

    }
}

