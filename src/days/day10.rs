use super::util::vec2::*;
use map::*;

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
}

impl From<&str> for Map<u8> {
	fn from(value: &str) -> Self {
		let lines = value.trim().lines();
		let height = lines.clone().count();
		let width = lines.clone().next().unwrap().len();
		let string = lines.collect::<Vec<&str>>().join("");

		let values = string
			.chars()
			.map(|c| c.to_digit(10).unwrap() as u8)
			.collect();

		Self {
			height,
			width,
			values,
		}
	}
}

fn directions() -> [Vec2; 4] {
	[
		Vec2 { x: 1, y: 0 },
		Vec2 { x: 0, y: 1 },
		Vec2 { x: 0, y: -1 },
		Vec2 { x: -1, y: 0 },
	]
}

fn get_destinations(map: &Map<u8>, trail_head: &Vec2u, distinct: bool) -> Vec<Vec2u> {
	let directions = directions();
	let mut positions = vec![*trail_head];
	let mut destinations = vec![];

	while let Some(pos) = positions.pop() {
		let pos = pos.sign();
		let value = *map.at(&pos).unwrap();

		for dir in directions {
			let next_pos = pos + dir;
			if let Some(next_value) = map.at(&next_pos) {
				if *next_value == value + 1 {
					if *next_value == 9 {
						let next_pos = next_pos.unsign().unwrap();
						if !distinct || !destinations.contains(&next_pos) {
							destinations.push(next_pos);
						}
					} else {
						positions.push(next_pos.unsign().unwrap());
					}
				}
			}
		}
	}

	destinations
}

pub fn part1() -> usize {
	let map = Map::from(INPUT);
	map.find_all(0)
		.iter()
		.map(|trail_head| get_destinations(&map, trail_head, true).len())
		.sum()
}

pub fn part2() -> usize {
	let map = Map::from(INPUT);
	map.find_all(0)
		.iter()
		.map(|trail_head| get_destinations(&map, trail_head, false).len())
		.sum()
}

const INPUT: &str = PROD_INPUT;

const TEST_INPUT: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

const PROD_INPUT: &str = "
987123434330121232101001234730123456781067632
876076576521010345692340349823212347892398701
945087689432105676787659856714503210987445610
332196576587654989801456787609654502376530923
211543210298923215432321098128778901430121894
300692340147210106523543210039569876589836765
456781678236103267015693016543410231276745650
576890549345234178106782187612320140345654321
985098432100125089235493498109876056034765012
834127102345456978340362569018765487123876678
123236221976347869651251078729034398101985589
014545340889298958707867897430120987012834432
105965456770107843216950956541231276543124501
896872378761016930345441019876501345678023670
787901069654325321210332398545432330589012981
107821543213034321089206787638901421432103210
215430694102123475670115896129876548901210349
126989780210014984308924925014578037654321458
037878921001235675217833210123669123109452367
549865438901045102346542106548754321278501476
678954987432696201256430087239689870347699985
230143006501787349961021298101236787656788014
123272112981010458872787034010345691875107623
054387623472129867763698125676210010961234510
565694502561036789854567012980387121250129878
676783411051045672343218763901296030343278569
989872123432345891050109654812345145467303450
012763094321056700891760345765432256958912341
103450185789763211709851236876301967843211032
814321276656854345612345654954101878701208983
923434434565956745678036783063210989870345674
874532345410345832989123192178981876781456564
265101654323234901808765013265432185692387565
103216765432101267814554323476501094501893474
232109856321011876923601098789678923432102985
343898707896540945498712367765672310567891078
456789010987231234321203456894581455454986569
556776125670102343100157654503490166303890432
543895434894321765212348983212321876212761201
432104898765010894301054581200110955211654300
301256567656987105498765690341034567300563212
434567430547896234787654785652123498456767843
321798121032345375696543098743096567877854952
210899021121036789781232143456787656928923761
326765430110145678710123232109876543210010890
";
