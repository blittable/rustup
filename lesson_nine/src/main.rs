// struct Binarynode<'a>{
//     myvalue: &'a i32
// }

// struct Leaf<'a>{
//     mynode: &'a Binarynode<'a>,
//     left_node: Some(&'a Binarynode<'a>),
//     right_node: Some(&'a Binarynode<'a>)
// }

struct Binarynode<'a>{
    myvalue: &'a i32,
    left_node: Some(&'a Binarynode<'a>),
    right_node: Some(&'a Binarynode<'a>)
}

fn main() {
    println!("Hello, world!");

    let intarray: [i32; 5] = [3,2,4,1,5];
    println!("My array is {:?}", intarray);

    let mut master_node : Binarynode = None;

    let binarytree: Vec<Binarynode>;
    for number in intarray.iter() {
        let new_node: Binarynode = Binarynode {
                myvalue : &number,
                left_node: None,
                right_node: None,
            };  

        println!("My number is {:?}", &number);

        if(master_node == None)
        {
            master_node = &new_node;
        }
        else
        {
            if(&number >  &master_node.myvalue)
            {
                new_node.left_node = &new_node;
            }
            if(&number <  &master_node.myvalue)
            {
                new_node.right_node = &new_node;
            }
        }

        
        binarytree.push(&new_node);

        
        
    }
}
