use std::collections::BTreeMap;

struct Stones(Vec<u64>);

impl From<&str> for Stones {
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

type CountMap = BTreeMap<u64, usize>;

fn incr_count(count_map: &mut CountMap, count: usize) -> impl FnMut(u64) + '_ {
	move |stone: u64| {
		*count_map.entry(stone).or_insert(0) += count;
	}
}

impl Stones {
	fn blink(&self, blinks: u32) -> usize {
		let stones = &self.0;
		let mut count_map: CountMap = BTreeMap::new();

		for stone in stones {
			*count_map.entry(*stone).or_insert(0) += 1;
		}

		for _ in 0..blinks {
			let mut new_count_map = BTreeMap::new();

			for (&stone, &count) in &count_map {
				let mut incr = incr_count(&mut new_count_map, count);

				if stone == 0 {
					incr(1);
				} else if let Some((left, right)) = Self::split(stone) {
					incr(left);
					incr(right);
				} else {
					incr(stone * 2024);
				}
			}

			count_map = new_count_map;
		}

		count_map.values().sum()
	}

	fn split(stone: u64) -> Option<(u64, u64)> {
		let digit_count = ((stone as f64).log10().floor() + 1.) as u32;

		if digit_count % 2 != 0 {
			None
		} else {
			let half_count = digit_count / 2;

			let left = stone / 10u64.pow(half_count);
			let right = stone % 10u64.pow(half_count);

			Some((left, right))
		}
	}
}

pub fn part1() -> usize {
	Stones::from(INPUT).blink(25)
}

pub fn part2() -> usize {
	Stones::from(INPUT).blink(75)
}

const INPUT: &str = PROD_INPUT;

const MIN_INPUT: &str = "0 1 10 99 999";
const TEST_INPUT: &str = "125 17";
const PROD_INPUT: &str = "1117 0 8 21078 2389032 142881 93 385";
