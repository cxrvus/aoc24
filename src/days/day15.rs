use crate::util::*;
use map::*;

mod parsing {
	use super::*;

	impl From<u8> for Tile {
		fn from(value: u8) -> Self {
			use Tile::*;

			match value {
				b'.' => Empty,
				b'O' => Box(false),
				b'#' => Obstacle,
				b'@' => Robot,
				_ => unimplemented!(),
			}
		}
	}

	impl From<Tile> for char {
		fn from(value: Tile) -> Self {
			use Tile::*;

			(match value {
				Empty => b'.',
				Box(right) => {
					if right {
						b']'
					} else {
						b'['
					}
				}
				Obstacle => b'#',
				Robot => b'@',
			}) as char
		}
	}

	impl From<u8> for Vec2 {
		fn from(value: u8) -> Self {
			match value {
				b'^' => -Vec2::Y,
				b'>' => Vec2::X,
				b'v' => Vec2::Y,
				b'<' => -Vec2::X,
				_ => Vec2::ZERO,
			}
		}
	}

	impl From<Vec2> for String {
		fn from(value: Vec2) -> Self {
			(match value {
				v if v == -Vec2::Y => '^',
				v if v == Vec2::X => '>',
				v if v == Vec2::Y => 'v',
				v if v == -Vec2::X => '<',
				_ => unimplemented!(),
			})
			.into()
		}
	}

	impl From<Map<Tile>> for String {
		fn from(map: Map<Tile>) -> Self {
			map.values
				.chunks(map.width)
				.map(|tiles| {
					tiles
						.iter()
						.map(|tile| char::from(*tile))
						.collect::<String>()
				})
				.collect::<Vec<_>>()
				.join("\n")
		}
	}

	impl From<&str> for Map<Tile> {
		fn from(value: &str) -> Self {
			ProxyMap::from(value).convert(|string| string.bytes().map(Tile::from).collect())
		}
	}

	pub fn parse(value: &str) -> (Map<Tile>, Vec<Vec2>) {
		let (map, movements) = value.trim().split_once("\n\n").unwrap();
		let map = Map::<Tile>::from(map);
		let movements = movements
			.bytes()
			.map(Vec2::from)
			.filter(|v| *v != Vec2::ZERO)
			.collect::<Vec<_>>();
		(map, movements)
	}
}

mod map {
	use super::*;

	#[derive(Debug, PartialEq, Copy, Clone)]
	pub enum Tile {
		Empty,
		Box(bool),
		Obstacle,
		Robot,
	}

	impl Tile {
		fn expand(&self) -> (Self, Self) {
			use Tile::*;

			match self {
				Empty => (Empty, Empty),
				Obstacle => (Obstacle, Obstacle),
				Box(_) => (Box(false), Box(true)),
				Robot => (Robot, Empty),
			}
		}
	}

	impl Map<Tile> {
		pub fn move_all(&mut self, movements: Vec<Vec2>, expanded: bool) {
			let move_box = if expanded {
				Self::expanded_move_box
			} else {
				Self::move_box
			};

			if expanded {
				self.expand();
			}

			let mut pos = self.find_all(Tile::Robot)[0].sign();

			for dir in movements {
				println!("{}\n{}\n", String::from(dir), String::from(self.clone()));
				sleep(0.5);

				let next_pos = pos + dir;

				if let Some(next_tile) = self.at(&next_pos) {
					match next_tile {
						Tile::Empty => self.move_robot(&mut pos, &dir),
						Tile::Box(_) => move_box(self, &mut pos, &dir),
						Tile::Obstacle => {}
						Tile::Robot => unimplemented!(),
					}
				}
			}
		}

		fn move_box(&mut self, pos: &mut Vec2, dir: &Vec2) {
			let mut search_index = 2;

			loop {
				let search_pos = *pos + *dir * search_index;

				if let Some(search_tile) = self.at(&search_pos) {
					match search_tile {
						Tile::Empty => {
							self.move_robot(pos, dir);
							self.set_at(&search_pos, Tile::Box(false));
							break;
						}
						Tile::Obstacle => break,
						_ => {}
					}
				} else {
					break;
				}

				search_index += 1;
			}
		}

		fn expanded_move_box(&mut self, pos: &mut Vec2, dir: &Vec2) {
			// todo: movements[x].1 is constant (=dir), => turn into Vec<Vec2>
			let mut movements: Vec<Vec2> = vec![];

			// direction is horizontal
			if dir.x != 0 {
				let mut i = 1;
				loop {
					let next_pos = *pos + *dir * i;
					let next_tile = self.at(&next_pos).unwrap();

					match next_tile {
						Tile::Empty => break,
						Tile::Box(_) => movements.push(next_pos),
						Tile::Obstacle => return,
						Tile::Robot => unimplemented!(),
					};

					i += 1;
				}
			} else {
				todo!()
			}

			for pos in movements.iter().rev() {
				self.move_tile(pos, dir);
			}

			self.move_robot(pos, dir);
		}

		fn move_tile(&mut self, pos: &Vec2, dir: &Vec2) {
			let next_pos = *pos + *dir;
			let tile = *self.at(pos).unwrap();
			self.set_at(pos, Tile::Empty);
			self.set_at(&next_pos, tile);
		}

		fn move_robot(&mut self, pos: &mut Vec2, dir: &Vec2) {
			self.move_tile(pos, dir);
			*pos = *pos + *dir;
		}

		fn expand(&mut self) {
			let mut expanded_tiles = vec![];

			for tile in &self.values {
				let (left, right) = tile.expand();
				expanded_tiles.push(left);
				expanded_tiles.push(right);
			}

			self.values = expanded_tiles;
			self.width *= 2;
		}

		pub fn gps_sum(&self) -> usize {
			self.find_all(Tile::Box(false))
				.iter()
				.map(|Vec2u { x, y }| y * 100 + x)
				.sum()
		}
	}
}

pub fn part1() -> usize {
	let (mut map, movements) = parsing::parse(INPUT);
	map.move_all(movements, true);
	println!("{}", String::from(map.clone()));
	map.gps_sum()
}

pub fn part2() -> usize {
	todo!()
}

const INPUT: &str = INPUT3;

const INPUT3: &str = "
######
#....#
#@O..#
#....#
######

>>>>>>v>>>>
";

const INPUT2: &str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const INPUT1: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const INPUT0: &str = "
##################################################
##.OO.O.O#........#..O.......O......O..O..O.O...O#
#O.O#.O......#.#O......O....O.O.....#.#.......#..#
#.##O.O..#OO...O..O.O..O#.#.O.............O..OO..#
#.OO#......OOO.OO.OO.O...O.O.................O.O.#
#.#....#O.......O.#OO.#..O#.O...O.O...O....O.O#O.#
#O..O...#O.O..#OO#O....O...#....OOO...O.###.#.OO.#
#.....O.....OO..O......O......O..........OOO..OO##
#.O#..O...O.......O.....OO...O#...O..OO...O......#
#O....O...#O......O..O.OO....O..O.OO....#......O##
##.O..O..#.OO#....#..O.#......O....#.....O..O#..##
#..#....OO.##.......O..O..#.#..O.O...OO#..O#....O#
##O.O....O.O.O....OO...O.......O#..........O..O..#
#.##.O.OO..................#.##O.O...#OO.......OO#
#O....#O.....OOOO.O.#.OOO#O.....OO...OO..O....#..#
#.#O.O.......OO..OO.O..O..#.O.O......O.O..#O.O...#
##..OO.O...#.....O#O.O..O.OO#.O.OOO...O....#O.#.O#
#....O.....O#O..O.O..O...............OO.O.O.O.OO.#
#.OO.O...O..O..OO#O..#....OO...O...O.O#.O.#......#
#..OO.....O...O..#O........O..O.O.O..#...O...#...#
#O......OO.O.........O#OOO.O..O...OO..........#O.#
#.O.....#.......#O......O........#.O.O...OO.O..OO#
#.#OO...#.#.O...OO.O.....#OO.O...O.....O.O#.O.O..#
#.O........#.O..O.O..#.#O#.OOO.....O..#.....#....#
#..O.O..#O#....OO#OO.#O#@.O..O.O.#...#.O.........#
#O....O.....O...O.#........O.OO.O..#...O....OO...#
#..OO.....O#.........O#........OOO...OO...#O#O..O#
#O....O...#O.O......O..OOO....OO#O..OO...........#
#O#.O..O...OO..........O.O...O.......OOO.........#
#...O.....O...O..#OOOO..O......#....#.O....O.....#
#..O..O.OOOO.O.......O.....O...#....O...O..OO.O.O#
#..OOO..O..OO.OO..O......O...#O...#.....O.....O..#
#..OO.O..O..#O....OO.....OO..O#O..#..O.#..#....O.#
#.O.#.#..##O.....O.....#OO...#......O.OO.O..OOOO.#
#..OO.O..O#.....#....O#......#......#.O..#..O..O.#
#O#...O.O..#....O.OO#.O.#....O.....#O......O..O.##
#......##O.OO.O#.O...O..OO#.O..###........O.OOO..#
#...O..#..O....O.#..#......O..O..#O...O...O......#
#.OO.OOOO.OO..#...OO......O.O.....O...O....#..O#O#
#O..#OOOO..#.O.O.......O...O.OO...O.........OO...#
#.O.....OO..O.....O.O#..OO...O#......O.....O....O#
#O.O..O.O...OO.O.....##...O..O....#O...OOO...O.#.#
##.O..O.O.O..##O#......O.OOO.#.....O.....O..#....#
#....#...O#...#..##.O.............O..#O..O..#.O..#
#...O..O#O....O...O...#.....O#OO...O.....#.O.....#
#.#O...O.O.O..O.#.O....O..O#..O.OOO..O....OO.....#
#......O..........O..#.#.....O...O..OO..O.O.##..##
#.O..O..OO.OOO..O...O.....O....O.......OOOOOO.O..#
#...#.O.#O#....#.O....OO..O.##....O.O.#.....#..O.#
##################################################

>vv^v>^^<>vv>v>^^<<^^>>^><^v<v<^>>v^><><<<<vv<^vv<^<>>><<v<^>>>>^^>>>v^v>^<v>>^^<>>^>^v><vv>v<^<^>><<v>^v^^v<^><<<<^><^v>^<>>^^vv<<<v^^><>vv<<<^<^v<>v<<<>>>v>v^>><v<>>^v^<v^v><<v<<^>^^<<^v>^v>vv^<><v^<v<^>v^<>^><<<^>>>^v^>>^^^^<<v<^>vv<^<><<>vvv<><vv><><v><^<v^^^v><<<<<vvv<>^<^>>vv^^^>v^>^v^^^v>^^><<<>^^v>^<^<>vv>>v^v<^^<<v><>>><^^v^<^<^v<^><^vv>vvvvv><>v<^^v<^v<<v>>>v<v<<>>^<v^>vv^>v<<^^<^<>v<><^v>vv>>^>><v<v<>^^<^><>v<v<>^^<v^vvv>>><v>><vvv><>v^v^<^<<v<v^^v<^^v^>^vv>^^<>v^^^^>vv<v^^^<^^v^^<>^><v><>v>^<v^vvv^<<^>>^^>^<>vv>v>v<^><v^v^<>^<^^><>^<v>vv<>v>^>>v^^^^>>v<v<^v<<v<>v<>^<>vv<>><<>v<vv^^<v<v^<^><v>^^^<^>vvv<^<<vv<v>^^^^<^>>><>>>v>>v><^<vv^<v^<^vv>><>v^<v<>>>><<v><v^>v^v^^<v<v<^v>^><<>>>v<v^<<>^^^^^v^^^<<v^<v>>>v<^vvv^>>^v^<^vv^v>^vvv>>^^^>v^>>^><^>^^<^<<^v><^<^<<>^^<<^<vv>>v^vvvvvv><v<^^v><^><><>^<<>v^><<>v<v>><<v^^vv^^v<>^>>>>>^>>^>><<v>v<v<>^>v<v^v^^<^<vv^^>>v>v<<<>^v^<<<^>^>^vv^^>v>>>^^>><v><>^<><^>><v>>v^<<<<^v^>v^>^v^v<>v^>v<^v><<>^^<>>v<>^<>vv^>v>^v<^><v>v><^><<<^>>v^<<>>^v
<>>>^v<<>^<<v<>^<>>><^v<<<>^<^<<vvv>^<v><>^vv^<<>v^vvv<<^v><>^v>^v><v<>>>^vv^v<vvv>^<>^<v<^^<>vvv<<^>v^^^v^>>^vv>^vv>^^>>>^<v<^v<vv>^>v>v<v<>><vv<^v>^^^vvv^^vv<>^<v><^>^<^>>v>><>><^v>^>^>^^<<vv>v>v>>^<^><<^<v<<<^<v><<^v^><^><v^^<^^<<<><>>^v><^^<<>>v^^<^v><^<v>v><>><>v<>^v^<<>>v^<>><<^<<v>^<^>v<v<vv<<v><<^<<>>v>>^>v<v^><^>^><<<><v<<<^^>vv<v^^vv^>>><v^<>v>^v<<><^<^<<>>>^vv^>^^<<<<vv>v>^^<><><^>>v>v>v>v<><vvv^>>^<>v^<^v^>v^<<><vv^>^^vvv<<<<v>^<vv<<>>>v^v^v^<><<v^<^<>><^^vv^v><v^vv<<^^<v^<>v><^><v^<>^^>^<<>vv<^><>^vv>><v>v><><<vv<v>>^><<^^><^>^vv>^v^><>v>v<>^<<v<<>^>><>v^vv>vvv>v>v>v^v^>^<vv>^<<v>^>v<^vvvv<<^^v<^^<>v<>v^>>v^v^<^v>v>><<<^v>v><vvvv<v>^<^<<<><>v>v<<^>><^vvv><<>^<>>vv>^>^<v>>>vv^<v^>><><^vv>v>v<^^><<<<<<v>^v>v>^^<v>v><<v^^v^vv>>><^v<<<v^v<<>><>^v<>>^<^v^^vv>>>^>v>^vv<<vv>^><^<>>vv>>^^<^><^^>><v^vv^<v^>vv<v<vvv>>vv^v<<>v^>><<vv>><^<>v>vv>^<v>^^>>v><<><^<>><><>^^^^v><^>^^<v^<>v>^^<^<v<vv^^^<>>v^vv^>>^^^vv<<>^^>>^v>>>>^><^^<^<v<>>><v<^<^>vv^v>^^^v^>><>vv><v<>^>>^<<v^v^v^^vv^<v>^<
><<vv<v^^^<>>>^>v>^<^^^<v<vv<^^<>><^<>><^>><<>v<^>v^><<^><>><<v^>^<<v><^>v>><vv<^v>><>v<<^^v>v><>>><^>v>>^v>>^v<>v>><v^<v<<>>>>^v><<v>^<>>v^^v><>^v<v>>v><^^<vv<<<v><^><><<><^><v^^>v^>>^<<>^v<<^^^v^<<<><^^<><vv<>>^>^>^^>>^><>>v>v><v^>><<v^v<>^v>^^^^<^>>>>vvv>v^v>v>^<>v>v<v>>>^vv<>^><v>^<^<v<<<<>vv^v<v<v^<>^>^^^><>v^^^>v^<>>^vvv^^<>>>^<<<vv>>^v<><^>vv>v^^^<v^>>v>^v>v^>vv>vv>>>vv^vv>>v<>v^vv<v<^><^v^><^vvv^vv<vv^>vvvv<vvv><><>><^<<<^>vv^>>>v<<>v><<^>v^<<<><v<<^^<><<<><<^><>v<><<>v<^v<^^v^^>>><<^vvv<<>>^>><<vv<v<^<<^<^<<<>><<v>^v<>^^^>>>^^<^^^v^^v<<^^v^>v^<<^<>^><>^v>>>^^vvvv>v^<v^<^<v<>><v^^>>v^v^^v^^><v^^^^^^v^v><^<><><<^<vvv^><vv^<<^<v<v<^^^^><>^vvv<v>>>v<v<<v^>v>vv<>^>><<v<v<v^^^>v<<>v<><v>v>><><<v><<^^^v>^vvv>^<^>^v<<>>^><>>v><><^v<v<<^v^<<v^v>v^^<^v>>^v^v>^^^<<<<>v^>>vv<vv<<><>>^^^v^v^^^><><v<><v<<v><<v>vvv>>^^<v>v^^>^><<v<v>>>vv<>^<>^>>v>>^>v>^v>^v>>vv<^<>v>>vv<<v><>^^>><>vv<<><<^>v>>^>><>><<>v><<v>><v^<>v<>>>>v<<v>>v^>^<^>>^<>^v<<vv^<<^v>>v<>v>^>v>>^<>^<vv><^v><>^<<v>^v^^^v>^>v>^^v
<v^<^^vvv^v<<^<v^<^vv^v>><>><<><>^v^^v^^^<v>v<^v<v^><v<>vv^<vv<^>^>^^>v<>vv^v^^><^^>>><vv<>><v><^>vv>>><^>v>v<^<<><<<v>v<v<>^>>>>v<^><^v^<>^>^><v>^vv<<^>>^v>^^^>^>v^v<>>^<>vv^^v^^<^<vv>v>^v>>>vv^>><v>vv<>><^<><v><v>^v>>>>^<^^<<>v<^vv><<^^<<v<v<v<<v>v>^vv>>vv<<>><^<^<v<>v<>^<^<<>v^vv>v<>^vvv>^<vv><v>^^vvv^^^>^v<^v^v^<<>^vvv>^>^vv>v^>^><vv^^v>vv<<v^><v>v<>>>>>^^>vv>vvv<vv<<^v<^<^v>^<^><v>^><<^<^<v<vv<><<v>^v><<v>>v^vv<^<<v<^>^>v<vv<^v><v^>^<v<>>v<>>^v^v^><^<<^v<vv><v^^^v<>>><>v<vvv><>^vv<>^><v>^><<<^v>>v^^>^^<^v>><^^>v<^<<>^v>><<v<^>>v<><>vv>vv>^^<v<^^<^<<^^<^^><v^>v>><<^^>v<<<^<v^><<^vv><>>v<><>^>v<^<<^>^>v><>v>^vvv<<<^>^^vv^>v<<v>^^^v<<>v^>v>><>>^v<^^><>><>v<v<vv>v<<>^>>^v^^<vv<^>^^>vvv^>v^>><><^<<>^<^<>v^>>^^^<^^v<vv>^<<<<<^vvv<><>^>>><^v<>^^^^v<^>>v>^v<<>>v^v<^>vvv<v^vvv><^^<>^v>^v>>>>^^<^^<<v><<>>>>>^v><>^><^v<>v>v<^^v^vv^^^<^>v^>^^<<v>^>>>^>^^vv<v<^v^<>v<vv^^vv^<^>v>v<<>>>^><<<^v>v<<vv^<>><v<><<>v^<^vv<>v><<^<>^><vvvv>v><<><^^<>v>>^<>vvvvv<<^^^>><v^<v>><>v><><^>><>^>>^><^<^^^^v^^vv
>v^v^^v^>v>^vv<>^<^^vvvv<>^vvv^<^<><v<vv^^>v^^^>^vv>>^^<^<^v^>v^v<^>>v^>^^v>v^<v<^>^>v>^^^<v^v^<v>><v^^^v^>vv^v<>^>>^^<v><^^<><<<v^v><v^v<><^>v^<^<<>>^v><>^^>^^<v^<<><>v<>v<><^<>v<>vv<<^><vv>v>^<>^v^v<v<^^>>^><<v>^<vv^v^^v<^^^v>v<v<vv>>v^>v<^>>^<v<^^>^vv<v^<><>vv><>v<><<>v><v>>>><<<vv>^<<><^>>^vv^>v<^<>v<^^v<v>vv^>v><<v<^^><>><v^<^v^^>>>>>>>v^<v>^>^^<^><^v>^^<<>^^>>^><v>^<vv<^^<<^v>v<^>>vv<^^>vvvv>^^>v>>>^^<<<^<^<^^<^<^v>>^>vv^<v<v^v^<v<><<>v^v^<><<vv>><^<^^v<<vvvv<<<<>>^v><>v^v^<^<^<v^<>vvv^v^>^vv^>^<^><><^v>^v<v>>^v<v^^^v^v^v><v<<v^><<>v<^>^<>v><><>><>v<>v^<v^><v^^<>^^<>v><v>^^>^^v^>v>v<^^<>^>^>v^vvv<>vvv<<^vv<v^^^vvvv^^<<vvvvvv>v<^>>^^vv<^^><<<>v<^^<v><v^>v<^^v><v^<><^<><><^^<<><><<vvv^<<^^><>>v^^<v^^v^>v>vv>^v<^><>vv^^>v<>^v<^^><>^^^><^<<v>v^<<<v<>vv>vv<><^<^<^<^<v>v<v>^v^vv<v<><>v<<^v^^>^>>v>^>v^>^v^<^^^<<<^<<<>^<^v<>v<<vvv<>><<<v^<<>^v<<><vv^<>^vv<^^<<>v<<<^^^<v<v<>v^<^^vvv<<^^^>><^<>v><v<><v<><v^<<vv^^v>>v<v<v><vv<^v><v>v>>v^v^^v><^v><>^v^v^^v^<^^v>><<<vv><^<v^^v>><>^^<v^>^>>><^
^v^vv>><^><^>>>v>>>>v<>><<vv<vvv^<><>^^>v<<><><^^>>^><^><><^^^^v<^^<><>>>^^<<<^v>>>><<^^^<>v<v>><<v<v^^v^<^>v<^<v>><>v^>v>>^><vvv^^>^v<><><vv>^<<<^vv^<^vv<v>v>v>^<>>^<<v^^^v^>v^<<<v<^^^^v^v<v>>><<<>v>v^><>^vvv<v<v<<<><>><^v^>>^<><<v^v^>vvv^v>^^^><<^v>^>v>^<<^^v>v>>^<v^^<vvv>^>vv^^><<><v<^^<v><>>>^<>><v<<<^^v^v>v^>v<<<^<^<^<^v^>^^v<^<v<>><<>>v<vv^<^>v<^vvv<^>v><>>>^>vv^^>><^>^<v<><v<v^v<<>^^<^^><>^v>^^v^^vv^<>v><^v<>^v<v^v>><v^^<v^<vv<<<<^><^^^^vvv>^v^>^<><v<^>>v><<v><^^>v<^v<v>>><v><<^v^<v^><<<v<>vv>vv<^^vv^^^v<>vv^<v<vv>^v><v<vv>v<<>^^vv>v^<<^<^^v><>vv<>v<v>vv^<v<^><<<v><^v<><<<v^<>>vv>v<v>^v^>v<<<>v^v<^>^^^^^<v<<<v^<^^^<vv>^><>>>v>v<^<v^^>^><<<^^><><v<v<^v>vv^^^<^<>>>^>^v<>v^><>><v^^v>vv<>^<^vv^<<>><>>v^<>>v>>>>^^<v^v<^<v^v^^>v><>v<v^^v^>v^<^^>^^^vv^v<>vvv>^>^^<v^v<^^v>^^v^<>v<^<vvvvvv>>^v<^^v>^<^^<><<v<>>>vv^v<<^<vv>^^><<<<<>^<^v<<v^v<<<<<vvvv<v<>v><^^<>^<<>^v<>^v><>v<v>><^<>><v<><v^>>^<>vv^<v^v<^vv^<><<<v^<>><^>v>v^>v>v^>>^^^>>^>v<<^<>v<>^^v>>>>>v^>v<<^^^^<^^^v<>^>^^v^<<><>><>^<v>v
v>v^^^^><v^v<vv>v>vv^v>v><>^>v<^><^>>v<>v<<<<v^v^^^^<<<<v^>^<v<v>><v>><^>v><<v><v>^vv^<vvv^^<<<>^<^<v^v>^^<^>v>>vv^vvv<v^><^^v><v<<^<^^<<^^><>>^vvv^<^<>v^>>^v><v<^^<v^<v>v^>^^<<v^<>^<<^<vv^^v<<^v<^vv><^^^v>^vv^v>v>^><^>^^^^<>^^^>v>>v<^v^v>>v>^<<>^<^^<>v^<><<^>^v^v^v^>v<<^vvvv^<<^^<^<^vv>^v>v<^<><v>^><^<><v<<<>>v^<<^<^^^^v>v^<^>v^>>^v^<>v>^v<>>>v>^>^^>>v>>>^^^^><^v<^>vv^>v<>>>v^v^vv^<>^><><^<>^<><vv<v^v>v<^v><>^<<v>>vv<><vvvv>vvvv<^v<<<>vv^<^<^v^<>v^v<v^<<>^v^<vv>>v>><vvv<^>>v^^v>vv^vv<v^<v^v>^^>vv^^>^<>^<^><><<>^>^^<<<>^v>v^<>v^>>><<>>^<>v<<v<^v<v>vvvvv><<<v<^<<>>v^>v^^<^v><^>vv^>vv^<><<>vv>vv^<>^^v^v<vv<>v>^<><^<<>^v<v<^^<>^<>>^<<>^^^<<v>^>><v<^^<><v><><v<v>^>v^<vv<<>>^v<>>^>v^v^>^vvv^><<v<<v<^<v<<><^^^^<<^^<<^<v>^>^>^<>>>^^>v<<^<>v<^<^<>><>v^>v>>>^^^v<^^><^><>v<v>>v^^vv>^v<v<^<^<^>^v>>^<^vv>^^<^^^<^>^<^<v^^<><v^><v^v<v<<^v^<>v>v^^v>^vvv^<^^v<^<>vv<v<v<^v<><^v<>v^><<^vv<v^<<<v<<v<vv<^v<>>^^<v<v>^v<<>v<^^<>v<^^^^<vv^^v<^><<v^>>^<<^<<<<><>>^v<<<>^><<^>vvv^v<^v<v^vv<vv^<>^^<^^^<<^<^<><<v
>vv^>^vv^<>^>>>^<^<vv<v><>>v>^>>v^<><<<^vv^<v^^v^vvv><<^>>v<^v>><^><vvv^>^v^<><<v<^>>^<>^v><v>^<>><><<<^>^<v^<<v<>><^^v><>^<><<<<v<^>>>v>v>v^v<^<^v>^>^><vv<>>><><^v>^v>>v^^>^<v<><^>>>vv>^v>^<>>v><^>^>^v>v<>^>vvv<>>^>vv>^v<><<<<v>>>v<<v^v^<v^vvv>><>^^<vv<^>v>vv<<^^<>^^v^>vv^>>v><><v><^><><v><<^><<^v^>><><><><^^^vv>vvv>v<>>v<><>^<><vv^v<^^>v>><>^>^>^^^v^>v>><><v^vv^>^^<^vvvvv^^<vv^<<v^v<<^^^<<v^^><^v<^>>>^v>v<<<^^><<v<>^vvv^<>^^<^^^v^<^v<<>>^v<<^^^vvv<v><vv><<vvvvv^^vv>><<^><<vv^><<v^v^>^^<>>v^<<>vv>v^v>><^<^>^^<><<v>^^^v<^<<^v>>^>vv^<<>>^v<>v^><<v^vvvv^^<<vv^<><^^^<>v>>^><<^vv><^><<v><<<<v>v^v<^^<v^v^>>><^v<^><<v^vv^^>v>v><>v^<v^<^<<v^^<<^^<>><v^<>^><<>><<>v>><<>^>>^v<^<^>vv<<<^<v>^v<^<v^^>v>>v<v^<<<v^<^v>v^v<^><vv^v<<<<^v>^<>^<<^>v<vv<<^><v<>^>^>v^<<^^v<<^>^>>>v^^^>>v><vvv<^vvv<><^>^>^v^<<^<^v>v^^<><vv^^><^v<<vv^^^<v^vv<<>><<<^<>><v<v<v<<^<<v>>><^vv^v><v^v<<<>v^^><>^<v<vv^^<<^<>v>v<^<vv>^>^v><v>^<^<<^<^^v<v^^v<^><v<<vv>vv<vv<<<vvvv><>^^v>vv>^<<^vvv>vvv<^^<^<<>v^v<v<>^vv><<>^>v>>^<vvv^>
vvvv^v^^>^>v^<>><<>>^>>>v<><>v<v>v>v><<<^^<>^^v^^^<>vv<v^^<<^v>>^vv^><^^>>>>^^^<<^<<>^>>vv^<>^^^>>vv<>>>^v>v^^<>>>>^^vv>^><^^vvv<^>^^>v>^<<^^><><>>v<>>^<v>>><^<<v>vv>v><^v^^v<<vv>^<v><<>vvv^>^>^<<^>>>^v^><>^^^v<<^v^>>v>v><>vv^v><vvv>vv^<<v<<^^<v<vv<<v>v<^^>>v>>v><>><>v><>v>vv<^^v><v<vv^v<>v<v<<v><<<<<>><>>v^>^^<^<<v<^^^>>^>>v>>>^v>v>^><<<^^>v>^<v^><v>v<>>^<<>^<>v<<^^<<<^<>><^<>v<>vvv<v<<><v^<v<>^><^^<^<^>>>v^>v^>v<v<v<^><>v<>v>^v>v<>><^<^<<><<^<<<>vvv>>>^^^<<vvv>^>^<v<>v^v<<^vvv><>>v>v><vv^vv^^<>^<>v>>^^<<>>><^>><v^<vvvv^>v<^v<<<><>vv^v^>^v>>>^v^<<v<>^>>^>>v>^vvv^v^^<<^^<>vvvvvv>>>v^>>>^vv>^v<<><><v<<^<>vv^^v^<^vv<<<><^v^vvv>v<>>vv>>vv<v<^^<vv<v^<<>vv^v^<<>><^v^^^>^v<<^><>^>><>v>^^<v^><v^>vvv^<>vv<<^^>><^<v<v<>vv>v<v^v<><vv^<<<>v><v>^>^>^<^v>^<^v<^>^^<v^<<<>^<^^v><>v<<^vv><<><vv<v<vv>v^v^v>>v>v>>^^><v<^v^<v>v<>vv^<<v><>><vv>>v<>^>^vv><>^^>v<<^v<v<v>>vv^>><<<v^<><<<^^><<^v^v^<^^<>v^>>>><>^<vv<v>^<v<<^>v^>vv<<<<v><v<v^>>^>v<vv^>vv<^^>^v>>^^^v>>^^v>>^>^<v<^<<<^<v><^>^^<v^>^>><>>>^v>>^<^<^
>><>>>v^><^>>>^<^>v^<^^<><^<^>v<><^><^>v^<><<v>>>>>^>vvv<^><>^>vv>>^^<v<<v^^>>>>^<>><v^>>^v^^^^^>>><<<<>>v<^^v><>>vv^>>vv^v^v<^<^<>>v><>v^^>v><>v>^^<^v<^>>>>>vv<v^vv^<><^^^v^vv>v><<^<v<vv^v>vv^^<^vv<v<v<<<>v<>>vv<><<>v><^<<vv^^v^^<v^v<vv>><^<><^>v<^<><><<^<><^v^<^^v<^<vv^>^<^vv^v>^<^<^<>v^v^<v<v^><><>v>>v<><v<^v^^<<<^>^^^<v><>><><>>>v^>>^>vv>>^vvv^^><^<<<>^^v^v^>v>^v><>^^vvv<^<^^><v^>^>>>^vv^><v<<<<^><<>v>><v<>>v<>^^^>><<<<v^<>^>^v^^<<v^>^^<v<>^v^<>^^<>><<^v<<^>^v>>^><^^<^>^^<<^^<<v<v^<vv^v>^>v^><>>^^^vv><^>>>>>^^v>^<>^<<^^v^^>>v^<<><^v<^v^v>v>v<><^vv<vv>v^^^<v<^^<^<^<^<>^^^<vv^<>v<><<><<v<v<<>v^>>v<>^<^^^^^><<<^>v<v><><>><^<<^^>^>>vvv^><>>><^>>vvv^^>><>^>>vv>v><^>^<v<v<^v^>vv>>v^v>vvvv^^^>vv<<<v>><v<v<>^>v<v^v<><v<>vv^^<vv><^^^v>^v>>v>^v><v<^<<v>vv>vv><<>>>><^v<<^>v>>^<vv^<v<^<v>v>vv^v^v>>>v><<^v^v^v>vvv<>>>>v>v^<<<<^^>^^>v^vv><v^v<vv<^<^^>>v^v><<><v><vv<<vv<v>^<>v^>>v^v>>>>^<>><>v^<^>v^<^<>>^vv<><v<<^v><>^>^<^v>><v^<>v><^v^>^^>v^>^vvv^<<>v^^^>>^<<<v<^v^^><^v^>vv>^vv^v^<>^>vv>vv>>>^>v
^^^^^>>><<^v><v>>v^>^<^>>>>v^>v>vv<>><^^^v^>^^^v<<><^v^^<^<<^<v<v<v>>v^><v^>^^^^><><<<^vv><<v^^>^v^<>v>^>^>>v>v^v^<>^v^>v^<v><>^<v>v><v^^>^><v^v<>^<v><>^>><vv>^v<>><v^v^v>vv^^<vv>^<^>><^v^^^vv>v>v<<^v>>>^^>v^>^<><^^>^<<v<v>^<<><v><>^^<<>vv>^>vv^vvv>><>>v<>>^vvv><>^^<vvv<<>>><^<v<v<<^>><<<>v^>^<v<>^^<^^<>vvv^v<^^vv<^^^>vv>>v<>v^v<<<>v>>^>v><vv>v^<vv><><>v>>vvvv^>v^v^vv>vv>^>^^<v^^>vv^^v>>^><>>>v<<<>^>^vvv^v^v><>v^^^>^>>v^><><v<^v>v>v>v>vv><<<><<^<>^vvv<<^<><v>vv>^^<<><^v>v>v>v>^vvv<>^v<v^<^^^>>vv^>v<^<<>^<>vv<v<^<vv><v^>^<^>^vv>^^>^>>>>>^vvv>^<<<v^v^v<<>v>^>>>^^<^>vvv>^v>^<v>v>>v><<^<>><^<^^^<>^<v^<><v^vv>^^><^^^<>><v<^>><v^^^v<<>><>><>><>>^^v<>>v><<v^<^^<vv^v^v<v^<v^v><<^v^>v>><v^><v<><^v>^<v^<<^>>^>vv>>v<><>v<v<<>><vvv><^>vv><v^v^^^<>v>v<^v^v<v>v<v>^^>v><><<v>>^>v<vv<>>^v>><^v^>><<>^>>v^v<>vv>^^^>^^vv^<^>v^^^>^<<vv<<^>^>^><>vv<^vv<^^<v<v<><<^v><^>>><^>^v>^v<<^^<<>^<v^v>^v<>>>^>v>v<vv^<v^vv<>>>^vv>><vv>^<vv^^>^^>>>vv>v>v^v>^vv<><>^<>>v<^^<>>v^>>>v>^>>^>v<<<>v^<^vvvv>v>^<v>^<^vv>>><vv^v
<^><>v><><>^vv^^v^<^<>>^>>v<><<<^>>^<vvvv<>vvv><>v>^>^<^^v<>^<<v^v>vv<v>v<<^v^<>>v>v>^>v>^^v^v^^<^v>^v<<><^v<<<<><<^><^>^v^^vv><>^^vvv<vvv>^>v^v^<>vv>v>^^^v>v>^<<vv^v<>^<^<vvvv^<>v><v^<<<>>vvv^>^^>>v<<v>^^<^v<<v>>v<>><>>^><v<v>>^><^<v^vvv^^>>>>>^>>>vv<v>^>v>>^<<>>v^v^^vv^>v<<><^><v><>^^<>^vvv^v<<^<^v<<><><v>v>>^>v><v>^>v<v^<v>><vv><><><v^v^^>^v<v>>>vv<^><<<v<^v^><<v><<v<^^v>>>vvv^>vv^<v^^><v^<<>v<v>^<>vvv<<^><<>^<^><^><><>>v><>>^vv<^^^^vvv<^v>>v>^<>>>^vv<>^^v<^v<<^><v^^^v<vvvvv><<vvv>v<<v><v<<v<vvv<<^>v^^v^>>^v^<^<vv^v<vvv^<<<><>>^>v>^v<^<>vv>>^><v^>^<<><^>^>><^<><^>v>^^^><v^<><v<v>^<><>>^^>^vv<v^>><>^>>><><>^<<<<v^<<^<v^>^<v<v^^<v<<<v>v<><v^<<^>^^>^>^><^vv>v>>v><v^v<^^v>>v<<>v<^^>v>vv^<v^^<v><v^^<vv<^<v><<vv^v>>^><vv<<><^>^>>v^vv><><^^<<>^^^<>^>>>^v^<>v<<v><vvv<^>v>^<>v^^<^^<v<^^<^v^<><<^<^^<vv^<<<v^><v><<><>><<<v>>v^>><vv<vv^<^>v>v>vvv><^<^v^^<>^<<>>><<^>>vv>v<v<v>vv>>>v^v><<v<>vv^>v<>>v^^<v^v^><^^^<^<>^><<<v^>^>^<>v>vv<<>^v>^<<><^^vv<v>>vvv>>^<<^>^vv<^>>>vv^<^<vv>v<v<<^>vv^<^v>>v>v^
^<^><v<<>v>v<<^<>^>>v^>>>vv<>v^^<vvv><<v^<<<v>>>vvvv><v<^<<^<>^vv<v^vv>>^^^v^<vv<<v^<>vv^<^vv<<>^^><v<vvvv<<>>v<^<<>^><<><<v<><<v>^><^v^v>v>v<>^vv^v<>>>^v^^>v^>v^v<vv^^>v^><^^^^>>v<vv<^>v>>^^><^<^>>v><v<><<<<v^<>^>><v>>>v^>>v>v^>v<vvv<^^>v<>^<^<^v^^v><^>>v<<v<<<<>v<^^vv>^vv>^^v<>^><>v<<v^<>v>><<^^<v>>^^<<<^v<v<<v^<<v<^v><>^<vv<<>><<v^^^<v>>^<v<^>^>v^>^>vv<v>^v<<>vv<^vv^<<<vv<<><v^^v>^^vvv^v>^<vv>><><<<<v^v^<v^v^>^vvv^v^<<>^>^<<<<^><><>>vv<>^v^<>>vv<>^^><>vv>v>vv><><^<<^<^vv>>v>>^<^^v<v>^^<v<^v<v<vvv>^><>v>v^v>>>vv^<^^^v^<^^>vv>^vvv^v^^v<vv^^<^^v><^v<<<^>v>vv^>^^<^<>vv>^<v^>vvv>^<>>>>vvvvv^^^<^v^>>>v<v>>>v^>^^<<><<v><v^>>^<^vv>^<<v^v^v>v>v<vv<<^<^^^<><^^><>v>^>v<^<^^^^^<^^^>^vv<>>>v><><>v<<<>v>vv><v><><^>^<<v^<^vv<>v^><^vv^^<<>v<<<>v><<>^<^>^^^><<v^>><<<vv^^v>^^<^^vv<<<vvvv^><v>>>v<<<<>^^<<<<>v^<<>^<v^v^>v><v<v><><<>>^^<>^v>^v>>vv^<<<v<><<^v>><v>><<>>^>vv>v>v>>v<>>^><><<><<<><>v<>>^^vv<>^<<<^^><>><v^<v<^>^<v<^^<<<>v>><^<<>v<v^>>^^>^^vv^<^>^^^v^^<v<<<^<vv^<<>>vv>vvv>^>v^vv<><^><>v^^<>vv<
^v^v>v^><^<<vv^<>^vv^v>>^>v>v^vvv>^>^v<<<<>^<v>>>v^vv>v>>^vv^<><<^><>v>>>v>v>^>v^vvv>^>^^<<<<^<vv^<v<<><>^^v^>vvv>>>^<>>^v<>^^>>>^<<>^>v>>>v<<><v>><<v<<<vv^<><v<^v>>>^vv>>>^>^^<>v^vvv<>vv<v<^<^vv<^^>>>v>v^^^v><^>>vv>>v<<vv^>v>^><<v>^vv^v<><v>^v>vvv^<<<^^>>>>^^v^vv><<<<<<<><^^v^^<v^v^>^^v^^v<>><v^>>v>>v<^^vv>^<^<<v>^<v<<v<>>vv<^<vvv<^v^<v><^<><<<^>>>><>^^v>v<^v<^<vvvv^^<v<^<v<^<>vvv>><<<vv^<vv><vv^v<><<>^v^<^<<v>^>>><^>^^>^<^>^vv^^<>v<^>v^^^^^><<<vv><>v<^v^<>v^>^><>v<<^v>>>^v<^v<^<^vvv>vv<>^v^>>^v^v>vv<v>>v>v^v^>>v>><><v^v<^>vv<<vvv^^<<vv^^<^v^<>v>^<<^>^<<<>v><^<<v>><^vv>^>>vv^vv^<^^^<^v>vvv<v<^v^<>^>>>^v<<<>v^^<^v<>^^^<^v^<>>>v<^<<^vv<>v>^^<v^><<vv<^v<<^v<v<v^<v<<^<<v<^>>>v><>^<><<^^^<^>^>vv<^vvv^><v^v>^^<v^<>^<^<^<<<^v>^^<v<vv>vvvv^<<<>^<^>v^^>^>>vv>^v^><>>^<<^^v>>v<^v>v>^^^>v^^>^>>>>><^^>>v^>^v<>><<vv^^v<^>v^v^vv><<>v^><<>v^>^^^vvvvv^^^>^^>v<>>>^><^v^<^>v><>>>>>vv^^<^v<<<<<<v^^^<^>v<^><><<^v^^<v><<<v>>^^<<v<>v<<>>v>v<<>v^v>vv<>>^><^<>v<v<<^>v^<v<^^^v>^<<v>>vv>>v^<<><<<<<^<<>vv^><^^>>
<><>^v^^^<vv><<^>^><^<><<v>^<v<>><v><<v>>vv^<^^^<v<^v>^^<^>v<v>v^v^<^><>>^>^^^v>^v^>>><v>^^^^vv<>v>>>^v<<<v<<v<^<v^v>>^>>>>vv><><^^<><vv<^v<vv<v<vv<vv>v<>v<^v>>^>v<><v>^^v^^>^^<<^><<v<<<<v<v><^^v^<^<<<>v>v^<^v<>>^>vv<^>^<>v>v^<^vv^<>>^^v>><vv^<>v>^<<<><^<>v<<v<<>^>><vv>^>v^><<v^>>vvv^v>>>^^<v<<v<v<^>^vv^v>>^^v^>vv>^>>v<>v<>v>><^<v^v>^<^^v<<>v<vv><vv^^>>v>v>v<><^v^v<<>^v>>>><v<>vv^><<>v<v^>>v^^v><<^<vvv^><<^><v<^^v^>^>v<>^<<^><^<>><>>v^<>^v^^<^<>>>><<<v>^^^vvv<<vv>v^<vv^^<^<v><v<vvvvvv>>vvvv^<v<^^>v<>><^<v^>^<^<^<>v<>^>v<>^^>v^^>vv>><v>^>^v^<>vv>>>^>vvv>^<>^<^<>^>v^v^<vv<>vv^<>^<v>><>^^>>vv^<><vv^^v><^>^^vv<>^<<<<vv<<vv^<v<v>^vv><^<<><v<vv^<<v<><>^>v<^^vvv^<^v>v^>>>^<v><^v<<v>^v<vvv>^^<^^vv>>v^^>v<<^>v>><^<v><v^>^>v^^<<v^vv^^vv>^^^<<<v><<<^<^v<v<^>v<^>>v^<<^<^>^v><<<>>>>>><>v^v^><>v^><^^^v>>v^v<<^v^<vv<>>^^<v^vv>v<^<v>>^<<<>><^<>^v>>><>>>^>>^v<<vvv^<<^<v>>^v>vvv<^^^^vv>v<>><>^<<><<vvv>^<^<^v>><>v<^>v^>v>v<<<>vv<>^v^^>>>>^<<>v><>>>^v^>^><<<>v<>><^>>^^<vv^^>^<^vv>>v<v>vv<<><^><<^v<>^v<<v<
v>^><v^^vvv>>>^<<<>v<v^v>>v>^<^>v^<<^>>^^<^vv>>>><>^><^^^vv><v<^^<<^<<>v^><<v<<v><>v>v<>^>>v^<<^>^<<v>^><^^v<<^>>vv>^^^^^v<^>>>v^v<^v>>>^^>v^<><<vvvv<^<v^>^<><^v^vvvvv^^<<>><<>>v^vv<^v<>>^<<vv<vv>^<<v>^^>v>>^<>><v>>v^v>v>^><<<>>>><<<<<v>^v>>>>^vv<<<^>^>vv><v^v^^vv^v^<v^<^>v><><v>><^^<^>>v^^<^><^<<v<<v>^v>vv^<^><<>v^>><>>v<^>^><vvv<>vv<<vv>^v>v<<^>^<v><<<<>^<^^v<vv<>>><^vv^v<<<>^^v<<><v^v^v>^<v>v^<<v^><v<<^v^^>v^^^^<>^<<>v<>>^>vvv<^^v><^><<vv<<^<<vvv^<>^^<<>v^<^^<<<>^v^<^<><v>v<^v>v<^v>^^<<^^><<<v<^<v>^<<^^><^>><^^<v>v<^^<vv<<>>^v^>^<^<^>v>>^<^>v>vv^<v>vv>vv<<<<<<<<v^<^<v<^<^v<>^<v>^v>^>vv<<^^<v><v<^<v<^>vv>v>^^v^v<>>^^>>>v<^^>><>>vvv^v^<>>vv^<>v^v^v<^>>^>><^><^<v^^vv<<><<<v<^v^vv<<<>vvv>^v^<>><v<v<<<^^^^v^>^><>^v^v>vv^v><^>^<vv^>vv<^<^>>vv<^^^><v^v^>^>v>><<v^v>vvvv^<>v<^>^vv^^^v^<^^v>v>^v>v>>^<v^>v>>>v^^<^v><<<>v<v^v<v^v<vv<<>vv^^<>vvv<><>^<v^>^v<^<^>v>^vv><>vvv^<>><<^v<>^^^<<<vvv^^>^vv<<^>><^v><>vv>vv<^<vv<^v><^>vvv<<^>^^<<^<^>v<^><^^v^^<v<<v>^v<^v>^^<<^^<>v^v^v^<^v^>>><>v^v^v^<<^<><v
<vv>^<v<<^<v>^^v><>vvvvvvvvv<vv<^v<>v<v><v>^><v<^^v^<^^v^v><v^^^<<<vvv<v^^^><><<<>><^><^vv<<><<^vv<>^v^<>>v>^vv^>v>v<^vvv^><<vv<vvv<v^^>v^v>v>v><v^><v^<^v<v^>>^<>v<>>^<^<^^><vv^^^>>v^^><<>^<^v^^vv^<<vv^<>><v^v^<^^v^v^>>^^<>v<^v>v>v<vvv>^^v<><>>vv>v<v>><^v>^>^>v<^<>^v<<vvv^v>^v>^^>>>vv<>v><vv>>>>^>>v^<<v^v<>v><><^^v>v<v^<vv^>>><<>>>vvv<>^<<v<<<<vv<>>>vv><^vv>><<v>><^v>^<<^v^vvv^v^>^>^v<v^<>v>><v<^v^>><>>>vv^>^>v<^><^^>>><^^v^<>v^^<^>>^<vv^>>v><^^>^^>^^v^>>v^<>^^<<<>><><>>^>^<>^><^><<^>><v>>vvvv^^^vv<><<>>^v<<<<>^^v><^v<<v>vv>>v<><>^^^>^<>v>>v^v<v<>^<<^>v^vv>^^v>v<>^>vvv^^v>^^v<^<>^v><<v^v^<>^^><^vv<<>>>^>v^>^>>><<<v^<<v<<<^v><^^>^v^^>v^^v<^v<>^>^<v<>><^>>>^>><<v^^<<<^><<<v><^>><^>>><v<>^><^vv<^>v>vvv^<^><^v<<<^v^^vv<^<v<>^vvv<>v>^<^><>>>^v^v^^>^^<<>>>^<>>^>^><><><^^<^^^<>>^<^>^><<vv>^<v><^>^^v><>>^>^>v^v<<<v<>v^<><^>v>v>v<>^^>vv^><v>>^v^<>^^>v<><><^<>><^><^v<<<<v^vv><^^v<<^v<v<<<v^<vv^<<<vv>v<<<>vvv<^^<<vv^>^<^vvv^^v^<>v>v^>><>^v>><v^^^><^^<<>^<>^<v>v^<<vv>><<^>><>^v>>>^v^vvv<v^vv^>^>v<
^>v^^<>^>v^^<>^<<v<><v^>^><<^<>^vv^vvv>>v^vv<<^^>^v<v^<vv>>^^^^vvv>>^^<>^v><<^^>>^><^<^>^v>^v<>^v<<v<<v>^><<v<v^vvv>v<v<<^^>>>^vv^vvv>vv<>^>v<vvv<^^vv>^v^><v^>vv><>><^>^^>^v^^<<^vvvv<v^v>>^>><^>>^<<v^>v>^v><v<<<v>>v<^<>^^^^vv>v>><>><v><><vv<<v<<^<^<^<^v<v^^>>>^<^<v^>>^v<><^<>^<vv<^<v<v^>>vv^>>>v<v^<^v^v^^>v><>^^v<<^^><^^>v^>v^^>>v^^<vv<v^>>^v>><>v>>>^v<<>^vv<^^>>>><v<>^>vvv>^v^>v^>^^<^>>>>>v^^<<>v>v<<>>>><>v<<>>>v^v<><>><<>^vvv<^^^>^v<^<v<^<><<<><^^<<>>v^^<v>^vv<^^v^^^>v^<><<^^v^^v<<<>v>^^^<>v<v<^>>>v^^>^v^^v>^^^v<>^^^>>v^^>>^v^<v<<>>><>><^>^><^v<<<^<<vvv^>>vv<v<vv^><>>^>^^v>v<^v>^^><>>v>>v>v>^^<>^vv<^v^vv^>vv>v<^v>><^v>vv^^>^vv<v<^>v<^v<<<^>^^>^<>>^<v<^>^<^>>v<vv^^<^<>^v<^v^<>>^^v^vv^>v<v^<<vv^<><^v><><^^<v^v<v^><<<>><><<^<^^^vv^v<v<><vvvv<>v<>vv^v^v>v<^v><^>v<<v>><^v<v<<^v<><^^^^v>v^v>v<^^^v^>v<vv>v<<vvv^<^v^<^>><<v<^v<><v<<vv^<>v<v<<>>v>v>>v^^v<<^v>><^<<<><>^^v^^<v<><>^<vv<>v<^<><>>v>v<^<^>><v<<><<>>>^v>^<v>><^>^v^vv<<^>^^v<^v<^^<<>^^vvv<>v<v<>>><v^^v^^v^^>^^<^^>v>^^<<>^>^^^><v>v>v>
v>v^vvvv^<<<>v<v>v><<<vv<>^>>v<><<<vvv>v>v^^<<<v<^v<<v<>v>v<^v>v^^><^<^^>><^^><^^^><^^v^v^<>>^<^v^^^>^<>^>v<<<vvv^<<^>><<>^<><<v^^<<<vvv><^v>vv<^>v>v>^vv>^<vvv<<v>v<<<>v<>vvv<v^^v<><v<v^v>>v^<<^v>^v^^>>v<>^<<>v>><^^^><^<<><v>><>^^>>^vv^<^^<v^^><<^v^^v<>v<>vv>v^>^>^>^<><><^^<^v>^><^^v^>v>>>^<^^^>vv^^>^>^^<vv><^><><^<<>^v<^<v^<^<v<v<^v<<<>vvv^>^^<><^^<<v>><^>v<^^^>v<v^><v>>>>^v<v>><^v<>v><^^vv^^<v<v<v<v>^<^v>vv><v>^<^<v^v>^<<><^<^v<>vv<<v>v>^<v<<^v<<^<<>^<<v<^>vvv^>^>v<>>^<v^^<^<>v>>v<>^v>>v<v^<>^>>^<^<v^vv>v>>v>>>v>^<<><^<>><><><vv>^<<<<v<<<v^^>^<^<vv>v<>^^>v^^<>v>><^>><vv<vv>>v<vv^v><<<<^>^<<<>><v>>^^^>^>^^v>>>^<<v^v><<^v><>v>^>>>>v^^^<^v>v^>>>^<>^<vv<>v^<><>>>^>^>^^^^<<<>>v><^^<<^><>vv>^<>^<><^vv<^>>vv^^>v^vv<v^<^^^^vv>vv>vv^vv^^^vvv^<>^v^<vv<><^^<<><>v^<^^^<v^<<>>^><<v<^v^><<^>v>>v^<v>^<>>>^v>^<<<v^^v<^>vv^^v<^v^vv>><>><^>^^>v>^^v^<^^v^^><>v<<<v>^>vv^><>v^v^><<>^^<v^<><v><v><>^v><>v><^^v^<v>^><<v>v<v>>>^v><<>^>^^>>v<v<v<vv<<<^>><^<vv<^v>^vvv>>v>>^<<>><v<^>vvv^v^^v><v<><<v<v>v^v^v<<v^
>>^v<v^v><>^^>v<>>v<v^^<vv>^v^>v<v^>>^>><^v>^^>^<^v<>v>>^>v<v>v><vvv^><vv^^^<v>>>>^<><v<^<>v>^v<^v^vv<>^<v>^vvv^>^><v<^v^<<<^^^>>>^<<^v^vv^>>^v^v^^<^v><<v<>^v<v>vv<v^vvv^<<>vvvv<^^v<^<vv<><v<^<>v^^<<>v<<>v^^<<><<<<>v^>^v<^^^<^^<^<^^><>>^<v>^^<^vv<>v>>^^^<>><^<^^v<vv><v^<><<^v><><<^v^>^>^^^<^^><vvv>^>v^<^v>v^>v<v<v^>><<v^^><^v^<<vvv^<>vv>^v<><v>>><>^v<v>^<<<v<>v>v>^<<<^^v<>>v<>v<>v<>v>^<^v><^^v<<^^>v><<<<>^^<vvv>>^^><<v<>><<v><^v^^vv><vv^>^^><v<>^v>^>vv>^<^><>^<v>v>><<^><>>>v^v>>^>v^><^<v<<><<>^vvv<v<<^>vv>v^<><<v^^<v<^<<><v<<>>^><v><>^^^^<v^v<^><>^<^<^<v><<v>v<^v>v<>><>>vv<v^>>v<^<<^^>^<<^^^^<v<v>^v^<^^v^vvv><<v>>>v<<v^><<^>>v>>v<<^^>>v^v><<<^>^^<vv<^^^<<vv>>^vv>>^<vv><<v>vvvv<<<<>vv>^<>>v<^<v>^<v^>><^<^>^vv^<vv><<<<<<<<>v>>>^v<>>^>^<^^><v>vv^v>^<^^<<<v^>^>^^v><vv^^<><v>><<^<<>v<vv^v<vvv<<v<^<^^v>^v<^^<v^>vv<>vv>>^>v<<>^vv^<>>^>vvv<>^^^v^<v^><>>vv^vv^^>>v^v<^^<>>v<>v>>>^<vv>^<><<>v<><>vv^v^v>^<<v^<^>><vv^v>vv>v>>>>^^^vv<^>v>vv^<<><>^^><^<<^>vv^>^^<<>><<><>^>^^<>>v^v<v^v^vvv><^v^^v<>vvv
";
