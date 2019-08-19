//Playing with immutable and borrowing

fn main() {
    let a = 5;
    let b = a;

    println!("{}", a );//print 5
    println!("{}", b );//print 5

    let mut s1 = "12345";
    let s2 = s1;
    println!("s1 before :{}", s1 );//print 12345
    println!("s2 before :{}", s2 );//print 12345

    s1 = "11111";//mutable variable will not effect to s2
    println!("s1 after :{}", s1 );//print 11111
    println!("s2 after :{}", s2 );//print 12345

    let message1 = "my message".to_string();
    println!("message1 before :{}", message1 );

    doSomeA(message1);

    //println!("message1 after :{}", message1 );//Borrowing -- throw an error "message1 value borrowed here after move" -because message1 is no longer exist in memory.

    let message2 = "my message 2".to_string();
    println!("message2 before :{}", message2 );
    doSomeB(&message2);
    println!("message2 after :{}", message2 );// by using reference, message2 still exist in memory


}

fn doSomeB(var2 :&String)
{
    println!("DoSomeA:{}", var2);
}

fn doSomeA(var1 :String)
{
    println!("DoSomeA:{}", var1);
}

