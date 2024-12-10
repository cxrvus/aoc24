pub mod vec2 {
	use std::ops;

	#[derive(Debug, Default, Copy, Clone, PartialEq)]
	pub struct Vec2 {
		pub x: i32,
		pub y: i32,
	}

	#[derive(Debug, Default, Copy, Clone, PartialEq)]
	pub struct Vec2u {
		pub x: usize,
		pub y: usize,
	}

	impl Vec2 {
		pub fn unsign(self) -> Option<Vec2u> {
			let Self { x, y } = self;
			if x >= 0 && y >= 0 {
				Some(Vec2u {
					x: x as usize,
					y: y as usize,
				})
			} else {
				None
			}
		}

		pub const X: Self = Self { x: 1, y: 0 };
		pub const Y: Self = Self { x: 0, y: 1 };
		pub const ZERO: Self = Self { x: 0, y: 0 };
	}

	impl Vec2u {
		pub fn sign(self) -> Vec2 {
			let Self { x, y } = self;
			Vec2 {
				x: x as i32,
				y: y as i32,
			}
		}
	}

	impl ops::Add<Vec2> for Vec2 {
		type Output = Self;

		fn add(self, other: Vec2) -> Self::Output {
			Self {
				x: self.x + other.x,
				y: self.y + other.y,
			}
		}
	}

	impl ops::Sub<Vec2> for Vec2 {
		type Output = Self;

		fn sub(self, other: Vec2) -> Self::Output {
			Self {
				x: self.x - other.x,
				y: self.y - other.y,
			}
		}
	}

	impl ops::Mul<i32> for Vec2 {
		type Output = Self;

		fn mul(self, scalar: i32) -> Self::Output {
			Self {
				x: self.x * scalar,
				y: self.y * scalar,
			}
		}
	}

	impl ops::Neg for Vec2 {
		type Output = Self;

		fn neg(self) -> Self::Output {
			Self {
				x: -self.x,
				y: -self.y,
			}
		}
	}
}

pub mod map {
	use super::super::util::vec2::*;
	use std::fmt::Debug;

	#[derive(Debug)]
	pub struct Map<T: Debug + PartialEq> {
		pub width: usize,
		pub height: usize,
		pub values: Vec<T>,
	}

	impl<T: Debug + PartialEq> Map<T> {
		pub fn in_bounds(&self, pos: &Vec2) -> bool {
			let Vec2 { x, y } = *pos;
			x >= 0 && y >= 0 && y < self.height as i32 && x < self.width as i32
		}

		pub fn at(&self, pos: &Vec2) -> Option<&T> {
			let Vec2u { x, y } = pos.unsign()?;
			if self.in_bounds(pos) {
				Some(&self.values[y * self.width + x])
			} else {
				None
			}
		}

		pub fn set_at(&mut self, pos: &Vec2, value: T) {
			if self.in_bounds(pos) {
				let Vec2u { x, y } = pos.unsign().unwrap();
				self.values[y * self.width + x] = value;
			} else {
				panic!("map index is out of range: {:?} = {:?}", pos, value)
			}
		}

		pub fn find_all(&self, target: T) -> Vec<Vec2u> {
			self.values
				.iter()
				.enumerate()
				.filter(|(_, value)| **value == target)
				.map(|(i, _)| self.get_pos(i).unwrap())
				.collect()
		}

		pub fn get_pos(&self, i: usize) -> Option<Vec2u> {
			self.values.get(i)?;
			Some(Vec2u {
				x: (i % self.width),
				y: (i / self.width),
			})
		}
	}

	#[derive(Debug)]
	pub struct ProxyMap {
		pub width: usize,
		pub height: usize,
		pub string: String,
	}

	impl From<&str> for ProxyMap {
		fn from(value: &str) -> Self {
			let lines = value.trim().lines();

			Self {
				height: lines.clone().count(),
				width: lines.clone().next().unwrap().len(),
				string: lines.collect::<Vec<&str>>().join(""),
			}
		}
	}
}
