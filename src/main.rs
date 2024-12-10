#![allow(dead_code)]

pub mod template;
pub mod util;

mod days;
use days::*;

fn main() {
	let result = day10::part2();
	println!("{result}");
}
