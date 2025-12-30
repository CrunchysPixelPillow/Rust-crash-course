
// Rust uses grammer like (Let | Const) and (Mut | Mutable)
//
// Variables always start with Let
// But never start with mut. This is because Mutable is a modifer for let
// Example
fn main() {
    let number=2;
    mut welcome="Hello Rust :)";
    ^
    | // Compiler complains because mut is by-itself. this correct way to assign a mutable variable ⬇️
    let mut welcome="Hello Again Rust :>";
}
