pub enum Publication {
    Book(Book),
    Magazine(Magazine),
}

pub struct Book {
    title: String,
    author: String,
    page_count: u32,
}

pub struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

impl Book {
    pub fn new(title: String, author: String, page_count: u32) -> Self {
        Self {
            title,
            author,
            page_count,
        }
    }
}

impl Magazine {
    pub fn new(title: String, issue: u32, topic: String) -> Self {
        Self {
            title,
            issue,
            topic,
        }
    }
}

trait Printable {
    fn print(&self);
}

impl Printable for Publication {
    fn print(&self) {
        match self {
            Publication::Book(book) => {
                println!(
                    "Book: {} author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Magazine: {} - Issue: {}, Topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}

pub fn print_publications(publications: Vec<Publication>) {
    let print = publications.iter().map(|x| x.print()).collect();
    print
}
