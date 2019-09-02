
mod book;
mod borrow;

fn main() {
    let book_vec = book::book::GetBookList();

    println!("Book list");
    for book_item in book_vec.iter() {
         println!(" - ID: {:?}, Book Name: {:?}, Is Available: {:?}",book_item.Id, book_item.Name,book_item.Available);
    }

    let borrow_tuple  = ("Tom".to_string(),"Moby Dick".to_string());
    borrow::borrow::borrowBook(borrow_tuple);

}