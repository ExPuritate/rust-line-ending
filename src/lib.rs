#![feature(cfg_select)]
#![feature(derive_const)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_clone)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod line_ending;
pub use line_ending::{LineEnding, LineEndingScores};

pub mod peekable_ext;
pub use peekable_ext::PeekableLineEndingExt;
