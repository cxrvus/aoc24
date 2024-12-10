use std::collections::BTreeMap;
use vec2::*;

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
}

#[derive(Debug, Default)]
struct ClusterMap(BTreeMap<char, Cluster>);

#[derive(Debug, Default)]
struct Cluster(Vec<Vec2>);

#[derive(Debug)]
struct Map {
	height: usize,
	width: usize,
	clusters: ClusterMap,
}

impl Map {
	fn from_string(input: &str) -> Self {
		// let split = input.trim().split('\n');
		let lines = input.trim().lines();
		let height = lines.clone().count();
		let width = lines.clone().next().unwrap().len();
		let string = lines.collect::<Vec<&str>>().join("");

		let mut clusters: ClusterMap = ClusterMap::default();

		for (i, freq) in string.chars().enumerate() {
			if freq != '.' {
				let pos = Vec2 {
					x: (i % width) as i32,
					y: (i / width) as i32,
				};
				clusters.0.entry(freq).or_default().0.push(pos);
			}
		}

		Map {
			height,
			width,
			clusters,
		}
	}

	fn in_bounds(&self, pos: &Vec2) -> bool {
		let Vec2 { x, y } = *pos;
		x >= 0 && y >= 0 && y < self.height as i32 && x < self.width as i32
	}

	fn get_antinodes(&self) -> Vec<Vec2> {
		let clusters = &self.clusters.0;
		let mut antinodes: Vec<Vec2> = vec![];

		for cluster in clusters.values() {
			let antennas = &cluster.0;
			for antenna in antennas {
				for other in antennas {
					if antenna != other {
						let mut i = 0;
						loop {
							let dir = (*antenna - *other) * i;
							let antinode = *antenna + dir;
							dbg!(&antinode, dir, i);

							if !self.in_bounds(&antinode) {
								break;
							} else if !antinodes.iter().any(|x| *x == antinode) {
								antinodes.push(antinode);
							}

							i += 1;
						}
					}
				}
			}
		}

		// all_antinodes
		dbg!(antinodes)
	}
}

pub fn part1() -> usize {
	unimplemented!()
}

pub fn part2() -> usize {
	let map = Map::from_string(INPUT);
	map.get_antinodes().len()
}

const INPUT: &str = PROD_INPUT;

const MIN_INPUT: &str = "
....
.a..
..a.
....
";

const TEST_INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

// ............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............

const PROD_INPUT: &str = "
...s..............................................
...................w......K.......t...............
........s.........................................
.......s......w...............1...................
.........w5.......................................
.......................t.F........................
..................................................
F................................1...........d....
.........................5......................K.
............5.................R..............KZ...
....F.....q.........w..............1.....t........
............8.......I.............................
..........8.................t....................K
...........8.................5.....Z..............
.........q..............................Z...d..U..
...................Y.q...R........................
....................E.....z...............y.......
..........................................U.......
.....F.................................k........S.
............q...................d.................
.................................R................
..x....................................U.........y
.......x.........................E..M...U..d......
......z.......X............................4......
...............I....m....M......R............y....
.......z...................................k..e...
..f..z.......................................e....
...f.I..........7..u..........M................D..
.......X..I.......x.................k.............
.........X.......7....................4.......S...
....................u9...T.....3.Z....o..........6
........f.......D..3....u..................S......
...W...0.........................................D
.....................T................E.......m...
...8....Y............f........T4..................
......Y...........................................
....0.............3...............................
....................3.T.....................k.....
.......................u..............6...........
...........................6..........9........e..
..................4....7.............o..........D.
.................................M...E..o.........
...i.................O...........................Q
.....0.i.....................................m.2..
.......Y.r........7..............S..O..2.......m..
.....r......0.............O.......................
..................................Q...............
........................6................o......Q.
..W...r.................................9.........
.W.........................O........2.............
";
