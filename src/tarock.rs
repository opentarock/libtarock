#![crate_name = "tarock"]
#![crate_type = "lib"]
#![feature(phase)]
#![feature(globs)]
#![feature(macro_rules)]

#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;

#[cfg(test)]
extern crate quickcheck;

mod util;

pub mod cards;
pub mod player;
pub mod contracts;

pub mod bidding;
pub mod bonuses;
pub mod announcements;
