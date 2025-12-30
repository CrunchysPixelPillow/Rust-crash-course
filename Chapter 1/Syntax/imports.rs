
// Importing A library (builtin and 3dparty) in rust is pretty simple with the use keyword
// frameworks will showcase how to import in usage, or sometimes on the landing page for easy access!
// Imports should always go on-top of the rust file and instead of dots, rust uses :: for traits(classes)
// Example of importing Std:

use Std;
use Std::process;

// You may want to import more then one library at one
// We can do this by replacing the trait name after :: to a broken bracket
// Then the rest is like making an array, or json file. just forget the semicolon
// Example:

use Std::{Process, File, Example, Rand};
// Or another way
use Std::{
    Process,
    File, Example,
    Rand
};
// Rust isn't picky assuming its formatted the right way
