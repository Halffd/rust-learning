struct Book {
    title: String,
}

impl Book {
    fn new(title: &str) -> Self {
        Book {
            title: title.to_string(),
        }
    }

    fn loan_to(&self, user: &str) {
        println!("Loaning '{}' to {}.", self.title, user);
    }
}

fn copy_book(book: &Book) {
    println!("Copying the book: '{}'", book.title);
}

fn withdraw_book(book: Book) {
    println!("Withdrawing the book: '{}'", book.title);
}

fn main() {
    let neuromancer = Book::new("Neuromancer"); // Create a new Book instance

    // Borrowing the book for loaning
    neuromancer.loan_to("Lucy"); // OK
    neuromancer.loan_to("Nia"); // OK

    copy_book(&neuromancer); // Give up ownership to fn
    neuromancer.loan_to("Priya"); // This would cause an error

    // After this point, we can withdraw the book
    withdraw_book(neuromancer); // Give up ownership to fn

    // We cannot use `neuromancer` anymore here
    //neuromancer.loan_to("Priya"); // This would cause an error
}
