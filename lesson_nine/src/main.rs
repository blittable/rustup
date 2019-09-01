//Homework
//Create a binary tree datastructure. Each node may have a parent, a node to the left, and a node to the right. 
//Add one function/method that allows insertion of a node.
#[derive(Debug)]
struct Node{
    value: i32,
    left: Option<Box::<Node>>,
    right: Option<Box::<Node>>
}

impl Node{
    pub fn insert(&mut self, new_value: i32) {
        if self.value == new_value {
            return
        }
        let target_node = if new_value < self.value { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_value.clone()),
            &mut None => {
                let new_node = Node { value: new_value.clone(), left: None, right: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

fn main() {
    let mut x = Node { value: 2, left: None, right: None };
    x.insert(1);
    x.insert(3);
    //x.insert(4);
    println!("{:#?}", x);
}

//Sample on Internet
// #[derive(PartialEq)]
// struct Node<'a> {
//     val: &'a str,
//     l: Option<Box<Node<'a>>>,
//     r: Option<Box<Node<'a>>>,
// }
// impl<'a> Node<'a> {
//     pub fn insert(&mut self, new_val: &'a str) {
//         if self.val == new_val {
//             return
//         }
//         let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
//         match target_node {
//             &mut Some(ref mut subnode) => subnode.insert(new_val),
//             &mut None => {
//                 let new_node = Node { val: new_val, l: None, r: None };
//                 let boxed_node = Some(Box::new(new_node));
//                 *target_node = boxed_node;
//             }
//         }
//     }
// }
// fn main () {
//     let mut x = Node { val: "m", l: None, r: None };
//     x.insert("z");
//     x.insert("b");
//     x.insert("c");
//     assert!(x == Node {
//         val: "m",
//         l: Some(Box::new(Node {
//             val: "b",
//             l: None,
//             r: Some(Box::new(Node { val: "c", l: None, r: None })),
//         })),
//         r: Some(Box::new(Node { val: "z", l: None, r: None })),
//     });
// }