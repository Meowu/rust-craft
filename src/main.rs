const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!(
        "===================================================\n\
        Welcome to lox {}!\n\
        Inspired by the book \x1B[3mCrafting Interpreters\x1B[0m\nAuthors: {}\n\
        ===================================================\n",
        VERSION, AUTHORS
    );
}
