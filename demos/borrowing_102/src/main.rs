fn main() {

    // SCOPE 
    {
        //Assign a numeric literal to the implicitly typed identifier, limited_scope
        //Scope is limited to the { }
        let limited_scope = 7;
    }

    // Compiler Error - limited_scope not found  in this scope
    //println!("Value of : {:?}", limited_scope);

    // IMMUTABILITY
    let x = 1;
    //x = 2;
    //Compile Error - by default variables are immutable

    /// 'mut' keyword, stack allocated because of fixed size 
    let mut z = 1;
    z = z + 1;
    let a = z;
    let b = z;

    //BORROWING, heap allocated because of unknown size
    let name_a: String = String::from("Mycos");
    let name_b: String = String::from(" Rocks");

    //Play with this for various explosions
    //1) Remove the borrow in front of name_a
    let name_c = &name_a;

    let mut output_string = String::default(); 

    output_string.push_str(&name_a);
    output_string.push_str(&name_b);
    output_string.push_str(&name_c);

    println!("Output: {:?} ", output_string);
}
