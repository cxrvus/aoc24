use Xmas::*;

#[derive(Debug, Clone, PartialEq)]
enum Xmas {
	X,
	M,
	A,
	S,
}

impl From<u8> for Xmas {
	fn from(value: u8) -> Self {
		match value {
			b'X' => X,
			b'M' => M,
			b'A' => A,
			b'S' => S,
			other => panic!("illegal char: {other}"),
		}
	}
}

impl Xmas {
	pub fn successor(&self) -> Option<Self> {
		match self {
			X => Some(M),
			M => Some(A),
			A => Some(S),
			S => None,
		}
	}
}

struct Matrix(Vec<Vec<Xmas>>);

impl Matrix {
	fn height(&self) -> usize {
		self.0.len()
	}

	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn at(&self, x: usize, y: usize) -> Option<&Xmas> {
		if x >= self.width() || y >= self.height() {
			None
		} else {
			Some(&self.0[y][x])
		}
	}
}

fn get_matrix() -> Matrix {
	let matrix = INPUT
		.trim()
		.lines()
		.map(|line| line.bytes().map(|char| char.into()).collect())
		.collect::<Vec<_>>();
	Matrix(matrix)
}

pub fn part1() -> usize {
	let matrix = get_matrix();

	let mut count = 0;

	for y in 0..matrix.height() {
		for x in 0..matrix.width() {
			if *matrix.at(x, y).unwrap() == X {
				count += count_occurrences(&matrix, x as i32, y as i32);
			}
		}
	}

	count
}

fn count_occurrences(matrix: &Matrix, x: i32, y: i32) -> usize {
	let directions = directions();
	let mut count = 0;

	for dir in directions {
		let mut previous = X;
		for i in 1.. {
			let (dir_x, dir_y) = (dir.0 * i, dir.1 * i);
			let (x, y) = ((dir_x + x) as usize, (dir_y + y) as usize);
			let pointer = matrix.at(x, y);

			if let Some(pointer) = pointer {
				let successor = previous.successor().unwrap();
				if successor == *pointer {
					if *pointer == S {
						count += 1;
					} else {
						previous = pointer.clone();
					}
				} else {
					break;
				}
			} else {
				break;
			}
		}
	}

	count
}

fn directions() -> [(i32, i32); 8] {
	[
		(0, 1),
		(1, 0),
		(1, 1),
		(0, -1),
		(1, -1),
		(-1, 0),
		(-1, 1),
		(-1, -1),
	]
}

// pub fn part2() -> i32 {
// 	todo!()
// }

const XXINPUT: &str = "
XMMAS
XMASS
SAMXS
";

const XINPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const INPUT: &str = "
SMASMXMXSAMXSASXXAMXMAXXXMSXMASAMAXMASXSMXXMXMXAMXXXSAMXMXMSMXSXMASMSAAAXMASXMSMMSSXMMSSMXSMXXXMAXMMMXMXAMXMXAXMASXMMMXAXMAMSAMXAMMAAXXMASAM
AMASAASAMAMSAMMXMSXSAXSMSXMAMXSXMASAAAAAXSXMXMASXMMMSXSMMXMAXAAASMMAMXXMSAMXMASXAAXAXAXSAMXMSMSMSXSAMMSMXXAXMSMSMSASXXXXMXSASXSXSXSSXSAMXSAX
MSAMMMMASAMMAMXSXXAMMMXAAXMSAMXAMASMMMSMMSAMXXAMAAXASMMAASXMSMSMMSMXMAXMXMAMMAMMMMSMMSMXSAMXAAAAXASASAAAMSMXAMAAASAMAASXSAMAMXMAMAMAASAMAXMM
XMASMXSMMXXSAMXSMMMMXAMXMSAXAXSXMASXAXAMASAMXMSSSMMXSAMSMMAXXAMXAXMASMASASMMSAMAAMAXAMMAXMMSMSMXMASAMMMSMAAMSMSMXXAMMMMAAAMSASMSMAMXMXAMMSMS
MMAAAAMXMAMAAXAMMAAXSXSAMXMXMMMAMMXXXMXMASAMXMAMAAXXMMMAASXMMAMMSSSMMXASASAAAXSMXSAMSAMASAMXAMMXSXMAMXXXMMXMXAXXMSAMXAMXMXMXASAMXXMXMSXMXAAA
AMSSMMSAASASXMSXSXSMMMSAXXXASXSMMSXMMSSMASAMAMASMMMXSASMMMAXMMXAMXAAXMXSAMMMXMSXMAXXMAMASMMAMXMASASXMMMMAXAASMMAXMAMSASMMMMMMMXMASXXMAMXSMSM
SAAAAAXMSMMASAAXSAMMSAXMASXMAAAXAMAMXAAMXMAXXSASAAAASXSSSXXXXMMSSSSMMAXMAMXSAAMMXSSMMXMMMAXAMXMASAMMSAAMMXMXAASXMXMXSAXMMAMSSSXSAMXMMAXXAAAX
AAMMMMSMAAASMMMMSAMAAMXMAMAAMSMMMSAMMXSMSSSMAXAXXMMMSXMAXXSMSMAXMAAAASXSXMAXMSMAAMAMSMMAMMXMXXSAMXMAMMXSAXSXSMMMMMSAMXMASASAAAAMAXAMSASXSMSM
MMXXXXAXMMMMAMMMSAMXXXXMAMXMXXXAXXASXMMAMAXMSMSMXXXXXASMMMSAAXSSMSMMSMMMMMMSXXAMXSAMAASMSSXSAAMXMMMMSMXMASMAMXMAAXMASMMASXSMMMMSSSMMSXMAAAXX
XSXMASXXXXSMSMAASXMSMMMSMSXSSSMSSSMAAXMAMXMXMAXSSXSMSXMSAAMMMSXMAXXMAAAXMAMMAXMXMAASMXMAAAAAMSMAMAAAAMXMXMMAMASMSSSMMMMASASXXXXAXXMASAMMMMSA
XMAXAMAMSAMAXSMMMAAAAAAAXAAAAAXAXAMXXMSXMAAAMMMAMASAMXASMSMXMXAMAMAMASXSXSAXMXXMAMMMMSMMMMMMSMMMSSSSSMMSAMSMSXSAMMAMAMSASMSSSMMMSXMASMMXSAMM
SSSMASASMAMAMAMMMMMXXMXMMMXMXMMMSXMSXXMASXXXSAMAMMMSXMAMAAMASMMMAXXMMXMMAMXAMMMMSMMSAMXXXXAXMAXMAAMAAAAMXXAAAXMXMXAMMMMXMXMASASMMMMXSXXAMASX
AAASMMXSMSMXSAMXASXASMSSXSAXASAMAAXAAMSMXSAASXSAXXSMMSSMMMMMXAXSXSAMXAXMXMSAMAAAAAAMASAXASAMMMMMMSMSMMMSMSMSMMMSMMMXMMSSMSSMSXMASAMXMMMMXXMX
MXMMSAMXAXAASXSSMSAMXAAMASASXMASMSMMMAXSAMXMSXSAXXASAAMAMAASMMMAASAMSMMSAMXAXSMSSSMSMMMSMAMMASAASXXXXAAAAAXXXAAASASXSAAAXAAXXAXMAMMAMAAXSSSM
XASXSXSMMMMMMXXAXXMAMSMMAMMMMSMMXAXXXMXMMMMXXXMASXAMMXSMMSMSASXSMSAMXAAXAXSSMMMAXAAMMAXMXAXMXSSSSXAXSMXSXMXSMMXXXXMAMMSMMSSMXXXMAXSASMMMAAAX
MMSASASAXMXSMMMSXSAMMMAMXMMAXAXSXMMSSMAXXAMXMXXAAMMMMXAAAMXSAMAAXSASXMMSSMMMAXMXSMMMSXSXMASXAXXXXXMAXMAXASASXAMSMSMSMMXXXMXXAMMSAMSASMMSMSMM
SXMAMMMXMAAXAAMAAMMXAMAMSMXAMMXMAAAAAMAMSMSAMXMAMMAAASMMMSMXASAMXSXMASAAAAAMSMSAAXSAMMSXMAXMMMMSMMXASXMSAMAXMXXAAMAMXMMMMMMMXSAAAMMMMXXAAAXX
MXMAMMSSSMXMSMSMMMXMMSAMXMASMXAXSMMSSMMMAAXXMAMMXSMSMMAXSXASMMXXXMMSMMSSSMMSMAXMASMAMAXAMMSMMAAAAXMASAASAMSMAMSMXMAMAMASMXAMMMMMMMXMASXMSMMX
SASASXAAMMAAMAMMMMAAASMMSMAMAXMMMAMAMAXSMSMSSMSAAXAXASAMXAMXAAXXMAXSAAXXAAXSMSMXMXMMMSSMMMAMMSMSSMMMMMMMAAXMAMASASASASMSXMAXAXXXSSSMASAXXASX
SASASMMSMMMSMAMAASXSMXMASMAMSMMASAMAXXXMAXXAAXMASAMXAMMXMMMSXMXSXSASMMSMSSMMAAMXMASXAAAMAXXSAXMMXMXMAASXMMMXXSASMSXSASXMXMASXMMAMAMMASMMSAMS
MAMAMAAMAXSXXXMSMSAMXSMASMMXAAAAXASXSSMSASMMSMXMAMSMXXAXAXAXAMAMXMMMXAXAAAAMXMMMMAMMMMMSASMMMSAMMMAMXSXXAAAMXMMSASAMXMAXAXAAAXAAMAMMASAXMMMA
MAMAMMMSAMMAMXMMAMMMMAMASASMXMMSMMMMMXAMASXMAMAMXAXXSXMMSMMSSMASXMMXMASMMSMMMXXAAMSAMAMMASAAASAMASAMMXXSSMMXAAMMMMXMASXMXSXASXSXSASMASMMAAAM
SAMAXAAXXAMAMAAMAMSAMXSXMAMAXSAMASXAAMXMAMASASMSMMAXMASAAXMAXSAMAMSAAMSMMXAXAAXSSMSMXAMMAXMMMSXSMSAASAAMXMASXSMASXMMXASMASMMAAAASMAMXMASXMXX
SAMXSMSMXMMASMSSMMSASMXAMSMSMMASAMXMSMSMSSMMASAXAMAMXASMSXMXMMMSSMSXSAMAMSMMMSMMAAXASMSSMAXAAMMMMMMMSAMSAMXSAAMAMAMXMASXAMASMSMMMSAMXSASMSMS
SAMAXAAXMMSMMXAXMASAMMSXMAAMAMXMXMMSAMXAAMXMXMMSXMSSMMSXMASXMAXAMXMMMMSAMAXASMASMMXXXAAAMMXMMSAAMAMMMAXMAMXXXMMMXAMXMASMAMMMAAXAXMXSMXAMASAM
SAMXSMMSAMASXMASMMSXSXXAXMSMXXMXXAXSASMMMSAMASXMMXXAAASAAXXAXSMMMAAXAAMASXSSMSAMXSMSMMSMAAMXMSMSSMSAMXMSSMSSMMXSMMSXMMSXSSXMSMSMMMSSXMSMAMAM
XMMXSAMXXSAXMAXMXXMASMSMMXXMMMXMSXAXXMXMASASASASASMSMMSMMMMSMMAXSSXMASXXMASAMMSSXMAXAAAXMMSAAMAXAXSXMAMXAAAMAAAXASXXSMMAMSAMMAXXAXAXXXAMMMAM
SXSASXMMXMASXSSMSMMAMAXMAXSAAMSXSMSMSSXMXSAMASAMSAXXSAMXAAAMASXMMMASAMXAXMMXMAXMAMMMMMMXSASMSMSMAMMXSAMSMMMSSMSMMMAXXAMMMMMMMMMSXMASMMMMMSMM
AAMXSAMXMASAAAAAAXMSXSXMASXMASAAXAAAXMASAMXMMMMMMMSMMSSSSSMSAMMAAXMMMMSMMXMAMSSSMMSSSSXAMASXAXAASXMASAXXXAXMMAXXXAMMSXMAAXMAXSAAXSAMAAXAAAAA
MXMXSMMASXXMMSMMSSXXAMXXSXXXSAMXMSMSMSSMMSASMMSAMXSAAAMAMAXMASMSMMSAAMAAXMXAMAAMXAAXAXMXSXMXMSMSMAMMSMMMMMMSMSMXSASMXMSMSMSASMMSMMASXMSMSMSM
XMXMXXMMSAXXMXAAXXAMAMMXMAXMASXAXMMXAMAAMSMSAAMASAXMMSMAMMMMMMAXMASMMSSSMSSSSMMMMMSSMXAAMMMAXXXAXMMAXMASMMMAAAAASAMXAXXMAAMXMXXXASAMMMXAMXMA
MXAAMMSAMXMSSXMMSMXMAMAMMMMXMAMXMXAMAMXMMXASXMMMMMXMMMMXSXAAXMMMSAMAMXMAMAAMSAMXAMMAXAMSMAXMMMSXSXMASMMMAASMSMMMMAMSSXMMSSMSMSXSMMMSAXMAMASM
MSSMSAAAXXXAAXSAMXSMSXSAASXMXSSSMMSSXMASXMXMMSXSASXSAAXSMMSXSAXAMAMMMXMAMMXMSXMSSSXXMMXAMSMMAASAAXSASAAMSMMAXXAAMXMXMASAMXAXXAMSAMXSXXMAMMMM
AAAAMAMSMXMMMMXAMXMAAAASMSAMSXAASMAMASASAMXSAXAMMSASMSMAAAAASAMXXAMAAMXSSXSMXSMAMMMSMSSSMXMXAXMSMAMASMMXAAMXMSSMSAMMSAMASMMMMMMSAMAMSXSAMXAX
MMSMXSMMXMXSAMXAMSMMMSMMXMAMXMXMMMXXAMAXASASMMAMMMXMAMXXMMSXXASMSMSMMSAAXAXAAXMAMAMAAMAMAXXXXXMAXSMAMASMSMMXXMASMXMAMMSSMXASXXAXAMSSSXMASMXS
XXXMAMXXXXAAAMXAXXAMAXAXXSMMMSAMXMMMSSMSMMASXMMSAMAMMSXMAMXMSMMAAAXAAMMMMMMMMMMMSASMSMAMMMMXSASMSMMXSAMMXAAMSSXMMMSAMXMAMMMXXMASXMXAMASMXMAM
ASAMXSXXMMMSAMSSSSXMASAMMAAAAMASAMAAMAMAAMAMAXAXASXSAAASXMAXAAMSMMMMMMXSASAXXAAAMXMMAMASXAAASAMXSAAAMAMAXMMMAAASXAXASASAMASMMMMAMSMMMAAAAMAM
MMASAMXMMSAXMMSAAMAMASASXAMMXSAMAXMXSASMSMMSMMSSMMAMMSXMASXSMSXAMAAMXSMMASXSSMSMSASMXSAMXMMXMXMAMMMXSAMSSMXAMSXMASMXMASASASASXAAXXAMMMXSMSAX
XSXMASMSAMXSASMXMMXMXSAMASMSXMASMMSASMSAMAXAXXXAAMAMXMXMASAMXMXMSSMSAAAMMMMMAAAASAMXAMASMXMMXSMSSXMXMAMXAXMSXMASAXSAMAMMMMSAMXSSSSXMAMMMAXXX
MMXAXXXMASAMXAXMASMXAMAMXMASXMASAMMAMAMAMXSASMSMMSMSAMASAMMMASMMAMAMXSMMAAAMMMMMMAMXMSAMMASXAXAXXMSASAMMMMAAASMMMASXMMMSAXXXMAMMAAMSASMMMMSA
XMASXMSSMMMSSXSASAMMMSAMMMAMXMAXXMXMMSMSMXMAMAXXAAXMASMMMSMSASASASASAMASXXMSSMXSSXMAMMXXSASMXMMMMMSASASASMSMXMXXMASAMSASMSMXMXSMMMMMASXAAAAM
AMAMAXMASMXAAASMMMAAXMASAMAMXSSMSSSXXXAASAMAMXMMSSXMAMAAAAAMAXMSASMSASXMMXSAMXAXMASXSAMMMASASAMAAMMAMASASAXXMMXXMASMMMASAAAAXXMXMASAMXMXMAXX
XMASAMSMMSMMMXMASXSSMMMSXMASAAAAAAMMMSSMSMSSSMMAMXAXSXSMSSSMSSXMXMASAMMMXMMASMMXSAMAMMSXMXMMXAXSMSMXMMMMMXMAMAMMAMXXAMXMXMSASXSASMXMSSXMXSXM
XMASAMXXAXXXMASMMAMAMSAMMSMMMSMMMSXSAAAXXAAXAMMASMSMMAMMAAAAAMXMXMXMMMSMAASXMASXMMSXXAAAAAMXSSMXAMMMXXSXSASXMAXMAMXSXSXSAMXAAASAXXAMAAASAMAS
XMASMMXMASXSXXXXMXMAMMASAAAAAAXXXXAMMSSMMMMSSMSXXXAAMXMXMSMMMSAMXSAAAAAMAMXAXSMAAAMAMXSXSMSAAMAMAMSMSASXSMSAMASMASXAMMMMAMMMMMMSMXMMMSMMASAM
SMASMSMSMMAAMSSMMAMAXSSMMSSMMXMXAMXMXMAMXAMXAASAMSSSMSMSXMMSAMXSASMSMSSSSSSSMMSSMMMSAMXAXAMMSMXMAMAAMXMAMMSAMASMAMXMASASMMSASXAAXASAMAXMXMMS
XMASMXAAXMMMMAAASMSMMMAAAMAXXASMSAASMXASMSSSMMMAMXAMAAAAASAMMSMMASAMXMAXMAAXMXAXXMAMXMMMMMMMAMAMXXMMMAMAMASAMXMMAMAMXMAXXAMASMSSSXSASASMXMAS
SMMXMMMMXXXAMMMMMXAMASMMMSAMMAMAASASAMASAXMAMAXAMMMMSMMMMMAMXAAMXMAMAMXSMMMMSMMMSMMSASAMASMSASASXMSASASASMSAMSMSMSMSMMSMSSMAMAAAMXSAMASAXMAS
SASMMAAXXMMSSSXSXSXSASASAMAXMAMXMMXXMXMMMMXXSXSSMSXMASXMXXMMMSSSMSSXMSAMXSMAAAAASAASASMSASXSASXSAMSAMAXASXSMMXAAAAAAXXAAAAMSMMMSAMMSMAMMMMAS
SAMXSMSMMAXAAMAMAMXMASAMXMAXMAMSXSMMSMXAMXMASAAMASASASXMSMASXAMAAXMASMASAMSSMSMMMSMMMMXMASAXAXASAMMMMSMMMASAMXMMXMXAMSMMMSAMXMAXMXAXMXSXXXAX
MAMMMMMAMSMMSMXMAMSMMMMMSSMMSAMSAXAAAAMMMXAAMMMMAMAMXMXAAXAXMXSMMMMXMAXMXMAXAXMAMASXXXXMAMMMXMMMAMXAAXAMMXMSMAMSASASAMASAMAXXAMXSMMXSAMXAMXM
MAMASASAMMAXAAASAMAXXAMAMAMAMXSMSMMSSSSMMSSSMAXMMSAXAAMSMSMXAMXXAAXSMSSSMSASXMAXSASXMXMAMASMAAXSSMMMXSSMXMMAMMSMAMAMXSAMASASXXMAXAMAMASAXXSA
XAXXSASXMXSSMSMSASMSSMMMSAMMSMMAXXMAMMAAAAXAXSMSASASXXAAAAMASXMSMSMSAAAAAMAMXSMAMAMASMMMSMASXSAXAAXSXAAXXXMAMMAMAMSMAMASAMAXMAMMSSMXSAMXSXSS
SSXMMXMAMAXXMAMXAMAAXXAAXMXMAMMAMXMXSXSMMSSMMMAMXMAXASMMSMSAMAMXAMXMMMMMMMAMMAMXMAMMMAAAAMMXMMMSMSMSMMMMMMSSSSXXAXXMAMAMAMAMMAMXAAMMMASXMAMA
AAASXSMXMSSMSSSMSMSMSSMSMSASAXMXSMSMSAXXXMMXAMMMASMMAMSMAMMXSMMMSMASXSXAXXASXXXXSASXSSMSSSXAXAAXXXAXXMAAXXAMXAMSSSXSXSSSXMAMMAXMXSMAMMMXMAMM
MSMMAAXSXMAAAAAXAAXAXXAAAXAXXMMMMAAAMAMSMMMSXSXSAAAMXAXXASAMXAAAAAMAAMMSSSMMMXAMMAAAMMMXAAMMSMMSSMMMXXXSSMSSMXMAXMMMMMXAASASXSMSXMMSMASMSSSS
MAAMMMMMASMMMSAMMSMMMMMMSMAMSASXSSMMMAMAXAAMASMMSSXMMMMSXSXMXSMSSMMMMMMAMAASAMXXAMMXMAMSXMXXAXAAXAXMMSAAXAMXMMMSSMAMAMSSMMXSAMASAMAASMMAAAAM
SMXXAXXSAMMSAXMAXAAMASXAMMAASASAMASXSXMSSXXXAMMAMXMXAMMAMXASAMXAAASASAMMXSMMMAXAXSSXSMMMAXASXSMSSSMAAMMMMSMAXXMMAMXXAMAAASASAMAMAMSMSXMMMMXM
MXMSSSXMASAMAXXSSSSMAXMASXXXMAMXMAXMASAAMMSMSSSMXSASXSXSXSAMXSMXMMSASASXMAXXXSXSXMMMMMASMMASAXXXAXMMMSAXAXSXMMXSAMXXSXMSXMASAMXSAMXXSAMXSXMS
XAAMAMASAMMSXMSXMAMMSSXSAMXMMAMMMSSXASMMMAXAAAAAASMSXSAMASXMAXMAMXMAMAMMAMMMMMMXXMAAASASASAMMAMMXMXMMSXMAMAXAAAMXXMAMMXMXXASAMAXAMXSSXMAMASA
SMMSAMMMMSXXAXXAMAMMAMMMSMSASXMXAMSMXSAASMMMMSMMMSAXXMAMAMAMSSSXSAMXMSMMAXAXAAAXAAXXXSASMMASXAXSMSAAXMAXSAMXXSXSAMSAMASXSMAMMMAMSMMMSXMMSMMM
XXAXMXMAMMMSMMSMMXXMAMAXAMAXMMSMMSMSMMXMMAAAAXXAXMMMASAMASXMXAAAAMMSMMASASAXSMSSMMSMAMXMASMMMMSMASXSMSXMAMXMAMAMAXMASXMASMSMSMSMXASAMXXMAAXX
XMMSMASMMAMAAAXMXSMSSSMSXSMSMAAMXAAAXSMMSSMMSSSMXAXMAXASXSASMXMAMXAAASAMASMMXMMAAAAMXMASAMAAXXAMXMMXXSMMAMAMAMAMMMMAMAMAMMAAAAAAXXMAMASXSMMM
MAMAMMXAXSMSXMXSAAAXXAAMXMXMMSXMMMXMSAAAXAAXAXMASASMMSAMASAMXMSAXXSMMMXMAMMAMSSSMSSSXMAMMSMMXMAMAAXMASAXMSXXXSXSXMXSSMMSSSMSMSMSMXSAMXXAAXAA
AASASAMSMXAXXMAMXMMMSMMMXSAMMXAAXMAXMXMMSMMMMSMMAAMAXMAMMMMMAASXSXMMAAXMXMMMSAAXAXXMAMXSMXMAASXMSSMSXSAMMMMAMXASAMAMAMXAAAAMAAAXAMXXMXMSMSXS
SASASMAAAMMMMMASMSSMXAAXAXAMASMMMSMSAMSMSXMAXAMXMMMXAXAMXASXMMXXMASAMXASAXXXMMSMMMSMMMXSXAMMMSAAAAXAAMMXAAXSSMXSAMXSAMMMSMMMXMMSMXMASMAMAMXX
XXMAMXSMMMXMXAMSXAAASMMSMSSMXSMSMXSAMXAAXASXSMSAMXMASMMSSXSXSAXXMAMXMXMMAMSAXAAAMMSAMSASMXMMXSMMMMAMXMMSMSMASAASAMXMAMXXMXAXXXMAAASAMMAMAXAM
SXSAMXXMXSASASMMMSMMXAXAXAAMXSAAXSAMSSMSMMMMAMSASXSXMAAAMAMASXXMMSSMMSXSAMXMMSXSAASMMAAMAXMXAMXXSXSXSXAXAMMAMXAXMSXSMXSAMSMSXXXMSMMASMXSSMXS
AAMAMSXMASASMMAAAAMXMSASMSXMAMSMSXXXAMAMASAMAMSAMXMMSMMMXAMMMMMXAAAAASAMSSXMMMAXMAMXXMMMSMSAXMASMAMASMSMXMMXSMXMAMAMAASAMXAAAMSAMXMAMXXXMMMM
MSMSMSAMMMMMXSXMSASXMMAXAXXMAXAXAMXMMSAMMMAXXMMSMMSAXXXXSMSMASMAMSXMMMAMASASAMAMXMMSMSXXAAASMXMAMMMAMAMXMXMAXAAAMMSMMMSAMXXMMMAASMSXSXSAXAAA
XAAXAXXMXAXSAMXAMXMMAMAMAMMSMSMMAAMMXMXSXSSMXMAXXAMMSXMAXSAMXSASMXXMASMMASMMAXMAXMAAAMSSMXMAMXSSMMMMMMMASAMXXSSSMAAAMAMMSSXMASXSMAAAMAAMSSSM
SXSMXSAXSMMMASMXASAXAMAMXMAAMXXXAXAAXMSXMAXAAMMMMSSXSAAMXSMSMXAXAMXSASMMXSXXXMMASMSMXSASXXSXSASAAMXMAXSASXMAMXAMMMSMMAMSAMMSASMAXMMAMAMMAAMX
XAXASAXAMXAMAMXMSXMSXMXSSMSSSXXMASXXSXSAAAMMMSAAAAXMSASXMMXSAMMMSMMXXXXSMMASMSMAMAAAXMMMMXAAMXSSMMSMMMMASMXXSMASXXAXSXSMAMAMAMMAMXXMAXSMMSMA
MSMMMMXMSSXSAXXMMAMAMSAMXAAMAMXAAXMAMAXMMSXSASMMMMSAMAXMAMAXXSMAXSAMXMMSAMXMAAMAXXMSMSXMXSMXMAMAMAMASXMXMAXMXXXMASXMXMAXMMSMSXMMSXMXMMXAAMXM
AXMMSXMMXAMSAMXAXAMAMSAMMMMXAXMSSMMAMXMXXXMMASAMXAAMMAMAXMXSAMMSSMSSXMASAMXXSMMMSMAMXAAXAXXXSXSAMXXAMAXXMSSMAMMMMMMMXXXXMXXAMMXMSXMAXAXMMSXX
SXMASAAMMSMMASXSSXMXXXAMAAMMSSMAXAMMSMXMXMAMAMXSAMSSMMSSMAMMXMAXAAXMXMASXMXMMXXAAMMMSXMMXSAXMAMXMXMSXSXXMAAMAMAAXAASXSMXSAMAMSMSMMSSSSXSAMMM
MAMXSMSAAMMSMXAAAXSXMMSMSMSAMAMMXSMAAAAMASXMASAMAMMAAAAAXMAXAMXSMMMXXAMXMMXASAMMSSXAAAXAMXMMMAMAXAMXAMMMMSSSSSSXSMMSAAXAMAMAMAXXAAAMMXXMASAS
SXMASAXMXSAAMMMMMXAXMAXMMAMMSXMAAMMSMSXSMSAMSMXSAMMXMMSMMXMSMSAMXSMASXSMASASXASAAMMMMSMAMAMXXSSMSSSMXMAXMAMMMMXAAMAMXMMSSMSMSXSSMMXSASXSAMMA
AMMAMXXAAMMMMSAAXXMMSASASAXXMXMMMSAXXMASXSXMAAASMSXSXMAMSMMAXMAXMAMAXMAXAMSXSAMMSMASAMMAMAXMSMAMAAAXMMSMMAXAAAMMMMXSAXAMAAAASMMAMAAMASAMMSXM
MXMXSXMMMMMMXSMSSMXAMMSASXMSMXSAMAMMAMAMXSASXSMMAXAAAXAMMASXSSSMSMMASMSMMSMXMXMXXXASXSSMSSSMSSSMMXMMXMAAMSMSMSSXXSASMMMSMMMMMASMMMMMAMXMAXXX
XMAAMAXAXAAMMXAXAXMXSASMMMAXMASASMXMXMAXMXXMAAXXXMMXAMSMSXMMMAXMAXMXSAMXMAMXMASMSMXSAMXMAAAAXAXXMASXSSXXAMXAAXMASMMMXAAAXAMSSMMXXMASAMXMMSXM
MMSAMSMSSSXSAXSSSMSMMMMAAXXMMMSXMXXMAXSSSMSMMMMMSAXMSAMAMMASMAMXMMSAMXMASASMSMSAAXAMXSAMASMMMMMMMMXAAAASXMSMSMMSAMXXSMSSSMSAAXXMXXAAAXXAXMXX
AAXXAMAMAMSMMSMAAASXAAMXXXXMAMXMXAXSMMMAAAXMAMAAASXSMAMXMSAMMASASASASXSXMAXXAAMMMMMMASAMAXAXMXAAAMMMMMAAMXXMAMXAXXMAAAXAAMMMMASMSMSSMMSAAMMA
XMMXXMAMMMASXSMSMMMXSMSMAMSXMMXMMSXSAAXMMMMSASMMSMMXMMMXXMAXSASASXSAMXAAMASMMSMMMMXMASAMSSMMSXMMMMAXXXXMAMSSMSSMMASAMXMMMMSXMASAAAXAMASXMAAM
XXSSSXSMXAMAXXAXXAMMXAAMAMMASMMSXMAXSMMSSXASXSASAMXAMAMMMMAXMMSXSMMMMMMXMASXAAXAAAXMXSAMAAAAAASASMSMSSSXSMAAAAAXMXMSSMMMAAAAXMMXMXSAMASASXSS
XMAAMAMASXXMSMSMSSXAMMMMXXXAMAAXMMXMAXSAXMMXXXMMASMXSAXASMSSSMXAXAXAAASAMXSMSMSXMSAXMXMMSSMMSXMAAAAASAMAAMSMMSMSAMSAXAXSMSAMXAAXMAMXMXSAMMXM
XMASMAMASMMXAAAXAXMMMSSMSXMMXMSMSASMMMMMMXMSXMASXXAMAMXMSAAAMMMMMXSMSMSASXSXMASAAXXMASAMAMXXXXSSMSMMMAMXMAXAXMAMAMMMSMMXMXMAMMSSMASAMXMAMSAS
XSAMXXMXMASMMSMMXMMXAAAXAXSXSAMAMASMAAXXMASAMXMMMMMMAMMSMMMMMAAXAXMXXXSMMMSAMAMMMMSAMAMXXXMXMAMAAXMMSSMAXXSXMMAMAMAXSXSMMASAAAAAMASASXSXMMAM
XMSSSSMMSSMSMAXXAASAMSSMXSAAMAMXMXMASXSXMXSMMAXAAAMMMSXAAXMASMSSSMSASXSAMASAMSSXXXMSASXMSSMMMSMMSMSAAASMSXMMXSXMAMSXXAAASXSXSMXSMXSXXASXMMSM
XAAXXMAXMMAASMSSMSAMXMAXXMMXMXMSMXSAXMAXSMMXSASMMMXAMXXSMMMAMXAAAXMASASAMAXMMXMAMXMXXXAMAAAMAAAXAMMMXXAMXAAMMMASMXAMMMMXMAMXXMMMMXSMSXMMSAMS
MMMMXSMMAMSMMMAAXXMAMSMMXMXMXMXXAMAAXMAMXAMAXXMASMSMMAAXXSMSSMMMMXSMMAMAMAMSAMXAMAMMXMAMSXMMMXSAMXAXXSMXMMMMAMMMSAAAXAXAMAMAXAAAAXXAMXMXMASM
XAAAASXMXMASXMMMXSXMXAASMSAAMAMMXMMXMMMSMSMMSXMAMMASMAXSAMAXMASAMXAXXAMAMXMAAMMSMXMAMSMMXXXXAAAMXSXSXMXSMMMSXSAAMSMXXMMMMAMXSSMSSMMAMAMAMMMA
SMMSAMXMXSAMXMAMAASXMSSMASASMSXSASXMMAMAAXXMAAMSMSAMXMMMMMAMSAMASMMSSSSSSMMXMMAXMSMXXAAXMASMMSSMASMMASASAXMAXSMSXMMSMSASMMSMXMAAAXXSSXSASAXA
MMAXXMXXAMMSXSAMMSMXMXMMMMAXAXXSASMSSXSMSMXMMSMMAMMSXXAXAXXMXASAXXXXAXAAAMSAXMASASAMSSMMAXXAAXXMXSAXAMXSXMASMSAMXAAAAMASAMSAMMMMSXSMAXXASASX
SMSSSSSMASAAMSASMAMMSAMMSMMMXMAMAMAXAMXMAXSAAAAMXMMAMXMSMSMMSMMMSMMMSMMMSAAASMXSMMAMAAXMAXMMMSXSASXMASXSASMAAXAXSMMSSMAMXASMSASXMMAMSAMXXAXM
MAMAAAAXAMMMMMMMMASXMASASAMXXMAMMMXMSAAMAMXMSMSSSXMASAAAXXMXMXASAXAAXAXAMXMAXXMXXSAMXSXMAMXAMSAMASXXXXASAMXSXSMMMXMXMMXMMMMAMAMASMXMAXSSMSMA
MAMMMMMSMMASXSMASXSASAMMSXMAXXAXAXSMMSSSSSMMXMAAXAMASMSMSASAMSXSASXMSMMMXAMMMSSMMXAXXMAMAMXAMMAMXMMMSMMMMMMAMMXAXMAAMXXXAXMSMSMAMASXMAAAAAMM
SASAAAAAAMAXXAMAMMSAMXSAXASXSMSSMSAAMAMAMXAASMXMSSMAMAAASASMXMAMMMXXXXASXXMAAXAAMSMSSMXMAXSMMSMMMSAAAASXAAAMAXSXSASXSAMMMSAAAAMMMASAMSMMXMSM
SXMXSSSSSMMSSMMSMAMAMAMMSMMAMAAAXSMMMXXMAMMMMXAXAAMXSSMMMAMXXMXMAAAXXMASAMSMSSSSMAMAMMSMXMAMXXAAAMMMMMMASMSXSXMASAMAMASMASMMSMSSMASXMMXXAAAM
MSSXXMMXMXAAXAAAMSMMMMSAAXMAMMMSMXAXXMAXMASMSSMMSSMAAXMXMAMXMSMSMSSSMAAXMXAAAAAAXAMMXAAAMSMMMSSMSSSMSASXMAXMMAAAMAMXMAAAAMXMXMAAMAMAMXASXSMA
SAMMAMXAAMMSSMSXMAMASXMXSSSMSAAAXXMXMMSXMAMAAAAAAAXAMXXXSASAAAXAAAAAXSSSSSSSMMMMSMSXMSSXXAMXMXXAXAMXAXAAMXMSSXMXSMMMMSSMMSXSAMMXMSSSMMAMXAXS
MASMXMMMXAMAMAAMSMSASXMXMAAASMSXXAXXMAXMMAMMMSXMXSMSMMMMSAMMXMMMXMMMMMMAMAMAMXXAAAMXAXAMAMXXASMMMAMASXMSXMXAMMMAXXXAAMAMXXASXSXXAMAMSXAMSMMM
SAMXASMSAMMXMXMXAAMXSAMMMSMMMXMAXSMSMASASASAAXAMAXAMXAAXMSMXASMSMSAMXXMMMAMASAMMSXMSAMXMMXMXMXAASXMAMAXAAXMXSAMSMAMMXSSMSMMSMXMAMMAMASAMAXAM
MMMXMMAAAXSMMAXMMSMMSAMSXMASXSXMMAAMMASMXASMXSXMAMAXSMXMAAXMMMAAASASMMXSSMSASXMAMXMXMAAMMAMAXMXMSXMASXMMSMMASXSXAMXSAMAAXXXMAASXMSASMXMAXMMS
SASXAMXMSMSASASXXMMAMSASASMMASAMSMMMMASXMXMMXSMMXSMMMMXMSMSMMSSMMMAMAAAXSAMXMMMASAXASMSXSASASMSXMMSAMAMAMAMASMMMSXAAAXMMMXXMMXSAXSASXMMAXSXM
SASMSMMXXAXMAAXMASXSXMXSAMXMXMAMAMAAMAMMXMASAMAAMAXSASMAXAAXAAAXXMSSSMMSMXMASXMMSMXMSAXASMSXSAAAMAMASXMASAMAMAAXMMSSMMMAXXMSMXSXMMMMAXMMMSAA
MAMAMASMMSMAMAMAXSXMXSAMXMAMSSMMSSSSMASAMSMMASMXSAMSASMMMSMMMSSMXMAAASMXMAMAMXAASXSXMAMAMASAMXMXMASXMMMASXMMSSMMSAXAASXSMMAMXMMXMAMXSMMMASXM
MAMSMMXMAAXAAAXAMXAMASMMAXAAXAXAAXAAMAXMXAXSXMXAMXMMXMAMMXAAXAMXAMMMMMSAMXSAMSAMXMAXAMXXMXMASAMMSXXAMMMMSASAAAXAMSMSMMAMAMSXMAMAMASXXAXMASAM
SXSAMSAMXMXMSXMAMMMMAMMSXSMMMMMMMSMMMSSMSMMAMXMMMMXXSAMXAXSXSASMSSSMXMMXMMMAXMMSMXSXMMMMMXSASMSAMMSMMAAXSXMMSXMMSAMXXMXMMMMASXSXSASAXMMMASAM
MMSAMSASMMSAAMXSAAMMSXMAMXMMSSMSMSXXAAAAMXSXMASAASXMMSSMMMXXSAMAXAMXAMSMSASMSMAMXAXAMXSASXMXSAMMSAAXSSSMMAMAMXSXSXSSMMMSMAMXMMAMMMMMMSAMXSAM
MASMMSMMAAMXMSAAMMSAMXSXXAXAAAMAAXAMXSMMMASASXSSSSXMAASASXMASAMXMAMMASAAAXAAAMASMMSSMXSXSAAAMAMXMMSXXAMASAMXAAMMMMMXAAAAMMSSSMMMAXAXMSAXAMAM
MMSXXXAMMMMMAMXSMMMMSAMXSSMMSSSMSMMMMMXMMMSAMXMMXMAMMSSMXAMMMXSXMASXMSMSMAMSMSASAMAXSXXXSAMXMAMXMAMXAMMASXSXMMSAMAMXSMSXSAAXXAASXSMSMXMMSXMM
XAXMMSMMXAXMMMAMAXAAMASMAXMAAMXAXAAAXXAMXXMAMXXXASMMMXMASMMXSAMASASAXXAXXMXMAMXSAMAXAMXXSXMMSASXSXMXSMMXSAMXSXSMSAMMMAXAMMSSMXMMAMXAMASAMAMA
MMSMXAAMSXXMAMXXXMMMSAMAMXMMSSMXMSMSSSXSMMSXSSSMASAAMMXMMXXXMMMXMASXMMAXMAMMXMMSAMMASMMMMAXMAMXAAASAXXXXMASXXAMASASMMAMXMAAAMASMSMSASXMASXMS
ASAXSMMMMAAXXMAASXSXMXSSXMXAAAXMMAMMAMXSAXAAAASMAMXMXSMXMXSXXAAXSAXMXSASXXSAMXAMAMXMXAAXSAMSXSMMMMMASMSXSASMMXMAMXMMAXSXMMSSMMMAMASAMXMAMAAA
SMMMMASASMSMXXAMAAMMSMMXSASMMSXMMAXXAXASXMMAMMMMMSAXAXAMMAMXSMSAMXSAMMASXMAXMMSXSAMXSSMMAAXXASXAXXMXMAAXMASAAXMXSXSAXMXMXMXAXXMAMMMSMAMASMMM
AAXMSAMAMAAMAAXXMMMASAMAAMXMXAAMSMSMSMAXASXASXSASAAMSSSXMASMMMAMXXMXAMSMMXMAMAAMAMXAMAXSSMMMMMSMXMXSMMMMMAMMSMMAXXMASMXSASXMMXSASAAASMSMSXSX
SSMMMXSSMSMMSSMAMXSASAMSSMXXAMXMAASAMMMSAMSXSASXSMSMXAAXSASAAAASAAXASXMAMMASMMXSXMMMSSMMAMXAAAMMXSAMAMXAXAXAMAMASMSMAXAMXSAAMMMXMMMMXXMASASX
MMAMXAAAAXXAAAAXMAMXSAMXAMAMSXMMMSMAXAMMMMSXMXMXSXMAMSMXMXSMMXMSMSAMXAMAMSAMAAMXMAMXAMMMAMASMXSAAMAXAMXSSMSMSMMASMAMMMXMMSXMXAXASXSSXSMSMAMX
XSAMMASMMMMSSSMMMXSASAMSMMAMMAMSMMMXMMMASAMASASAXAXXMAXAMXXXMSXMMAAXMAMSMMASXMASXMASMSSSSMXXAAMMSSSMSSMASMAXXXMAMXAXXAASASASMSMMSAAXAMMMMSMM
AMXSAAMMSMMAAXAMXAMAXAMXXSMMMAMAAXAAMXSXMAMAMAMMMSMMXXSSSSMAAMAAASXMMAMMASAMMSASMMASAAAAAXMMMMSMAMASAAMAMXAXSAMXSSSSMMSMAMAMXXAMMXMMMMSXAMAM
XMASMXMXAAXMAXSMMMSASXMSMSAASXSMSSSXSASXSAMSMMMSMAASAMXMAAXAMSSMMMASXMXSAMASAMAXAMAMMXMSMMAMAMAMASMMSSMMSSMSSMXAMAMAXSAMAMSMMXSMAAMASASMXSAM
XMASXAXSSSMXMXMAAMAXMAAXAMSMMAXAAAAMMASAMXSXAAAAAMAMXXMMSMXSXMASXSAMAAMMMXXMASMSMMAXSAMMXAMSASASMSXXXXAXAAXAMXMXMMMAXMAMXSMASAMMSMSSMASMMSMS
SMMMXMXXXAXXSASXMSSSSMMMSMMMMSMXMMXMSMXAXMAXSMSSSXSAXAAAAXXXAMXSMMMSMMMASMMMXMAAAMAXXAXSXSASASASAXXMASMMSSMMSXSMSAMXSSXMSMXAMASXMAMMMMMAXXAX
XSAAAMSSMMMXMAXSAAAMMAXAMAXXAAMMMSMMMXSMMXMAXAAAXAXAASMMSXSMMXAXAAXXMXSAXAAAAMMMMMMSMMMSAAMMAMAMMMMMXXAAAMAXAMAASXXAXXAXAMMMSXMXMAMAAXXXMMXM
SXMXMSAMAXAAMAMMMMSMSAMASXMMSMSAAAAASMSASXSMMSMMMSMMMAAAXMSAMXMMSMSAMMMMSSMSSSMASXMAAAAMXMXMMMAXMAAAMMMMMSMMMMMMMMMMMXMMMSAASAMSSMSSSMMAMXAS
MMSSMMXSXMSXMMSXSAMXXAMMMXAMAASMSSSXSASAMAAXXAMXAXAMXSMMSASXMASAMXSAMAAAAAMAAAXXMAMSSMSSMSXSMSASMSSSMSAAXXMAXAXAMXSAAASAMXMMSAMAAXAAAMXMMXAS
MAAAXMASMXMASAXAMAXASXMXMMSXSMMMMXMAMXMAMSMMSSXSXSMMAAXMAMMXSASAXAXAMXMMSSSSSXMXSSMMAXMAMXAXAMXSAMAMAMSSSMSSSXSXSASMSMSAMXMAMXMSSMMMMMASMSMX
MMSSMMASMASAMXMSMSMMMMMAMXMMMAMXMAMSMMXAMXAXAAASMMASXMMMMMSXMXSXMSSMMSSMAXAAXMAMAXAXSMSAMMMMMMXMMMAMAMXXAAMXSAAAMASAMASAMSMSSMAMMAXSXSMSAAAM
XMAAXMXMMAMAMXAMAXSXMASXSAMASAMAMXMMASMMSXMMMMMMASAMAAMSMAMXAAMAMAAAAAAMSXMMMAAAASMMMASAMXAAAAXMASXSSSXSMXSAMXMSMAMXMAMXMAAMMAAMMSMSAMXMMMMA
MMMSMSMSMSSSSSSMXMSSMASMAASASASXMSXSAMAXMASAXXXMXMAMSMMAMASMSMSAMSSMMXMMMASXXMASXMAMSAMXMSSSSMXXAMMAMXXMAAMASMMAMAXAMSSSSMSMMSXSAAASAMXMASXS
MSAXMSAXAMAAXAMXSMMAMASMSMMXSAMAXAAMXXXMAXSAXSAMXXSMMXSXSASXAAAAXAAAAMSSSXMAXSAMAMXAMAMXXXXAMMSMMSSXMASMMMSAMASASMSMXMAXXMMAAXAMMMXSAMXMASAM
XMASAMMMMMMXMMMAAASAMXXMAMXAMAMXMMSMSXSAMXMAMXAMAAXAMXXXMASXMSMMMSMMMSAAXMMMMAASMMXXSSMXSAMXMAAASAMXXAAXXXMASAMASXAXAMSMAMSMMMMMSSMSAMXMXMAM
MMMMXMXSXMXMMAMXSXMASXSSMSMMSAMSXMAASMSAMXXMASXMSSMAMMMXMASAXAXXAXASMMMXMAAXXMMMXMXMAXAMMMMAMMSMMASXSMMMMMSAMXMXMMSSXSAAAAAXMXMAXAXSMMXSASAM
AAAXAXAMAMAAMAAAMAMASXAAXAAAAAXMAXMXMASAMXXXMXXAAXXAMAMAMSMMXMAMMSXMASMMSSSSXXXXAMASMMXMAASASAMMSXMXMXAAAAMXMXMAMAXAXSMSMSSXMAMMSMMMASASASMS
SSXSAMXSASASXMSXSAMAMMXMMAMXSMMSXMSAMMMMXMASXMMMMSSSSMXAMAAXMMAAXAXMSMAXXAAAMSMSASAXAMXSXXSASXSAXXMASMSXSSSSXSSSSMMMMMXXMMMASXMMAXXSAMMSASAS
MAASAAMSAXAXXAAAMXMSXSXXAAXMMMASXAXAXMAXXXAMAAXAXXAMAMSSSMSMXXAXMAMXAMXSMSMMMAAMXMXSAMXMSMMMMMMMSMMMSMAAXAMMAMAMXXAXAXXXMAMXMAASASMMASAMAMAM
MMMMASMMMMSMMMMSMSMMAXMASASMSMASMSMMSSSMXMMSXMSMSMAMAMAAAAMAMSSSMAMSSSMXAAMXSMSMSAASAMAAAXAMAMAXMXMXMMMMMAMMAMAMSSMSSSMMSMSXSSXMASASAMAMMMSS
MSAMXMXMAAXAXXAAMAAMXMAMMAAAAMASAAAXAAASAMXAXXAAAMXSXSXSMMMXMAAAMAXAAAXMSMSAXMAASAMXASMSMSMSXSAMXAXAXAAASXMSSSMSAAAAAAXAASXAXMXXASXMXSAMXAAA
ASAMXXXSMSSMMMSSSMSMXMSXMXMSMMMSMSSMMSMMSMXASMMSMSMXXXAAXXXXMMSMMSMMSMMXXXMASMMMMXXSXMAXXXMAMMXMSASXSSSXXSAXMXMAMMMMSMMSSSMMMAXMASXMMSASMMSS
";
