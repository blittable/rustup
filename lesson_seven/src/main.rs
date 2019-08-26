mod child;
mod school;
mod display;
fn main() {
    let a = child::get_children();
    let best = school::get_max_score(a);
    display::display_best_child(best);
}
