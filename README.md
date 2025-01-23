# Tantalum Programming Language

## Language Features

- Functions
  ```
  fn main(): i32 {
      return 0;
  }
  ```
- Variables
  ```
  let x: i32 = 10;
  let y = 20;
  ```
- Control flow
  - If statement
    ```
    if x < 10 {
        return 0;
    } else {
        return 1;
    }
    ```
  - Return statement
    ```
    return 0;
    ```
- Literals
    - Integers: `0`
    - Floats: `0.0`
    - Booleans: `true`, `false`
- Expressions:
    - Binary operators: `*`, `/`, `%`, `+`, `-`, `<<`, `>>`, `<`, `>`, `<=`,
      `>=`, `==`, `!=`, `&`, `^`, `|`, `&&`, `||`, `=`
    - Prefix operators: `!`, `-`
    - Postfix operators: `.*`, `.&`

## License

This project is dual-licensed under the terms of the MIT license and the Apache
License (Version 2.0). See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## Acknowledgements

For the parser this project uses ideas from [Resilient LL Parsing Tutorial] and
[Simple but Powerful Pratt Parsing]. The code included in these blog posts are
licensed under the MIT or Apache-2.0 license (see <https://matklad.github.io/about.html>).

[Resilient LL Parsing Tutorial]: https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html

[Simple but Powerful Pratt Parsing]: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

