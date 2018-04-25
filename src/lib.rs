// #[macro_use(array)]
#![feature(box_syntax, box_patterns)]

extern crate ndarray;
extern crate quickersort;
extern crate rand;


#[macro_use]
extern crate serde_derive;

pub mod utils;
pub mod search;
pub mod sort;
pub mod dynamic;
pub mod tree;