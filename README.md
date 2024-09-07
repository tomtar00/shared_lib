# shared_lib

Wrapper around [libloading](https://github.com/nagisa/rust_libloading) crate that is a binding around platform's dynamic library loading primitives with greatly improved memory safety.
`shared_lib` aims to improve the system path handling by using a custom structure representing platform independent path.
The library also returns custom enum variants to better distinguish different kinds of errors.

## Instalation

```
cargo add shared_lib
```
or add `shared_lib` to your `Cargo.toml` file.

## Usage

```rust
use shared_lib::*;
use std::path::PathBuf;

fn main() {
    let lib_path = LibPath::new(PathBuf::from("path/to/dir"), "library_name_no_ext".into());
    unsafe {
       let lib = SharedLib::new(lib_path).unwrap();
       let func = lib.get_fn::<fn(usize, usize) -> usize>("foo").unwrap();
       let result = func.run(1, 2);
       println!("Result = {}", result);
    }
}
```

[Documentation](https://docs.rs/shared_lib/latest/shared_lib/)

This library is available under the MIT License
