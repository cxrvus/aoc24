struct FlatStones(Vec<u64>);

impl From<&str> for FlatStones {
	fn from(value: &str) -> Self {
		Self(
			value
				.trim()
				.split(' ')
				.map(|x| x.parse().unwrap())
				.collect(),
		)
	}
}

impl FlatStones {
	fn blink(&mut self, n: u64) {
		let stones = &mut self.0;
		for blink in 0..n {
			let mut insertions = 0;
			for i in 0..stones.len() {
				let i = i + insertions;
				if stones[i] == 0 {
					stones[i] = 1;
				} else if let Some((left, right)) = Self::split(stones[i]) {
					stones[i] = right;
					stones.insert(i, left);
					insertions += 1;
				} else {
					stones[i] *= 2024
				}
			}
			dbg!(&stones, blink);
		}
	}

	fn split(stone: u64) -> Option<(u64, u64)> {
		let mut n = stone;
		let mut count = 1u32;

		while n >= 10 {
			n /= 10;
			count += 1;
		}

		if count % 2 != 0 {
			None
		} else {
			let half_count = count / 2;
			let mut left = stone;

			for _ in 0..half_count {
				left /= 10;
			}

			let right = stone - left * 10u64.pow(half_count);

			Some((left, right))
		}
	}
}

#[derive(Debug, Default)]
struct Stone {
	blinks: u32,
	left: u64,
	right: Option<Box<Self>>,
}

impl Stone {
	fn vec(stones: FlatStones) -> Vec<Self> {
		stones
			.0
			.iter()
			.map(|&left| Self {
				left,
				..Self::default()
			})
			.collect()
	}
}

pub fn part1() -> usize {
	let mut stones = FlatStones::from(INPUT);
	stones.blink(75);
	stones.0.len()
}

pub fn part2() -> usize {
	let stones = FlatStones::from(INPUT);
	let stones = Stone::vec(stones);
	todo!()
}

const INPUT: &str = PROD_INPUT;

const MIN_INPUT: &str = "0 1 10 99 999";
const TEST_INPUT: &str = "125 17";
const PROD_INPUT: &str = "1117 0 8 21078 2389032 142881 93 385";
