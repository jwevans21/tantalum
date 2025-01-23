#[macro_export]
macro_rules! setup_parser {
    (
        $files:ident with $parser:ident => {
            $entry_file_name:ident => $entry_file_contents:literal,
            $($file_name:ident => $file_contents:literal),*
        }
    ) => {
        let mut $files = tantalum_source::SourceFileCollection::new();
        let $entry_file_name = $files.add_file(
            stringify!($entry_file_name).to_string(),
            $entry_file_contents.to_string()
        );
        $(
            let $file_name = files.add_file(
                stringify!($file_name).to_string(),
                $file_contents.to_string()
            );
        )*
        let $files = $files;

        let mut $parser = {
            let lexer = tantalum_lexer::Lexer::new($files.get_file($entry_file_name).expect("file not found"));
            $crate::Parser::new(lexer)
        };
    };
}

#[macro_export]
macro_rules! pretty_snapshot {
 (
     $tree:ident with $files:ident => $result:literal
 ) => {
     {
         use tantalum_syntax::PrettyPrint;
         let mut result = String::new();
            $tree.pretty_print(&mut result, 0, &$files)
                .expect("pretty print failed");
         insta::assert_snapshot!(result, @$result);
     }
 };
}
