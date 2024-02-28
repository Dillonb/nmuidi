//! Deletes stuff, hopefully quickly
//!
//! ## Installation
//! - [Download for Windows](https://nightly.link/Dillonb/nmuidi/workflows/build/main/nmuidi-windows.zip)
//! - Or just use `cargo`
//!
//! ## Benchmarks
//! - [This video](https://www.youtube.com/watch?v=G8BdXgBdaOA) benchmarks several popular suggestions for deleting files quickly on Windows and compares them to nmuidi.
//!
//! ## How to use
//! ### As a command-line tool
//! - You can download using the link above. The easiest way to use it in Windows is to make a folder (something like `C:\bin`)
//! - Add that folder to your path
//! - Then add `nmuidi.exe` file you downloaded to that folder and restart any terminals you have open
//!
//! Then you can run `nmuidi /path/to/some/dir` and you should see some output like the following:
//! ```PS
//! → ~\repos\nmuidi [main ≡ +0 ~1 -0 !]› nmuidi test
//! Cleaning test
//! ```
//!
//! ### As a library
//! ```
//! use nmuidi::prelude::*;
//!
//! let dir = "path/to/something";
//! Cleaner::new(dir).clean();
//! ```
//!

/// All nmuidi `Cleaner` functionality
pub mod prelude {
    pub use crate::nmuidi::Cleaner;
}

/// Module containing basic libray functionality
#[doc(hidden)]
pub mod nmuidi;
