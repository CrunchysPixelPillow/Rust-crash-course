## Cargo

### Create-Project
To create a cargo project in your currect directory you can use two commands:
```
Cargo init <projectname>
Cargo new <projectname>
cd <projectname>
```

### Run-Project
To run a cargo project make sure current-directory is the folders root
You can also add dependencies using cargo, and remove by simple editing <cargo.toml>
```
cargo build
cargo run - This will also build
cargo add <library>
```

### Project Structure (needs work)
```
Example_Folder

  * Src
    main.rs

cargolock.toml
cargo.toml - targets main.rs
```
