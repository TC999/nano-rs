# nano-rs

A Rust port of the nano text editor, converted from C using c2rust.

## Status

✅ **Compiles successfully** with Rust nightly toolchain  
✅ **Builds and runs** without errors  
🚧 **Functionality** - Basic module structure in place, full editor features to be integrated  

## Prerequisites

- Rust nightly toolchain (required for unstable features)

```bash
rustup toolchain install nightly
rustup override set nightly
```

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Project Structure

This project contains c2rust converted modules from the original GNU nano editor:

- `src/main.rs` - Entry point and module organization
- `src/browser.rs` - File browser functionality
- `src/cut.rs` - Cut/copy/paste operations  
- `src/files.rs` - File I/O operations
- `src/global.rs` - Global state and initialization
- `src/search.rs` - Search and replace functionality
- `src/text.rs` - Text manipulation functions
- `src/winio.rs` - Window/terminal I/O
- `src/move.rs` - Cursor movement functions
- `src/color.rs` - Syntax highlighting and colors
- And more...

## Compilation Notes

This codebase has been fixed to compile with modern Rust. See `COMPILATION_FIXES.md` for details on the changes made to make the c2rust output compile successfully.

The code currently generates many warnings (typical for c2rust converted code) but compiles and runs without errors.

## Dependencies

- `libc` - for C interop
- `regex` - for regular expression support

## License

Same as original GNU nano (GPL-3.0)