use std::ops;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
	Empty,
	Obstacle,
	Visited,
	Start,
}

impl From<u8> for Tile {
	fn from(value: u8) -> Self {
		match value {
			b'.' => Self::Empty,
			b'#' => Self::Obstacle,
			b'X' => Self::Visited,
			b'^' => Self::Start,
			other => panic!("invalid char: {other}"),
		}
	}
}

impl From<Tile> for u8 {
	fn from(val: Tile) -> Self {
		match val {
			Tile::Empty => b'.',
			Tile::Obstacle => b'#',
			Tile::Visited => b'X',
			Tile::Start => b'^',
		}
	}
}

// todo: convert to struct with 1D tile Vec and size Vec2
#[derive(Clone, Debug)]
struct Map(Vec<Vec<Tile>>);

impl From<&str> for Map {
	fn from(value: &str) -> Self {
		Self(
			value
				.trim()
				.lines()
				.map(|line| line.bytes().map(Tile::from).collect())
				.collect(),
		)
	}
}

impl From<&Map> for String {
	fn from(map: &Map) -> Self {
		map.clone()
			.0
			.into_iter()
			.map(|row| {
				row.into_iter()
					.map(|tile| u8::from(tile) as char)
					.collect::<String>()
			})
			.collect::<Vec<_>>()
			.join("\n")
	}
}

#[derive(Debug, PartialEq)]
struct Transform {
	pos: Vec2,
	dir: Vec2,
}

impl Map {
	/// wanders through the map and returns whether or not there is a loop (true) or an escape (false)
	fn wander(&mut self) -> bool {
		let start_pos = self.find_start();
		let mut pos = start_pos;

		let dirs = DIRECTIONS;
		let mut dir_i = 0;
		let dir_l = DIRECTIONS.len();

		let mut visited: Vec<Transform> = vec![];

		loop {
			self.set_at(&pos, Tile::Visited);
			let dir = dirs[dir_i];

			let transform = Transform { pos, dir };
			if visited.contains(&transform) {
				return true;
			} else {
				visited.push(transform);
			}

			let next_pos = pos + dir;

			// println!("{}\n", String::from(&*self));

			if let Some(next_tile) = self.at(&next_pos) {
				if *next_tile == Tile::Obstacle {
					dir_i += 1;
					dir_i %= dir_l;
				} else {
					pos = next_pos;
				}
			} else {
				return false;
			}
		}
	}

	fn is_in_range(&self, pos: &Vec2) -> bool {
		let Vec2 { x, y } = *pos;
		x >= 0 && y >= 0 && y < self.height() && x < self.width()
	}

	fn height(&self) -> i32 {
		self.0.len() as i32
	}

	fn width(&self) -> i32 {
		self.0[0].len() as i32
	}

	fn at(&self, pos: &Vec2) -> Option<&Tile> {
		if self.is_in_range(pos) {
			let Vec2 { x, y } = *pos;
			Some(&self.0[y as usize][x as usize])
		} else {
			None
		}
	}

	fn set_at(&mut self, pos: &Vec2, tile: Tile) {
		if self.is_in_range(pos) {
			let Vec2 { x, y } = *pos;
			self.0[y as usize][x as usize] = tile;
		} else {
			panic!("map index is out of range: {:?} = {:?}", pos, tile)
		}
	}

	fn find_all(&self, target: &Tile) -> Vec<Vec2> {
		let mut pos = Vec2::default();
		let mut positions = vec![];

		for row in &self.0 {
			for tile in row {
				if tile == target {
					positions.push(pos);
				}
				pos = pos + Vec2 { x: 1, y: 0 }
			}
			pos.x = 0;
			pos = pos + Vec2 { x: 0, y: 1 }
		}
		positions
	}

	fn count(&self, target: &Tile) -> usize {
		self.find_all(target).len()
	}

	fn find_start(&self) -> Vec2 {
		*self
			.find_all(&Tile::Start)
			.first()
			.expect("map needs a Start tile")
	}
}

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

const DIRECTIONS: [Vec2; 4] = [
	Vec2 { x: 0, y: -1 },
	Vec2 { x: 1, y: 0 },
	Vec2 { x: 0, y: 1 },
	Vec2 { x: -1, y: 0 },
];

pub fn part1() -> usize {
	let mut map = Map::from(INPUT);
	map.wander();
	map.count(&Tile::Visited)
}

pub fn part2() -> usize {
	let original_map = Map::from(INPUT);
	let mut wandered_map = original_map.clone();
	let start_pos = original_map.find_start();
	wandered_map.wander();
	wandered_map.set_at(&start_pos, Tile::Start);

	let mut loop_count = 0;

	for obstacle_pos in wandered_map.find_all(&Tile::Visited) {
		let mut map = original_map.clone();
		map.set_at(&obstacle_pos, Tile::Obstacle);
		let is_loop = map.wander();
		if is_loop {
			loop_count += 1
		}
	}

	loop_count
}

const INPUT: &str = PROD_INPUT;

const TEST_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

const PROD_INPUT: &str = "
..........#..........#.#.......................................................................#...............#......#...#.......
........................................#..#..........#.........#..#..............................................................
.........#.......................................................................................................#................
..................................#.#..................................#..#.......#.....#.#............#.#........................
...#.......................................................#.........#...................#.................#......................
...............#.........#...........#................................................#..............#...........#.............#..
...................#..#..................................................................#........................................
........##..............................##................................#.....................#...#.............................
................................#...................#...........#.#...........................................................#...
......#......#...........................................#............................................#...#..#....#...............
............................#...#......#.....#.........#...#..#..............#....#...................#.........................#.
....................##.#........#.........#.............#......................#.#....#..............#........................#..#
#....................#......#..........#................................................................................#.........
..............#.............................................................#..........................#........#.....#..........#
.....................#........#......##.#.........#..........#.##.......................................#....#....................
................................................................#..#...........#.........#........................................
...............#..........#..#....#.#.#..................................#................................#..................#....
...................#....#.........................................................................................................
....#........................................................#......#......#.......................................#............#.
................................................#...#...........................................................#.................
..............#............................#.#.........................................#....#.....................................
...........#....#......#..................................................................#.....#..........#.............#.#......
...........................................#........#.#.........#.................#.....#.........................................
.............#.........#.........................#......#.....................#.#.................................................
.............#.....#................#...................................................#.........................................
..#....#............#................#...........................#..................#............#................................
#............................................................#......#..................................#...............#.......#..
.........................#...#.................................................................#..#...........................#..#
.................#.......................................#...............................#...#...#............#..........#........
......................#.........#...............#......#................................#.#....#.......#..........................
.#......##.................................#.................#...#.................#............................................#.
..#..................#.....#.#......#................................#..#...#......##....#..........#..................#..........
.................................................................................#...................................#............
..............................#..#...............................#....#.............................#.......#..................#..
..........#........#.....................................#..............#.....#..#................#........#.#..................#.
....................................#....#..........#........##...................................................................
.......#............................##.......#....#................................................................#.#............
.......................#......................................................#.................#.................................
.......#...................#..................................................................................................#...
.........#.....#................................#...............#........#.....................................#................#.
..#...........................#.....#..........#............................................#........#........#.#.................
...#.........#.............................#......#......................#.......................................#.......#.......#
.........................#.........#........#....................#..................#....#...#.................#..................
...............#.....................................................#....#....^............#.....................................
.......#.......#.....#.............#...........#...#..................................#...#.#....#.........................#......
.............#....................#..................................................#.........................#..................
......................................................##..........................................................................
.....................................#.....................................................................#............#..#....#.
#......................#....................#...#..................#....#.......................................#........#........
.#.....................................................................#...........................#....................##........
............#............#.........................#.........#.....................#..#........#...............................#..
.......#..#...........................................................#..#........#................#................#.............
..............................................................................................................#...................
..#............................#.....#.............#...............#..................................................#...........
..#........#..........................................................................#...........#.....##...................#....
..#.........#...................................#.................................................................................
...#.#......................#.......#...........#.........................................................#..........#............
.............#......#............#............................................................#......#............................
..............#......#...................................................................................................#........
..............#.#............................................................................#......##............................
...........#....................#...................#......#..............#.........................#.....#...#.............#.....
.................#......#................................................................................#...............#........
..#...........................................................................................#...................................
#.............#.......................#..#.....................................#..#.....#.....................#...................
.........#..#...#......#.......#................................#..............#.........#..........................#......#......
.....##...........#................................................................................................#..........#...
.#.........#....................................................##.............................#...#.........#....................
............#...............................#.#...............................................#...............................#...
...............................#..................#...........................#..............................................#....
.#..........#....#.............#.....#.....................#.....................#...................#.........................#..
.............................................................#................##..............#...................#...............
..........................##.........#................................................................................#...........
........#..#...........................#...................................................................#......................
..#.....#...................................................................#.....#....#..#.......#.......#.....#.....#...........
....................................#.......#....#......................................#........................#...........#....
...............#..............#.................#...............#....#....................................................#.......
....#..................#..........................................................................#.......#..........#............
..#...................................................#...............#...............................#......#................#...
.......#............#............................#.......................................#...#....................##..............
...............................#.......#................................#..............#....................#.....................
...........................#..............................................#..........................................#.#..#.......
....#........#....................#...........................................................................................#...
#..................#.....................#....................................#.#............#...................#................
....#........#...#............................#...#.......................#...............#........#....#.........................
.............................................................................................#....#...............#..#............
.........#..........#...........................................................#.#........................#..................#...
.....................................#................#.........#.#..........##......................#..#.........................
.....#..#.....................................................................#.#................................##...............
...............................#....#.#..................................................#.#.........................#........#...
#.......................................##.....#..........................#..#....................#.........#..........#..........
......#.#.............#....##...................................................................#................#....#......#....
..................#...#........................................#........................................#...#.....................
....................#...#...........##...................#..#....#.............................#......#............#..............
.........#.........................................................................#..........#...................................
......................................#...................................................................#.#.#..................#
.......#...#.....#........#..............#................#.........................#.#...........#.#.............#.......#.......
...#.................#......#...........#.............#.....................................#.....................................
.#...........................................................................................................#....................
............#..........#.......................#...........................................................................#....#.
......#....#...#.........................#......#......................##....#.......#.....................#....#.................
....................................#..........................#.......#.......#......................................#........##.
...................#..................................................................................#..............#.#..........
....#..........................#.....................#.#.........#...#......#..#....#.............................................
..................#......#................#........................................#...#........#.................................
...............................#.........#..................................................#.....................................
..................#..............................................#...............#...........#..#.................................
...........................#........................................................#........#....................................
.................#.......#.........##.............#...#....#......................#.......#.......................................
...........#...............#........................................................................#.#...........................
...........#..........#..........................#.....................#....#.........#...................##...#.........#........
.......#.............#........#...........................................#.................................................#.#...
.....................#..................................#.#......#...................#.........................#...#....#..##.....
.......#...................#.............#..#..................................................................#..................
..#........#........................#............#..#...#...........................#......................................#.#....
.............................#..#.......#.........#.....................................................................##...#....
..............##................#........................#..................................................#.....#...#...........
.............#..............#......#..............................................................#...................#...........
#...#............#...........##.............#..........................................#..........................................
..........................#.....#...................#....................#................#.......................................
.....................................................#..................................................................#...#.....
..................#.#........#....#.#.#..#...............................................#.....###..#.........................#...
.#...#.........#..........#.......................................................................#.#............#............#...
...#..............................#.............................#.#............................................#..................
#................................................#..#..........................................................#...............#.#
........................................#.............#..............#...............#.........#...#..............................
..............................#.................#..................#................#.....#.......#..........#....................
............#.......#...........#.........................................................................................#.......
#.#..............................#................#.#...................#.....................#.....................#......#......
.....#..#.......##...........................................#............................#.......................................
.......#..#..#.........#..........................#.#....#..........................#...#........................#.....#..........
";
