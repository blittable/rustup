fn main() {

    let name = "Krit";
    let name_with_description = "Welcome to my world."; 
    
    //println!("{}",name);
    myFunction(name,&name_with_description);
}

fn myFunction(mName: &str, mDesc: &str){
    println!("Hello {} {}",mName,mDesc);
}
