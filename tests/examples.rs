extern crate tantalum_lexer;
extern crate tantalum_parser;
extern crate tantalum_source;

use std::path::Path;
use tantalum_lexer::Lexer;
use tantalum_parser::Parser;
use tantalum_source::SourceFileCollection;

fn compile_test(path: impl AsRef<Path>) -> bool {
    eprintln!("testing compilation of \"{}\"", path.as_ref().display());

    // load the file
    let mut files = SourceFileCollection::new();

    let file_id = match files.add_file_from_path(path) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("error loading file: {}", e);
            return false;
        }
    };

    let file = match files.get_file(file_id) {
        Some(f) => f,
        None => {
            eprintln!("error getting file from collection");
            return false;
        }
    };

    // parse the file
    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);

    parser.parse();

    let _ = parser.finish();

    eprintln!("successfully parsed file");

    true
}

#[test]
fn test_examples() {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    for entry in examples_dir
        .read_dir()
        .expect("error reading examples directory")
    {
        let entry = entry.expect("error reading entry");
        let path = entry.path();

        if path.is_file() {
            compile_test(path);
        }
    }
}
