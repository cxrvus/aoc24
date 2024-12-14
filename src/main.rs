#![allow(dead_code)]

pub mod template;
pub mod util;

mod days;
use days::*;

fn main() {
	let result = day14::part1();
	println!("{result}");
}
