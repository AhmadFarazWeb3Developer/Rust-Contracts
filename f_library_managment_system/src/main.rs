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
    fn return_book(self) {}
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
        // let some_value=Some(boo);
        // of some_value= etc , so the readable code is used below
        if let Some(book) = self.book.as_mut() {
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
    fn return_book() {}
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

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book : {:#?}", result);
        }
        Err(err) => {
            println!("Borrowed book : {:#?}", err);
        }
    }

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book : {:#?}", result);
        }
        Err(err) => {
            println!("Borrowed book : {:#?}", err);
        }
    }

    // Attempt to borrow then book and handle the result
}
