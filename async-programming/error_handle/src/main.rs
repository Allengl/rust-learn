use std::fs;

fn main() {}

#[derive(Debug, thiserror::Error)]
enum BooksError {
    #[error("book not found")]
    BookNotFound,
    #[error("too many books")]
    TooManyBooks,
    #[error("file read failed")]
    FileReadFailed,
}

fn load_books() -> Result<Vec<String>, BooksError> {
    let content = fs::read_to_string("books.txt").map_err(|_| BooksError::FileReadFailed)?;

    let books: Vec<String> = content
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.trim().is_empty())
        .collect();
    if books.is_empty() {
        return Err(BooksError::BookNotFound);
    } else if books.len() > 10 {
        return Err(BooksError::TooManyBooks);
    }
    Ok(books)
}
