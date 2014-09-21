#![crate_name = "tarock"]
#![crate_type = "lib"]
#![feature(phase)]

#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;

#[cfg(test)]
extern crate quickcheck;

pub mod cards;
