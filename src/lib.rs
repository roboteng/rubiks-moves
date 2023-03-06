//! A crate for manipulating algorithms for the Rubik's cube, and speed-cubing
//!
//! # Example
//!
//! ```
//! use rubiks_moves::moves::Algorithm;
//!
//! let moves = Algorithm::from("R U R' U'").unwrap();
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

// https://jperm.net/3x3/moves
mod cube;

pub mod moves;
