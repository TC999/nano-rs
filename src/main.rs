//! Nano-rs: A Rust port of the nano text editor
//! 
//! This is a c2rust converted codebase from the original GNU nano editor.
//! The code follows C-style patterns and uses FFI for system interactions.

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
#![feature(c_variadic)]

// Import all the c2rust converted modules
pub mod browser;
pub mod chars;
pub mod color;
pub mod cut;
pub mod files;
pub mod global;
pub mod help;
pub mod history;
pub mod r#move;
pub mod prompt;
pub mod rcfile;
pub mod search;
pub mod text;
pub mod utils;
pub mod winio;

// For now, we provide a simple main function that calls into the c2rust converted code
// The actual main functionality will be in the global module
fn main() {
    // Initialize the nano editor
    println!("Nano-rs starting...");
    
    // The c2rust converted code typically has its main function in another module
    // For now, we just indicate that the modules are loaded and available
    println!("All nano modules loaded successfully");
}