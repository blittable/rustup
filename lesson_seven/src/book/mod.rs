pub mod book{
pub struct Book{
    pub Id: i32,
    pub Name: String,
    pub Available: bool
}

pub fn GetBookList() -> Vec<Book>
{
    let book_mock: Vec<Book> = 
    vec![Book{Id:1, Name:"Atomic Habits".to_string(), Available:true},
    Book{Id:2, Name:"In Search of Lost Time".to_string(), Available:true},
    Book{Id:3, Name:"Ulysses".to_string(), Available:true},
    Book{Id:4, Name:"Don Quixote".to_string(), Available:true},
    Book{Id:5, Name:"Moby Dick".to_string(), Available:true}
     ];
     return book_mock;
}
}