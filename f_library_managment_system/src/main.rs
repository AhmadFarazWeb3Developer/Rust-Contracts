struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

struct Library {
    name: String,
    address: String,
    book: Option<Book>, // there can be some book in library or none
}

enum LibraryError {
    BookNotAvaliable,
    BookNotFound,
    AlreadyBorrowed,
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
    fn return_book(self) {}
}

impl Library {
    fn add_book(self) {}
    fn borrow_book(&mut self) -> Result<&mut Book, LibraryError> {
        if let Some(book) = self.book {
            match book.borrow() {
                Ok(borrowed_book) => {
                    println!("Book is borrowed");
                    Ok(borrowed_book)
                }
                Err(err) => {
                    println!("Book is already borrowed");
                    Err(err)
                }
            }
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
    fn return_book(self) {}
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
        book: Some(book),
    };

    library.borrow_book()
    // Attempt to borrow then book and handle the result
}
