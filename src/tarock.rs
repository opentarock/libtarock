#![crate_name = "tarock"]
#![crate_type = "lib"]
#![feature(phase)]
#![feature(globs)]

#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;

#[cfg(test)]
extern crate quickcheck;

pub mod cards;
pub mod player;
pub mod contracts;

pub mod bidding;
