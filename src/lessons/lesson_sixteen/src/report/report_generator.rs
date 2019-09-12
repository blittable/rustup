//Report Team
use moobaan::corelib::*;

fn main() {
    moobaan::corelib::build_it();
}

fn generate_report() {
    println!();
    println!("***************************");
    println!("Here is your damn report...");
    println!("***************************");
}

fn banner_single_line<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn banner_lines<F>(f: F)
where
    F: Fn(i32),
{
    for x in 0..10 {
        f(x);
    }
}
