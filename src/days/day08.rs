use std::{collections::BTreeMap, default, ops};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Vec2 {
	x: i32,
	y: i32,
}

impl ops::Add<Vec2> for Vec2 {
	type Output = Vec2;

	fn add(self, other: Vec2) -> Self::Output {
		Vec2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl ops::Sub<Vec2> for Vec2 {
	type Output = Vec2;

	fn sub(self, other: Vec2) -> Self::Output {
		Vec2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

#[derive(Debug, Default)]
struct ClusterMap(BTreeMap<char, Cluster>);

#[derive(Debug, Default)]
struct Cluster(Vec<Vec2>);
impl Cluster {
	fn get_antinodes(&self) -> Vec<Vec2> {
		let antennas = &self.0;
		let mut antinodes: Vec<Vec2> = vec![];

		for antenna in antennas {
			for other in antennas {
				if antenna != other {
					let antinode = *antenna + (*antenna - *other);
					antinodes.push(antinode);
				}
			}
		}

		antinodes
	}
}

#[derive(Debug)]
struct Map {
	height: usize,
	width: usize,
	clusters: ClusterMap,
}

impl Map {
	fn from_string(input: &str) -> Self {
		// let split = input.trim().split('\n');
		let mut lines = input.trim().lines();
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
		let mut all_antinodes: Vec<Vec2> = vec![];

		for (_, cluster) in clusters {
			for antinode in cluster.get_antinodes() {
				if self.in_bounds(&antinode) && !all_antinodes.iter().any(|x| *x == antinode) {
					all_antinodes.push(antinode);
				}
			}
		}

		// all_antinodes
		dbg!(all_antinodes)
	}
}

pub fn part1() -> usize {
	let map = Map::from_string(INPUT);
	map.get_antinodes().len()
}

pub fn part2() -> usize {
	todo!()
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
