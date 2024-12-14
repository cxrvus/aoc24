pub fn part1() -> usize {
	todo!()
}

pub fn part2() -> usize {
	todo!()
}

type Input = (usize, usize, &'static str);

const INPUT: Input = INPUT1;

const INPUT1: Input = (
	11,
	7,
	"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
);

const INPUT0: Input = (
	101,
	103,
	"
p=1,65 v=-5,-84
p=14,63 v=-85,6
p=49,81 v=71,14
p=61,77 v=85,67
p=58,37 v=42,47
p=99,13 v=-4,-72
p=50,29 v=58,96
p=84,63 v=-93,-25
p=77,90 v=-3,-70
p=11,82 v=-74,54
p=94,3 v=16,-96
p=30,86 v=-29,96
p=31,92 v=-70,97
p=58,99 v=16,-89
p=14,99 v=45,36
p=42,25 v=-93,-16
p=25,68 v=37,61
p=55,100 v=4,-98
p=34,95 v=-41,53
p=40,86 v=37,-73
p=55,7 v=50,-63
p=76,19 v=30,-75
p=98,73 v=55,-4
p=92,0 v=37,-67
p=66,75 v=98,-43
p=96,54 v=14,-38
p=74,16 v=26,-88
p=37,36 v=72,-32
p=59,95 v=34,91
p=28,102 v=5,44
p=46,21 v=-94,76
p=51,34 v=-66,-66
p=93,99 v=-40,70
p=2,79 v=-18,9
p=0,12 v=81,20
p=85,25 v=-68,-77
p=7,81 v=31,-28
p=43,29 v=-68,-74
p=45,33 v=15,50
p=20,66 v=-57,38
p=13,52 v=-55,35
p=17,6 v=-47,-35
p=31,93 v=80,99
p=33,49 v=-86,8
p=64,41 v=63,-16
p=57,11 v=-45,80
p=62,6 v=-96,-84
p=35,2 v=71,-26
p=36,51 v=95,-39
p=88,3 v=90,57
p=17,16 v=12,-82
p=40,56 v=-64,90
p=67,101 v=-89,4
p=35,84 v=66,-68
p=96,51 v=-84,-66
p=99,14 v=96,-96
p=44,55 v=14,-53
p=8,61 v=-12,40
p=16,2 v=52,-91
p=39,35 v=94,13
p=23,30 v=-64,-58
p=18,90 v=87,34
p=51,70 v=31,-23
p=40,36 v=-80,-10
p=100,102 v=-62,-46
p=69,73 v=77,-81
p=24,90 v=48,-82
p=24,22 v=23,-96
p=88,74 v=4,51
p=71,42 v=-75,-26
p=99,79 v=-62,-97
p=41,9 v=59,-50
p=99,42 v=-27,77
p=77,36 v=-63,-60
p=50,42 v=-38,-87
p=21,21 v=29,26
p=58,65 v=-52,-84
p=67,42 v=-1,-66
p=83,14 v=-68,-88
p=86,83 v=-55,59
p=10,33 v=31,-98
p=52,27 v=-67,84
p=55,73 v=78,35
p=15,46 v=2,19
p=19,51 v=-12,95
p=15,58 v=-99,-71
p=36,96 v=8,15
p=95,15 v=32,-14
p=54,98 v=38,-52
p=78,85 v=19,-94
p=16,70 v=95,35
p=78,91 v=-47,-15
p=92,29 v=25,-40
p=69,77 v=48,-52
p=30,44 v=-72,58
p=3,86 v=-4,-97
p=37,53 v=7,61
p=69,60 v=-17,-47
p=49,7 v=86,-80
p=0,28 v=89,20
p=42,90 v=26,-60
p=17,91 v=-14,20
p=75,48 v=62,-66
p=12,15 v=-56,-72
p=85,81 v=-18,91
p=76,78 v=98,70
p=89,13 v=-83,65
p=55,24 v=-66,-24
p=44,100 v=-65,33
p=96,51 v=-99,48
p=42,23 v=8,18
p=37,84 v=-7,22
p=19,7 v=-20,-88
p=86,26 v=-32,18
p=2,83 v=-12,46
p=35,44 v=66,48
p=93,101 v=10,49
p=62,47 v=42,-50
p=97,98 v=-40,25
p=98,80 v=-97,-49
p=97,72 v=-62,54
p=74,6 v=-53,-46
p=92,53 v=-32,72
p=65,88 v=3,-94
p=31,42 v=15,-45
p=4,97 v=-98,70
p=8,52 v=96,3
p=35,31 v=-97,45
p=32,4 v=51,-1
p=44,68 v=50,-44
p=50,42 v=-46,29
p=20,99 v=37,12
p=77,33 v=47,-38
p=34,99 v=32,62
p=17,75 v=88,75
p=68,90 v=70,38
p=94,33 v=-39,71
p=50,97 v=-4,6
p=36,29 v=98,84
p=53,94 v=-15,-12
p=64,64 v=-8,-81
p=89,78 v=-17,-81
p=99,59 v=68,61
p=6,30 v=-85,-6
p=95,0 v=38,68
p=8,69 v=-12,-89
p=84,12 v=41,-54
p=26,73 v=58,40
p=2,84 v=53,-36
p=9,69 v=-18,-71
p=88,56 v=83,-2
p=89,78 v=61,80
p=38,1 v=66,33
p=94,72 v=-90,36
p=74,61 v=-83,-23
p=21,53 v=51,15
p=55,91 v=-14,-49
p=93,29 v=11,-95
p=63,31 v=99,-78
p=62,93 v=-81,-91
p=37,101 v=46,18
p=60,77 v=-33,89
p=7,52 v=-98,87
p=11,64 v=2,-97
p=45,6 v=86,52
p=13,82 v=36,90
p=15,18 v=45,-64
p=0,101 v=10,25
p=13,49 v=-15,-71
p=43,24 v=-22,-48
p=54,8 v=95,-7
p=33,101 v=-29,4
p=0,32 v=-1,-38
p=73,14 v=99,44
p=94,64 v=54,-2
p=36,93 v=22,-78
p=72,98 v=12,-25
p=77,62 v=-75,61
p=58,83 v=42,83
p=86,62 v=-83,45
p=81,69 v=25,46
p=87,73 v=-78,-1
p=70,91 v=41,12
p=21,12 v=-72,81
p=9,87 v=-69,70
p=33,72 v=36,-76
p=10,10 v=-87,-44
p=100,63 v=-4,83
p=99,1 v=-69,12
p=96,96 v=39,-70
p=9,3 v=-12,94
p=85,101 v=10,-20
p=62,35 v=56,87
p=76,79 v=-14,-2
p=44,14 v=-51,-96
p=81,15 v=68,-80
p=83,63 v=60,-50
p=91,66 v=90,80
p=87,21 v=82,97
p=62,58 v=-9,82
p=12,79 v=16,-20
p=98,92 v=-42,95
p=44,53 v=94,-89
p=21,96 v=95,17
p=84,12 v=11,-30
p=46,89 v=57,23
p=69,18 v=-9,-90
p=65,47 v=-60,-29
p=30,39 v=1,-23
p=46,62 v=-97,-70
p=97,64 v=59,99
p=54,95 v=-88,-41
p=0,53 v=39,-63
p=81,85 v=99,-54
p=46,53 v=50,8
p=17,51 v=-51,-56
p=40,64 v=85,58
p=38,21 v=63,66
p=14,21 v=81,-98
p=82,63 v=-25,56
p=41,65 v=24,44
p=75,9 v=51,31
p=75,80 v=-21,-36
p=70,99 v=19,75
p=90,58 v=-90,24
p=87,36 v=40,37
p=84,65 v=26,56
p=27,47 v=43,-18
p=3,47 v=65,-31
p=83,42 v=-82,-18
p=52,54 v=78,46
p=20,91 v=94,-1
p=72,64 v=5,80
p=95,38 v=-83,-82
p=66,63 v=33,-34
p=94,65 v=-40,-92
p=31,57 v=-44,89
p=63,68 v=57,80
p=45,63 v=-22,11
p=26,54 v=13,67
p=17,14 v=-78,-22
p=9,71 v=1,14
p=31,34 v=94,58
p=58,5 v=-52,-80
p=61,36 v=-72,32
p=95,102 v=90,31
p=41,58 v=73,90
p=27,42 v=15,-21
p=40,99 v=37,71
p=56,61 v=6,19
p=0,62 v=-69,-58
p=85,88 v=-76,47
p=31,99 v=1,62
p=42,80 v=9,7
p=79,17 v=-38,-24
p=99,74 v=-91,-23
p=56,45 v=-80,58
p=15,31 v=15,42
p=77,57 v=-3,32
p=76,87 v=-82,70
p=53,43 v=23,-29
p=73,98 v=-30,14
p=92,98 v=-84,38
p=41,21 v=-28,-30
p=79,86 v=64,-85
p=84,68 v=62,-47
p=65,94 v=38,-64
p=21,64 v=60,-47
p=85,65 v=-91,14
p=55,39 v=6,-34
p=53,24 v=49,-35
p=99,73 v=82,67
p=33,56 v=29,53
p=59,98 v=70,-14
p=39,97 v=50,33
p=20,24 v=-71,81
p=3,93 v=2,54
p=26,39 v=30,13
p=37,2 v=15,49
p=10,24 v=-86,-51
p=5,53 v=68,-95
p=15,95 v=16,-33
p=34,49 v=-78,74
p=91,81 v=54,-28
p=91,18 v=61,-77
p=36,27 v=-8,-14
p=44,3 v=-69,82
p=6,75 v=-76,-18
p=53,38 v=34,-19
p=95,69 v=-88,-28
p=41,76 v=94,-89
p=99,57 v=-19,-50
p=20,77 v=23,-15
p=73,95 v=-60,78
p=91,66 v=-9,-88
p=3,99 v=3,75
p=44,51 v=-36,16
p=53,32 v=99,-52
p=25,87 v=-42,-78
p=3,86 v=4,17
p=93,96 v=-86,63
p=51,54 v=42,-71
p=12,20 v=9,-11
p=100,59 v=-98,19
p=83,26 v=-46,-15
p=38,17 v=-14,60
p=100,39 v=20,81
p=75,22 v=-60,-28
p=0,18 v=-55,9
p=10,21 v=16,-30
p=0,59 v=-54,89
p=100,95 v=46,-91
p=49,79 v=-8,9
p=11,99 v=-34,94
p=74,66 v=91,35
p=2,78 v=83,54
p=5,84 v=-9,-31
p=62,50 v=63,32
p=4,51 v=-24,-71
p=8,27 v=74,-69
p=72,26 v=-37,-56
p=46,44 v=51,40
p=27,31 v=8,-56
p=33,0 v=15,63
p=15,92 v=62,-36
p=24,7 v=-85,57
p=54,101 v=-26,-70
p=83,12 v=62,83
p=99,99 v=-26,99
p=56,49 v=75,-96
p=42,91 v=50,25
p=93,64 v=-5,-71
p=80,85 v=-97,20
p=98,10 v=53,-30
p=54,95 v=28,4
p=53,59 v=99,-15
p=29,60 v=29,-85
p=47,79 v=21,22
p=93,89 v=-5,-31
p=74,21 v=-60,-30
p=3,52 v=89,98
p=35,9 v=-71,41
p=10,85 v=45,-49
p=23,12 v=-70,47
p=97,61 v=-40,-92
p=91,16 v=24,-1
p=56,61 v=-9,77
p=77,11 v=98,24
p=18,41 v=-42,79
p=96,16 v=2,-3
p=48,15 v=-9,-54
p=84,58 v=-25,93
p=48,85 v=95,-41
p=54,11 v=-37,-96
p=76,95 v=90,-54
p=33,59 v=-65,88
p=30,32 v=22,-40
p=4,5 v=53,94
p=22,14 v=-36,65
p=52,99 v=28,-4
p=65,40 v=-3,-98
p=100,65 v=-19,72
p=25,49 v=-99,61
p=94,51 v=31,-74
p=65,34 v=34,-53
p=62,16 v=-58,-45
p=86,95 v=33,-86
p=7,31 v=-63,-98
p=76,40 v=83,3
p=13,64 v=96,-92
p=7,10 v=-85,25
p=85,102 v=-46,-20
p=30,76 v=-51,75
p=38,95 v=-53,45
p=13,44 v=99,-28
p=37,20 v=15,39
p=88,14 v=-9,-45
p=20,77 v=98,97
p=37,90 v=-14,9
p=9,75 v=96,75
p=83,52 v=66,-27
p=20,51 v=-53,-70
p=91,15 v=-11,-88
p=21,98 v=-13,41
p=80,38 v=4,95
p=24,12 v=22,73
p=72,15 v=5,68
p=72,33 v=84,95
p=91,92 v=83,-20
p=74,43 v=-2,87
p=19,101 v=58,-75
p=0,61 v=-8,69
p=81,64 v=-2,-55
p=42,38 v=-15,-16
p=99,102 v=-92,23
p=54,98 v=81,11
p=28,87 v=-22,-60
p=38,7 v=66,-99
p=13,76 v=74,-57
p=79,85 v=-68,75
p=16,70 v=74,59
p=20,32 v=73,71
p=15,97 v=-85,62
p=68,67 v=-46,11
p=42,37 v=55,-53
p=92,98 v=18,25
p=39,101 v=5,-79
p=91,60 v=-51,-65
p=78,11 v=-90,81
p=12,14 v=89,-22
p=45,58 v=79,93
p=4,78 v=-98,-84
p=16,101 v=-85,94
p=41,9 v=-28,-72
p=59,53 v=13,-31
p=87,30 v=-54,-90
p=86,68 v=90,85
p=55,86 v=-34,77
p=66,90 v=48,51
p=60,53 v=-74,-95
p=45,78 v=-68,26
p=90,86 v=25,46
p=41,38 v=-29,50
p=40,22 v=43,-40
p=30,92 v=-85,-56
p=35,27 v=-29,37
p=54,49 v=-96,16
p=46,9 v=21,-9
p=24,50 v=-34,-37
p=92,45 v=68,34
p=7,58 v=13,27
p=2,82 v=31,1
p=79,59 v=-10,48
p=7,102 v=-34,12
p=59,29 v=-88,-8
p=16,59 v=-59,55
p=72,17 v=41,89
p=9,25 v=53,-27
p=15,23 v=-27,-8
p=66,84 v=-24,-36
p=68,62 v=-66,-7
p=90,31 v=19,-64
p=0,96 v=24,-91
p=47,86 v=36,57
p=58,23 v=88,88
p=58,56 v=-45,40
p=68,70 v=-53,-76
p=21,57 v=23,32
p=91,26 v=16,98
p=51,84 v=-73,99
p=40,34 v=28,-86
p=26,82 v=-75,41
p=40,38 v=-65,-3
p=50,85 v=63,-57
p=8,62 v=-35,-9
p=37,49 v=42,-11
p=100,60 v=-7,29
p=14,100 v=-21,60
p=35,3 v=23,-16
p=67,57 v=34,-39
p=71,69 v=12,-31
p=64,91 v=56,33
p=72,68 v=48,61
p=38,9 v=57,92
p=65,93 v=-70,-99
p=2,70 v=44,59
p=14,47 v=-53,87
p=61,72 v=42,-60
p=48,88 v=-58,67
p=15,72 v=66,-76
p=26,60 v=46,78
p=73,24 v=34,59
p=64,55 v=92,95
p=29,24 v=-89,-37
p=40,98 v=65,-91
p=91,65 v=76,-26
p=88,59 v=-60,-2
p=80,73 v=-48,51
p=85,41 v=83,95
p=87,11 v=76,68
p=17,65 v=15,63
p=88,96 v=-18,-1
p=7,6 v=24,-19
p=3,18 v=-89,56
p=41,74 v=-59,-23
p=20,44 v=38,87
p=60,96 v=-71,-27
p=5,33 v=54,49
p=55,68 v=97,44
p=22,60 v=-57,19
p=89,18 v=39,-38
p=96,27 v=61,-14
p=57,16 v=-15,42
p=0,95 v=-26,72
p=58,39 v=-59,-58
p=44,67 v=14,6
p=100,48 v=-14,67
p=26,28 v=22,47
p=24,60 v=19,53
p=31,39 v=16,34
",
);
