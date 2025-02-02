struct Manuscript {
    title: String,
    content: String,
}

struct Editor {
    name: String,
}

impl Manuscript {
    fn new(title: &str, content: &str) -> Self {
        Manuscript {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    fn display(&self) {
        println!("Title: {}", self.title);
        println!("Content: {}", self.content);
    }
}

fn edit(book: &mut Manuscript, editor: &Editor, new_content: &str) {
    println!("{} is editing the manuscript.", editor.name);
    book.content = new_content.to_string();
}

fn edit_borrow(book: &mut Manuscript, editor: Editor, new_content: &str) {
    println!("{} is editing the manuscript.", editor.name);
    book.content = new_content.to_string();
}

fn sell(book: Manuscript) {
    println!("Selling the manuscript: '{}'", book.title);
}

fn main() {
    let mut neuromancer = Manuscript::new("Neuromancer", "Original content of Neuromancer."); // Create a mutable Manuscript
    let ace_books = Editor {
        name: String::from("Ace Books"),
    }; // Create an Editor instance
    let dave = Editor {
        name: String::from("Dave"),
    }; // Create another Editor instance

    neuromancer.display(); // Display initial content

    // Mutable borrow of neuromancer
    edit(
        &mut neuromancer,
        &ace_books,
        "Updated content of Neuromancer.",
    ); // Edit the manuscript
    neuromancer.display(); // Display updated content
                           //
                           // Editing again with Dave (this line is commented to show the error)

    // Uncommenting the next line would cause an error
    // edit_borrow(&mut neuromancer, dave, "Neuromancer."); // ERROR: only one mutable borrow allowed
    // neuromancer.display(); // Display updated content

    edit(
        &mut neuromancer,
        &dave,
        "Further updated content of Neuromancer.",
    ); // Edit with Dave
    neuromancer.display(); // Display further updated content

    // Selling the manuscript
    sell(neuromancer); // Pass ownership to sell

    // Uncommenting the next line would cause an error
    // neuromancer.display(); // ERROR: neuromancer moved
}
