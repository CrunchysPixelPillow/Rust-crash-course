

// Simple functions in rust are like any other language... idk why it wouldn't
// Functions are created with the keyword (fn) like haskell and return type with an arrow like ->
fn Example(name) -> String {
    return "Hello, "+name;
}

// For unknown types (i think) you can return _ and rustc will find the type and assign it
// functions don't require a return statement or to return anything at all like most languages

// Example of no return statement:
fn NoReturn(name, age) -> String {
    "hello "+name+" you are "+age+" years old, thats so cool";
    // No return needed, rust will find the last line and check its type
    // (not sure thats 100% the way it works but you get how to use)
    // Also both (name) and (age) may be string so you will need to handle adding (str to string) or however
}
