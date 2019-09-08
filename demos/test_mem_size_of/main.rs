use std::mem;

fn main() {
    println!("Hello, world!");
}

#[allow(unused_imports)]
mod test {
    use std::mem;

    struct Node2D {
        x: i32,
        y: i32,
        v: i32,
    }

    struct Graph {
        tensor: Vec<Node2D>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self { tensor: Vec::new() }
        }
    }

    #[test]
    fn build_graph() {
        let mut g = Graph::new();

        g.tensor.push(Node2D {
            x: 12,
            y: 12,
            v: 124,
        });

        for i in 0..1000000000 {
            g.tensor.push(Node2D {
                x: i,
                y: i,
                v: i + 1,
            });
        }

        println!("Graph Length: {:?}", mem::size_of_val(&g.tensor));
    }

    //MID-TERM!
    #[test]
    fn size_test() {
        let a: u8 = 0b00000000;
        println!("slice shift  size: {:?}", mem::size_of_val(&a));

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
