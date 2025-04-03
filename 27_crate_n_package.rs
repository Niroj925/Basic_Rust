//crate can contain one or more rust modules 

//types
//1.Binary crate
//2.library crate

//1.binary crate is a rust program that compile to an executables or more executable
//and has a main function for each executable


//2. libaray crate does not compile executables and does not have main function
//a library crate define a genrally shared functionality which can be used in multiple project

//crates can be bundled together into package

//creating package using cargo package manager which is build in rust

//package contain many binary package but at most only one library package


//creating binary package in rust
// cargo new hello_new --bin
//create folder like this
// hello_world //directory
// ├── Cargo.toml //contain metadata about crate such as name,version and dependencies
// └── src
//     └── main.rs //crate root and contain source code of the binary package

//creating library package
//cargo new hello_new_lib --lib


//Rust cargo package Manager
//cargo is command line tool for rust 
//features:
//1.Dependency management
//2.builing and packaging
//3.Document generation
//4.Download crates
//5.Run a binary or test

//dependency management with cargo in rust
//one of the cargo feature is that it can download,manage external libraries

//to add dependencies 
//cargo add rand(package name)
//cargo build 
//cargo run

//other useful cargo command 
// cargo new	Create a new Rust project with basic directory structure
// cargo build	Build (compile) the current project and generate a binary executable
// cargo run	Build and run your current project (cargo build + run)
// cargo check	Build the current project without generating a binary executable
// cargo add	Add a new dependency and include it in Cargo.toml file
// cargo update	Update all dependencies of current project to latest version
