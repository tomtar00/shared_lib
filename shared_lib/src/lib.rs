//! A small wrapper around the libloading crate that aims to improve the system path and error handling.
//!
//! # Usage
//!
//! In your code, run the following:
//!
//! ```no_run
//! use shared_lib::*;
//! use std::path::PathBuf;
//! 
//! let lib_path = LibPath::new(PathBuf::from("path/to/dir"), "library_name_no_ext".into());
//! unsafe {
//!    let lib = SharedLib::new(lib_path).unwrap();
//!    let func = lib.get_fn::<fn(usize, usize) -> usize>("foo").unwrap();
//!    let result = func.run(1, 2);
//! }
//! ```

use thiserror::Error;
use libloading::{library_filename, Library, Symbol};
use std::{ffi::OsString, path::PathBuf};

/// Enum representing the possible errors that can occur when working with shared libraries.
#[derive(Debug, Error)]
pub enum SharedLibError {
    #[error("Path is empty.")]
    PathEmpty,
    #[error("Failed to convert path '{0}' to {1}.")]
    PathConversion(PathBuf, String),
    #[error("Failed to load library from path '{path}'. {msg}")]
    LoadFailure { path: String, msg: String },
    #[error("Failed to find symbol '{symbol_name}' in library '{lib_name}'. {msg}")]
    SymbolNotFound { symbol_name: String, lib_name: String, msg: String }
}

/// Structure representing a shared library path.
///
/// `dir_path` is the directory path where the library is located.
///
/// `lib_name` is the library name without the platform specific extension and prefix.
#[derive(Clone, Debug)]
pub struct LibPath {
    pub dir_path: PathBuf,
    pub lib_name: String,
}
impl ToString for LibPath {
    fn to_string(&self) -> String {
        let binding = self.path().unwrap();
        binding.to_str().unwrap().to_string()
    }
}
impl TryInto<OsString> for LibPath {
    type Error = SharedLibError;
    fn try_into(self) -> Result<OsString, Self::Error> {
        let path = self.path()?;
        path.clone().try_into().map_err(|_| {
            SharedLibError::PathConversion(path, "OsString".into())
        })
    }
}
impl LibPath {
    /// Create a new shared library path.
    ///
    /// `dir_path` is the directory path where the library is located.
    ///
    /// `lib_name` is the library name without the platform specific extension and prefix.
    pub fn new(dir_path: PathBuf, lib_name: String) -> LibPath {
        LibPath { dir_path, lib_name }
    }
    /// Create a new shared library path without a directory path.
    /// Using this function will mean that the library is located in the current directory.
    ///
    /// `lib_name` is the library name without the platform specific extension and prefix.
    pub fn new_no_path(lib_name: String) -> LibPath {
        LibPath {
            dir_path: PathBuf::new(),
            lib_name,
        }
    }
    /// Get the platform specific library filename.
    ///
    /// For Windows, it will return the library name with `.dll` extension.
    ///
    /// For MacOS, it will return the library name with `lib` prefix and `.dylib` extension.
    ///
    /// For Linux, it will return the library name with `lib` prefix and `.so` extension.
    /// # Example
    /// ```no_run
    /// use std::ffi::OsString;
    /// use shared_lib::*;
    ///
    /// let lib_path: LibPath = LibPath::new_no_path("test_name".into());
    /// let lib_name: OsString = lib_path.filename().expect("Failed to get library name");
    /// ```
    pub fn filename(&self) -> Result<OsString, SharedLibError> {
        if self.lib_name.is_empty() {
            return Err(SharedLibError::PathEmpty);
        }
        Ok(library_filename(self.lib_name.clone()))
    }
    /// Get the platform specific library filepath.
    ///
    /// `dir_path` is the directory path where the library is located.
    ///
    /// `lib_name` is the library name without the platform specific extension.
    /// # Example
    /// ```no_run
    /// use std::path::PathBuf;
    /// use shared_lib::*;
    ///
    /// let lib_path: LibPath = LibPath::new(PathBuf::from("path/to/shared/library"), "shared_library".into());
    /// let lib_path: PathBuf = lib_path.path().expect("Failed to get library path");
    /// ```
    pub fn path(&self) -> Result<PathBuf, SharedLibError> {
        Ok(self.dir_path.join(self.filename()?))
    }
}

/// Structure representing a shared library function.
#[derive(Clone)]
pub struct SharedLibFn<'a, Fn> {
    symbol: Symbol<'a, Fn>,
}
impl<'a, Fn> SharedLibFn<'a, Fn> {
    pub unsafe fn new(symbol: Symbol<'a, Fn>) -> SharedLibFn<'a, Fn> {
        SharedLibFn { symbol }
    }
}
impl<'a, Ret> SharedLibFn<'a, fn() -> Ret> {
    pub unsafe fn run(&self) -> Ret {
        (self.symbol)()
    }
}
// === Implementations for functions with arguments (Rust does not support variadic functions yet)
impl<'a, Ret, A1> SharedLibFn<'a, fn(A1) -> Ret> {
    pub unsafe fn run(&self, a1: A1) -> Ret {
        (self.symbol)(a1)
    }
}
impl<'a, Ret, A1, A2> SharedLibFn<'a, fn(A1, A2) -> Ret> {
    pub unsafe fn run(&self, a1: A1, a2: A2) -> Ret {
        (self.symbol)(a1, a2)
    }
}
impl<'a, Ret, A1, A2, A3> SharedLibFn<'a, fn(A1, A2, A3) -> Ret> {
    pub unsafe fn run(&self, a1: A1, a2: A2, a3: A3) -> Ret {
        (self.symbol)(a1, a2, a3)
    }
}
impl<'a, Ret, A1, A2, A3, A4> SharedLibFn<'a, fn(A1, A2, A3, A4) -> Ret> {
    pub unsafe fn run(&self, a1: A1, a2: A2, a3: A3, a4: A4) -> Ret {
        (self.symbol)(a1, a2, a3, a4)
    }
}
impl<'a, Ret, A1, A2, A3, A4, A5> SharedLibFn<'a, fn(A1, A2, A3, A4, A5) -> Ret> {
    pub unsafe fn run(&self, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5) -> Ret {
        (self.symbol)(a1, a2, a3, a4, a5)
    }
}
// ===

/// Structure representing a shared library.
pub struct SharedLib {
    lib: Library,
    lib_path: LibPath
}
impl SharedLib {
    /// Create a new shared library from the given path.
    /// # Safety
    /// This function is unsafe because it loads a shared library, which is generally unsafe as it is a foregin code.
    pub unsafe fn new(lib_path: LibPath) -> Result<SharedLib, SharedLibError> {
        let os_str: OsString = lib_path.clone().try_into()?;
        let lib = match Library::new(os_str) {
            Ok(lib) => lib,
            Err(e) => {
                let path_str: OsString = lib_path.try_into()?;
                let path_str: String = path_str.to_string_lossy().to_string();
                return Err(SharedLibError::LoadFailure {
                    path: path_str, 
                    msg: e.to_string()
                });
            }
        };
        Ok(SharedLib { lib, lib_path })
    }
    /// Get a function by name from the shared library.
    /// # Safety
    /// This function is unsafe because it loads a function from the shared library, which is generally unsafe as it is a foregin code.
    /// # Example
    /// ```no_run
    /// use std::path::PathBuf;
    /// use shared_lib::*;
    /// unsafe {
    ///     let lib_path = LibPath::new(PathBuf::from("path/to/shared/library"), "shared_library".into());
    ///     let lib = SharedLib::new(lib_path).expect("Failed to load shared library");
    ///     let add_fn = lib.get_fn::<fn(usize, usize) -> usize>("add").expect("Failed to get 'add' function from shared library");
    ///     let result = add_fn.run(1, 2);
    /// }
    /// ```
    pub unsafe fn get_fn<T>(&self, fn_name: &str) -> Result<SharedLibFn<T>, SharedLibError> {
        let symbol = match self.lib.get(fn_name.as_bytes()) {
            Ok(symbol) => symbol,
            Err(e) => {
                return Err(SharedLibError::SymbolNotFound { 
                    symbol_name: fn_name.to_owned(), 
                    lib_name: self.lib_path.path()?.to_string_lossy().to_string(),
                    msg: e.to_string(), 
                });
            }
        };
        Ok(SharedLibFn::new(symbol))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lib_name() {
        let lib_path = LibPath::new_no_path("test_name".into());
        let lib_os_string: OsString = lib_path.try_into().unwrap();
        if cfg!(target_os = "windows") {
            assert_eq!(lib_os_string, OsString::from("test_name.dll"));
        } else if cfg!(target_os = "macos") {
            assert_eq!(lib_os_string, OsString::from("libtest_name.dylib"));
        } else if cfg!(target_os = "linux") {
            assert_eq!(lib_os_string, OsString::from("libtest_name.so"));
        } else {
            panic!("Unknown target OS: {}", std::env::consts::OS);
        }
    }
    #[test]
    #[should_panic]
    fn create_lib_name_empty() {
        let lib_path = LibPath::new_no_path("".into());
        let _: OsString = lib_path.try_into().unwrap();
    }
    #[test]
    fn create_lib_path() {
        let lib_path = LibPath::new(PathBuf::from("test_dir"), "test_name".into());
        let lib_os_string: OsString = lib_path.try_into().unwrap();
        if cfg!(target_os = "windows") {
            assert_eq!(lib_os_string, OsString::from("test_dir\\test_name.dll"));
        } else if cfg!(target_os = "macos") {
            assert_eq!(lib_os_string, OsString::from("test_dir/libtest_name.dylib"));
        } else if cfg!(target_os = "linux") {
            assert_eq!(lib_os_string, OsString::from("test_dir/libtest_name.so"));
        } else {
            panic!("Unknown target OS: {}", std::env::consts::OS);
        }
    }
    #[test]
    #[should_panic]
    fn create_lib_path_empty() {
        let lib_path = LibPath::new(PathBuf::from("test_dir"), "".into());
        let _: OsString = lib_path.try_into().unwrap();
    }
}
