use std::result;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

#[derive(Debug)]
struct Library {
    name: String,
    address: String,
    book: Option<Book>, // there can be some book in library or none
}
#[derive(Debug)]
enum LibraryError {
    BookNotAvaliable,
    BookNotFound,
    AlreadyBorrowed,
    BookAlreadyExits,
}

impl Book {
    fn borrow(&mut self) -> Result<&mut Book, LibraryError> {
        if self.is_avaliable == true {
            self.is_avaliable = false;
            Ok(self)
        } else {
            Err(LibraryError::AlreadyBorrowed)
        }
    }
    fn return_book(&mut self) {
        self.is_avaliable = true;
    }
}

impl Library {
    fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
        if self.book.is_none() {
            self.book = Some(book);
            Ok(())
        } else {
            Err(LibraryError::BookAlreadyExits)
        }
    }

    fn borrow_book(&mut self) -> Result<&mut Book, LibraryError> {
        if let Some(book) = self.book.as_mut() {
            match book.borrow() {
                Ok(borrowed_book) => { Ok(borrowed_book) }
                Err(err) => { Err(err) }
            }
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
    fn return_book(&mut self) -> Result<(), LibraryError> {
        if let Some(book) = self.book.as_mut() {
            book.return_book();
            Ok(())
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
}
fn main() {
    let mut book: Book = Book {
        title: String::from("The rust book"),
        author: String::from("Steve klabnik"),
        is_avaliable: true,
    };

    let mut library: Library = Library {
        name: String::from(String::from("City Library")),
        address: String::from(String::from("")),
        book: None, // Some(book)
    };

    // Add book to libaray
    match library.add_book(book) {
        Ok(_) => {
            println!("Book added");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    // borrow book
    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book : {:#?}", result);
        }
        Err(err) => {
            println!("Borrowed book : {:#?}", err);
        }
    }

    // again borrow book
    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book : {:#?}", result);
        }
        Err(err) => {
            println!("Borrowed book : {:#?}", err);
        }
    }

    // return book
    match library.return_book() {
        Ok(_) => {
            println!("Book Returned Successfully");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
}
