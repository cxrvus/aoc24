#![allow(dead_code)]

pub mod template;
pub mod util;

mod days;
use days::*;

fn main() {
	let result = day16::part1();
	println!("{result}");
}
