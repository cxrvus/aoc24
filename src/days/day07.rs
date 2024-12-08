type Number = u128;

#[derive(Clone, Debug, Default, PartialEq)]
enum Operation {
	#[default]
	Add,
	Mul,
	Cat,
}

impl Operation {
	fn exec(&self, a: &Number, b: &Number) -> Number {
		use Operation::*;

		let (a, b) = (*a, *b);
		match self {
			Add => a + b,
			Mul => a * b,
			Cat => Self::concat(a, b),
		}
	}

	fn next(&self) -> Option<Self> {
		use Operation::*;

		match self {
			Add => Some(Mul),
			Mul => Some(Cat),
			Cat => None,
		}
	}

	fn next_without_concat(&self) -> Option<Self> {
		use Operation::*;

		match self {
			Add => Some(Mul),
			Mul => None,
			Cat => panic!(),
		}
	}

	fn to_str(&self) -> &str {
		use Operation::*;

		match self {
			Add => "+",
			Mul => "*",
			Cat => "|",
		}
	}

	fn concat(left: Number, right: Number) -> Number {
		let mut left = left;
		let mut i = right;

		while i > 0 {
			left *= 10;
			i /= 10;
		}

		left + right
	}
}

#[derive(Debug)]
struct Calibration {
	result: Number,
	operants: Vec<Number>,
}

impl Calibration {
	fn parse(input: &str) -> Vec<Self> {
		input
			.trim()
			.lines()
			.map(|line| {
				let halves: Vec<&str> = line.splitn(2, ": ").collect();
				let result = halves[0].parse().unwrap();
				let operants = halves[1].split(" ").map(|x| x.parse().unwrap()).collect();

				Calibration { result, operants }
			})
			.collect()
	}

	fn is_valid(&self, allow_concat: bool) -> bool {
		dbg!(&self);
		let Calibration { result, operants } = self;

		let get_next_op = if allow_concat {
			Operation::next
		} else {
			Operation::next_without_concat
		};

		let base = (Operation::default(), operants[0]);
		let mut stack: Vec<(Operation, Number)> = vec![base];
		let mut failed = false;

		while let Some((op, left)) = stack.pop() {
			if failed {
				if let Some(next_op) = get_next_op(&op) {
					stack.push((next_op, left));
					failed = false;
				}
				continue;
			} else {
				stack.push((op.clone(), left));
			}

			// dbg!(&stack);

			let right = operants[stack.len()];
			let value = op.exec(&left, &right);

			// dbg!(value);

			// todo: re-add value > result optimization

			if operants.len() > stack.len() + 1 {
				stack.push((Operation::default(), value));
			} else if value == *result {
				return true;
			} else {
				failed = true;
			}
		}

		false
	}

	fn total(calibrations: &[Self], allow_concat: bool) -> Number {
		calibrations
			.iter()
			.filter(|x| x.is_valid(allow_concat))
			.map(|x| x.result)
			.sum()
	}
}

pub fn part1() -> Number {
	let calibrations = Calibration::parse(INPUT);
	Calibration::total(&calibrations, false)
}

pub fn part2() -> Number {
	let calibrations = Calibration::parse(INPUT);
	// todo: allow concat
	Calibration::total(&calibrations, false)
}

const INPUT: &str = PROD_INPUT;

const MIN_INPUT: &str = "83: 17 5";

const TEST_INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

const PROD_INPUT: &str = "
479027832: 8 9 69 659 96 634
539373: 6 19 6 863
50830: 27 9 91 25 2
40213: 9 9 35 9 8 63 11 641
26324383: 7 9 8 7 414 471 185 8
11884578: 26 91 864 391 590 78
72700530602: 854 46 3 415 194 601
61748500: 74 5 896 689 5 66 53
108: 7 70 23 3 5
298900979: 4 57 9 144 35 14 979
1302118: 4 5 38 7 239 4 6 373 7
7238: 716 7 2 5 8
37856477: 728 65 8 4 74
189170: 7 269 76 698 93 3
7325: 73 2 5
345703232252: 3 8 4 6 9 128 6 4 4 8 2 52
9609860666: 208 91 230 2 666
24003: 5 99 9 4 4
9389450245: 47 329 5 2 66 3 92 628
959212: 4 4 2 516 472 4 25 708
363266: 2 8 7 7 3 2 6 8 302 863 3
4758: 74 19 51 14 1 1
1387500: 58 128 68 8 5 37
165768: 179 9 2 5 87 27 5
34729721751: 8 5 7 5 6 2 4 3 4 1 742 10
5544045: 621 83 875 5 9
23: 1 5 4 4 7
31938411: 53 264 23 20 195 411
50880: 253 6 4 4 5 5 54
36123702: 94 8 319 92 337
1041: 586 6 3 26 419
1318649: 5 4 26 96 9 2 7 6 8 4 418
1131220137: 7 5 5 5 7 315 64 5 76 7 5
107904742: 29 97 350 4 9 139
8692190347508: 38 62 8 3 957 1 50 9 5 9
248139: 14 9 5 4 3 7 844
1213689: 291 6 43 873 688
480495: 3 3 5 59 3 74 852 7 9 6 3
257610286: 42 48 45 50 28 20 6
11378: 167 15 62 94
3120: 52 4 7 4 7 5 7 36 991 71
375716893773: 86 9 7 6 6 3 24 459 3 1 6
3015937: 7 9 222 5 87 6 7
78452: 927 41 81 7 37
36679602: 856 9 1 535 8
133081692237: 2 475 583 5 16 28 5
1800960: 27 61 6 8 8 420
304148: 5 8 8 73 630 8
18374977210: 9 6 2 48 584 5 9 3 5 1 8 1
10192: 5 202 8 9
22869: 72 710 4 70 69 7
10138276936: 4 52 306 1 733 5
647881: 5 6 610 308 5 206 559
290: 25 4 29 2 8
2399633064: 236 1 675 5 3 3 1 2 5 6 7
264556262: 7 77 4 3 40 8 49 3 33 5
82305266448: 882 642 17 6 9 605 9
6932815: 59 631 3 20 72 742
14686: 7 36 58 70 1
1649: 2 1 9 82 53 4 90 463 55
7798657: 43 751 18 30 6 96
12553640: 24 79 554 220 1
7907: 74 65 444
382091: 45 8 3 24 4 9 2
2420: 27 3 380 23 5
455638: 6 62 67 26 12
36988109: 660 502 8 7
575686004: 59 6 3 4 6 43 8 9 4 6 5
993899607: 40 4 8 353 84 3 29 94 8
32532: 8 1 5 25 5 13 18
7416: 93 20 6 28 53 5 36 36
92906446: 7 88 171 98 9 95
3089876: 6 5 89 876
5455192: 899 164 37 51 9
20856: 77 32 1 913 163 899 3
72267789002: 2 3 8 2 5 4 3 438 775 4 5
8079: 8 994 4 4 58 9 9 8 26 1 8
54730: 7 7 3 4 5 180 1 8 4 4 6 44
4935024: 3 769 9 81 78 97 3 566
9904791314: 7 9 190 96 9 39 315
3223765695: 732 674 440 1 91
7711608: 7 8 6 5 4 3 5 1 94 4 82 1
18911175: 6 72 6 645 626 9 23 15
8316883: 3 9 505 290 2 65 3
377020: 6 88 251 4 4
526041819402: 944 953 657 89 1 43
3851549231: 2 6 8 28 3 3 28 4 980 33
107: 3 3 2 4 84 9
92244852: 2 44 1 9 4 77 1 98 42 9
62170895131: 7 65 14 61 457 49 9
11227450: 193 62 44 50 751
4351218022388: 12 98 37 1 802 238 6
24566861832764: 29 959 584 3 6 82 763
295399313497: 9 8 4 3 4 24 8 7 8 29 9 7
769: 67 8 594 95 3
61420988: 1 8 68 7 1 7 162 6 8 5 8 9
9876: 6 6 823
10054653519: 73 3 381 3 18 372 4 9
6989: 7 7 384 68 15
135129529789: 777 33 277 85 62
7673: 7 1 23 6 38 2
7903928909: 784 6 392 8 907
1240076: 5 6 485 6 595 4 32 64 1
32746896: 330 13 918 13 8
1244802: 5 2 431 532 49 6 203 5
1230672: 5 37 693 17 8 40 1 16 7
12041297446956: 424 674 27 457 922
5513286: 772 6 4 5 814 7 708 7 7
97636049: 1 2 199 5 4 99 4 1 8 8 5 6
8049: 7 627 421
39261044: 28 6 256 1 451 148 9 5
19224632856: 2 486 4 9 39 3 39 8 7 3 1
1929115855: 5 37 7 8 6 5 1 5 8 57
27304: 3 6 284 85 106
32616946: 3 6 3 8 6 4 454 9 6 7 17
815199619: 6 57 4 77 773 4
8775823: 1 3 6 9 5 65 8 1 5 7 1 2
217617: 53 9 8 6 52 45
477661548: 6 3 45 4 2 7 842
29764: 7 411 71 86 3
67100184: 2 5 98 545 65 18 2
38047: 238 374 62 68 30 5
235: 85 84 52 14
8303004: 86 9 23 380 1
10666: 9 2 95 21 1 1 8
26841: 99 64 22 4 142
536460: 1 48 5 3 380 45 8
3621549: 659 785 43 7 43
2087722: 629 505 263 4 7
10933912587730: 47 9 3 7 563 54 7 5 578
15397609243: 513 25 36 3 362 2 876
1167073: 220 7 9 5 751 10 9
730901143: 1 571 80 122 9 5 177
79670161: 1 7 4 643 3 9 51 1 7
55499656002: 596 831 9 348 322 42
1013553596: 85 271 2 16 85 8 44
27479462: 428 642 6 8 795 5 996
116525843073: 7 5 813 657 713 91 3 7
1322433: 56 9 263 1 4 7 9 6 44 8 4
9490150373555: 82 501 902 6 3 55 7 1 6
16539526: 6 4 43 601 4
78813047: 4 74 8 1 30 48
49747920765: 2 4 4 9 9 8 8 7 64 546 9
10240294986: 2 529 31 40 2 949 77 6
31132779: 4 3 9 38 1 3 3 8 5 3 19 84
25191837: 9 8 19 2 1 3 4 81 701 9
8225: 781 859 5 5 1
448799: 56 8 7 29 72
137161187: 762 200 9 11 67 18 3
44310875: 92 773 7 63 89
35349953: 4 9 2 2 3 6 7 6 9 244 711
39390624: 7 4 39 90 9 96 91
15361586: 9 770 2 393 4
938450: 37 667 17 2 19
15001400: 668 7 674 701 4
66735558037: 681 4 6 7 3 68 5 487 35
86293: 612 6 14 507 22
45734133: 2 5 367 83 34 132
3362: 3 3 42 6 26 271 4 375
6802263846: 35 427 4 8 2 8 4 6 6 8 5 6
18865059385: 628 3 68 2 465 3 1 3 8 8
339028: 735 46 84 8 9
6678676: 6 678 274 18 386
1714445: 2 69 4 9 9 207 4 273 4 5
3572841955: 4 57 544 18 7 6 6 37 7 8
1133784000: 651 60 87 20 3
623: 35 2 2 16
707: 6 4 58 5 99
19994817: 9 98 2 6 352 9 230 6
16562765604: 4 7 48 7 4 9 1 3 7 1 12 77
2049: 66 9 38 15 352
45221301678: 657 309 7 225 99
34742: 7 11 8 421 4 176 3
3041280: 4 5 6 660 48
435872412455: 3 7 9 8 8 539 1 2 4 3 45 6
4757632028: 812 46 63 1 99 88 26
1646464388441: 783 7 9 9 611 382 44
3044809: 11 12 189 2 234
48784029: 9 2 56 6 4 5 96 7 956 3 9
619028: 476 13 1 73 1 5 49
72600: 5 6 44 66 5 4
341739027572: 50 42 4 38 405 2 7 572
148575: 8 6 75 73 758 7
108050910044: 491 4 5 3 6 26 550 4 6
421008: 410 8 3 5 179 42
5387741232438: 695 510 6 66 19 8 436
439590198: 748 6 865 674 658
2340: 1 23 39
30518405: 10 88 175 2 459 878 3
15531686192: 466 417 37 10 1 9
2004561: 8 688 4 3 1 80 1 27 3
27249935040: 3 4 99 401 9 7 5 3 9 8 7
446568: 38 326 9 4 600
342738: 6 9 6 4 27 79 3 7 135 9
20803548992: 7 1 3 157 9 5 8 5 48 9 9 2
4620711220: 840 55 7 112 2 2
580614049: 92 8 993 58 2 45 6 7 1 9
15358178: 5 307 81 71 8
6260474896: 645 5 45 5 1 9 832 2 9
57: 8 1 4 8 9 27
3074841: 477 900 319 7 1
1403327: 12 5 3 495 45
1949: 68 21 3 3 8 7 5
443856499: 3 41 385 4 2 4 88 1 2 6 3
252853988: 766 2 1 242 33
56667: 508 11 54 24 7
4151663: 3 7 9 4 9 1 5 9 9 105 24
5705: 7 1 4 1 6 564 6 6 7 5 53 7
49731430430: 141 1 5 448 9 6 4 6 7 7
28786: 6 76 61 442 1 382 145
264924: 440 1 6 6 99
646397410: 359 5 544 4 4 1 4 9 86
2130139: 291 8 1 6 2 6 2 97 4 3 2 5
607033: 4 55 547 682 2 18 334
81325728039: 838 19 895 3 960 3 6
1642848: 29 7 3 808 184
216332418: 582 1 53 50 281 7 4 4 5
1799886: 566 318 3
2077118361: 3 785 882 3 71 9 50 10
4788545: 63 950 80 3 451 89
1237754: 43 65 7 5 9 678 6 8 7
4854245763: 273 12 42 420 84
19246: 47 33 9 4 278
198645: 49 66 4 4 2
426450: 389 4 7 9 8 9 4 2 541 2 6
8221614: 403 3 299 94 58
28148373: 891 1 5 92 3 9 6 526 9
7379: 4 2 1 86 590 5
22504104678: 7 8 8 9 93 92 4 1 6 69 7
25892: 8 32 6 7 222
890487: 5 708 92 85 120 370
22384616: 2 3 8 12 90 301 6 749 2
2040941138: 99 8 426 48 6 34 8 8 4
29902530: 361 45 267 99 2 3 9 27
42916034: 3 855 3 5 1 64 5 2 391
297894: 941 321 490 3 17
5147781196853: 857 84 5 7 35 328 6 55
297338803: 39 5 29 69 7 7 2 9 4 6 1 5
1424023: 81 96 995 8 8
201670525236: 417 53 73 7 69 93 38
3288661955: 567 58 609 84 9 960
814208538: 214 4 5 1 845 34 7 6 26
17362647368: 109 1 9 9 6 24 265 5 1
1586825227: 21 699 4 9 82 9 5 2 2 9 6
9791: 9 88 90
148878: 736 102 22 975 3 81
7300: 2 7 318 85 311 7 10
3095735794: 36 6 3 1 477 57 94
1339880257: 40 85 82 4 259
11365989800469: 56 829 94 900 2 3 2 7
1474435: 24 8 594 5 345 962
57327: 4 1 46 5 21
6577055605: 798 64 7 1 1 8 809 756
456890: 489 5 248 615 7 553
5763: 8 9 38 48 483
574081871: 8 15 598 234 8
528840464: 80 2 9 5 52 6 415 51
57552044154: 872 660 44 149 6
10800518765: 12 1 900 518 764
6120: 74 5 897 6 6 3 2 54 138
1038120881: 533 950 7 20 881
460348740: 868 3 4 401 656 2 740
29069625808: 966 3 6 3 3 59 6 2 581 1
69561: 1 5 2 590 5 2 42 861 9
3098487638: 1 32 414 5 52 4 9
433007: 2 165 8 25 8
3645213943: 54 91 8 5 4 2 771 7 8 3 5
378966: 7 332 907 24 5 975 41
6714932938022: 7 8 993 7 4 145 7 92 25
37235: 5 8 69 177 698 5 880
7970525755: 8 74 599 91 7 247 8 2
140797490: 681 37 2 687 974 90
2372816023246: 968 9 772 5 6 588 46
9304984576: 9 21 9 4 921 63 565 10
28276428: 6 6 31 1 1 9 7 1 38 32 4 8
308076: 39 9 418 71 48 4
53926: 9 149 7 4 6
242577574: 2 7 88 248 87
2544764544: 47 44 73 5 3 28 9 304
18439604: 9 6 2 3 5 59 3 9 7 1 989 4
432: 83 4 5
14236771: 19 8 88 439 282
25672: 6 21 96 5 5 6 8 2 58 9 1 2
147: 93 9 18 7 20
186945375: 21 9 646 71 7 61 9 9
2762762: 71 5 168 23 2
4793309: 32 8 4 6 5 2 5 48 8 6 8 7
3339372: 46 3 26 5 2 7 6 522 842
39070504: 700 305 183 4
39955399: 3 285 977 142 9
127900089: 80 3 3 9 1 7 293 51 8 2 7
32534: 9 96 7 4 1 72 8 6 77 3 5 1
1496879175: 12 61 36 7 8 8 4 71 88
43314: 1 8 3 9 89 40 23
443: 4 59 63 92 223
27758: 2 7 75 2 4
455283983: 542 84 39 8 2
17903904: 8 2 74 2 5 8 9 7 5 6 597 3
164212614: 6 99 3 467 589
332: 1 3 63 59 19
169957872291: 62 8 777 1 441 2 3 3 58
1579812: 1 13 41 988 3
7565601: 68 3 13 9 471 4 888
650111616: 7 1 4 6 660 44 36 4 7 7 3
64658888681: 205 9 2 7 4 8 9 3 1 973
110542: 21 47 37 7 987
15954644: 289 6 20 92 3
21444451948: 709 6 777 75 386
62439561: 29 4 42 18 712 9
276121: 4 40 7 547 322 7 801
159969: 2 987 81 6 66 6
3035371450: 7 27 978 289 494 9 4 1
239460: 2 5 641 9 4 7 200 9
5713195236: 40 2 426 49 3 62 7 9 7
2149739426: 8 88 2 28 6 54 7 95 83 5
1718: 4 118 30 9 2 9 7 5 9 7 3 9
9919: 7 7 7 818 5 45 40 4 7 55
34032: 151 1 9 50 24
14715168903: 68 5 56 9 62 6 8 905
2994525122: 143 1 93 7 3 75 43 80
43209839: 7 6 3 150 60 3 5 1 93 36
185759568: 4 57 30 4 709 6 9 65
9161171: 6 270 922 5 36
28041: 64 28 906 98 6 75
3593179865754: 359 31 798 65 7 52
2865: 2 86 8
10692457: 8 6 69 75 1 27 2 4 58
4687467843727: 86 744 9 407 17 2 3 24
26612: 13 76 4 483 6
638458315: 4 742 1 8 6 9 435 3 4 9 7
846650: 27 376 7 412 8 4 963
611312537954: 998 8 766 96 612
830179: 8 2 98 38 340
81705528: 40 7 216 4 244 76 2 8
1103706718: 52 7 459 734 296 9
3277071: 456 38 2 9 189
8229389: 777 45 938 7
534640294: 1 533 6 40 294
2052383067: 1 120 7 6 4 1 3 3 91 2 7
10491952: 3 7 49 11 8 53
8303907: 3 18 2 345 7 34 28 4 7 6
121279661: 3 3 231 875 9 37 59
8016321895: 30 28 4 2 5 7 4 1 9 4 9 98
105366571639: 731 712 3 6 3 55 8 1
602344227: 941 8 1 3 8 1 1 9 6 5 4 95
5083703181: 50 8 344 25 722 596 1
25570353: 3 9 76 3 9 729 7 8 8 6 1 8
8520: 2 7 3 839 9
13240: 713 189 2 751 8
7507719: 4 5 384 65 4 2
24634550: 18 823 95 4 25 262
461920: 8 58 31 51 32
48092166237366: 7 29 5 5 9 2 228 6 877 4
691: 63 39 590
1101: 1 2 12 96 1
4776018820094: 5 1 48 796 7 5 28 25 96
71549891: 234 6 73 402 45 4 891
65971: 95 1 474 3 38 1 990
2833381733328: 943 8 6 6 5 2 88 8 2 6 8 6
12366799605021: 8 48 2 3 54 4 8 450 9 1 8
1713454082: 611 8 4 7 497 5 7 6 5 7 5
987959604486: 341 618 6 7 23 482
419832: 3 97 5 3 1 833
5590023: 852 223 52 22 2
230667: 64 6 5 3 2 3 8 5 1 5 34 3
1574643681: 69 9 79 464 36 82
15620: 26 1 79 4 2 71
3398985: 15 182 5 83 45 3
72740702586: 227 1 6 748 7 9 43 2 51
2043806: 6 6 5 3 4 85 125 50 3 56
61980953: 291 599 53 696 65
882700: 8 89 7 52 5 5
66529013: 9 90 672 381 632
2008688: 6 63 24 3 827 808
28367539823: 30 572 984 8 210 623
2877: 3 60 8
52543072: 28 98 7 9 259
7258524: 331 3 3 697 5 76 644 3
1487567187: 21 4 6 6 4 965 3 6 32 8
1474483503169: 9 70 7 743 5 9 30 1 66
486353339: 863 860 563 110 50
7183823: 921 2 13 3 23
64897414: 2 39 7 5 79 6 6 806 3 9
33481349208: 2 7 5 89 1 912 651 8
10637: 14 8 585 2 2 7 776 88 1
36614095: 103 64 108 5 203
6882018086: 173 6 6 9 2 3 2 330 83
2580928: 366 11 2 65 64
3574756019: 4 1 79 905 60 17
4561971456: 9 490 66 2 4 758 84 8
983106398: 98 3 106 40 1
251853981: 4 29 4 5 52 3 2 2 4 82 5 7
30030613: 396 9 815 4 74 9
3871: 8 35 2 97 2 77
95531982: 2 8 9 7 7 897 331 18 24
4618468: 7 506 108 52 9 28
607: 53 1 5 19 51
38888: 2 588 433 867 5
677821505: 12 6 56 23 388 4 2 8 6 3
75800: 1 2 47 537 83
265349883428: 2 47 1 388 5 7 6 4 3 6 9 7
241279: 4 2 6 5 4 2 315 1 40 65 4
690856530: 1 8 814 95 741
199715: 9 4 4 7 40 2 7 1 53 1 3 61
965312183: 8 312 2 19 3 58 7 80
61183275: 667 5 357 55 1 87
1400708: 6 8 5 3 1 396 1 7 67 6 8
112594: 9 8 8 45 92
429636: 1 864 496 590 6
1482458: 23 6 5 97 457
31614408: 76 27 87 2 18 98
2482: 94 4 5 7 5
48634: 1 5 3 2 6 63 58 924 22
2312924: 384 496 66 925 6
350017: 8 27 7 6 7 46 214 4
610092810: 1 27 4 774 93 4 3 54 5 9
16927878611: 9 66 6 703 272 652 82
8915044725: 786 470 1 883 9 535 5
499175627: 50 2 801 76 77 1 1 8
53008382: 9 6 35 2 7 9 55 5 532
433314040: 64 64 7 21 6 9 380 7 42
17701929: 688 3 3 5 9 3 3 2 1 9 17
150174193: 816 46 7 314 232 2 4
2470006: 5 2 5 33 699 17 87 3 92
11037638037: 876 42 5 6 2 6 8 3 10 9
949685: 5 6 321 2 4 41 67 6 88
1717284: 8 7 1 2 438 9 77 7 35 4 5
7316: 55 7 19
90497: 5 418 5 223 39 95
18614610: 21 63 67 30 7
4776528: 39 8 761 4 25
4692240011: 70 798 70 12 14
8013: 80 1 2
7719: 2 616 9 6 273
722396745: 9 917 3 8 36 3 7 9 2 1 7 8
140072504: 21 4 7 4 750 5 832 3 8
50461993118: 4 5 888 4 287 440
1652432: 319 7 74 6 6
5405525: 9 51 37 9 2 5 552 5
18749162: 187 491 6 5
147238350973: 813 193 479 3 7 9 1 3 2
62818731: 5 7 82 2 1 8 7 6 1 9 346 9
43686: 890 4 6 4 48 4 290
236322561: 9 8 11 844 248 2 1 6 1
1780290: 94 20 6 71 3 786
254880093: 95 754 596 4 3 30
12143752: 2 986 687 725 5
1515882: 9 6 1 7 9 4 17 75 6 2 180
40428: 9 8 6 41
48749373: 5 8 7 41 61 704 8 2 47 3
2197095: 3 3 71 1 3 61 9 56 1 99 4
7991: 62 46 92 2 94 27
21360925: 79 3 34 32 4 9
25258: 2 9 5 1 8 1 5 9 7 45 7 2
25520: 480 52 557
167623903: 2 5 5 6 534 2 107 3 6 47
9255627: 8 73 3 41 929
4324284200: 116 320 5 253 49 98
657311: 2 54 2 772 85 752 6
91781: 721 895 23 28 2
382727719428: 7 875 6 3 3 5 6 5 9 74 1 9
2120662: 989 4 6 838 84 86 74
107819: 66 7 3 5 5 47 5 54 5 2 71
86547302572: 2 6 363 900 3 5 8 9 7 2
53064278158: 67 792 1 207 71 15 4 6
306602: 6 274 18 8 604
41241669: 44 52 8 537 70
1084391294983: 97 9 69 6 960 831 6 9 3
143990: 9 8 4 3 778 216 2
87046848: 4 6 6 7 7 325 1 8 8 9 96 2
3042364000: 4 4 4 914 10 1 2 5 650 8
78292621: 2 650 33 411 71
12249: 6 8 9 165 42 9 6 4 8
596166973: 4 741 8 8 8 6 3 5 84 83 6
597: 13 1 16 37 1 352
17177: 1 40 1 84 48 74 69 63
33357599: 9 3 14 4 1 3 572 2 3 16 6
621584: 85 61 679 2 53
8476482: 6 282 7 9 93 41 9 5 8 4 1
727646: 6 3 4 7 643
19310004: 9 26 79 777 218
936016: 104 5 5 360 9 7
6271776412470: 18 3 6 68 21 103 118 4
16848: 1 8 936
309755: 2 4 5 23 5 3 711 3 2 205
468321108826: 77 9 4 3 8 4 3 81 451 75
94284: 3 103 5 28 3
207977924: 25 1 68 4 621 9 3 7 7 1 6
77834773888: 6 772 347 73 887
77741: 7 19 1 56 7 94
80917752: 7 1 9 7 118 65 239 24
1319980: 51 83 995 190 982
305645866808: 4 84 3 6 9 586 6 8 10 7
97155423: 1 86 6 176 5 71 6
31621436279: 651 24 2 65 510 83 9
26817: 4 469 6
281762: 7 4 99 4 41 63
204471673406: 35 6 7 4 647 1 842 143
1111: 76 50 8 5 400 36 1 5
187652950250: 69 501 27 99 151 253
676192220766: 8 169 38 444 145 5 42
15319968215: 951 4 2 498 16 215
150001: 534 5 25 3 1 264 45 4
790637990: 4 48 2 7 7 67 3 79 2 7 10
1852437: 8 65 356 8 431
115748421794: 1 10 207 7 6 6 3 7 7 5 5 9
221122803677: 32 691 2 803 680
79525470: 787 8 23 2 441 29
512392: 62 8 95 92 76
9224: 20 28 9 717 4 8
7233408: 4 7 8 39 644
12326464: 133 3 3 3 329 462
1240109: 8 5 121 5 1 8 5 9 11 1 4 8
1847: 68 6 2 895 136
32730604: 541 605 59 47
127: 4 1 8 4 5
6887587392: 9 5 4 128 4 7 4 9 8 6 84 8
40284359256: 242 9 7 9 9 117 6 25 9
9085238895: 514 996 4 6 8 4 3 8 8 94
45267169: 7 9 27 7 571
26320033: 53 13 1 53 3 12 2 82 6
4092536: 909 42 45 6 5 76 6 50
214224125: 286 4 6 418 5 397 5 6 8
926890320: 82 7 9 906 49 2 471 40
6561314: 82 4 2 131 3
1601: 5 69 2 911
36541121965: 6 3 1 58 2 22 43 8 8 1 5 5
3920136731: 9 6 2 74 721 6 5 567 2 8
38024116764: 3 6 4 2 2 8 241 1 676 6
3077565371715: 7 336 90 3 1 6 468 166
106438447: 622 2 440 384 49
5260: 5 47 60 1
278705700: 4 508 6 9 1 8 5 6 7 5 4 9
1436928: 8 9 6 921 6 71 5 2 6
1951431370: 2 4 89 85 8 76 9 5 48 6 5
19899723: 1 3 83 412 1 25
8432163: 946 3 34 886 88
812364030: 509 95 24 7 29
7462067: 44 39 8 2 839
9157805023: 49 6 2 792 2 4 68 5 203
137882988389321: 90 383 7 47 4 389 319
33583: 2 28 4 31 965 12 66 4
3717: 97 6 9 4 9
1599785: 616 585 1 510 11 85
117849: 5 1 6 3 8 5 6 8 709 38 3 8
6090541652: 27 41 371 2 1 55
8719108: 897 9 9 4 5 6 5 50 64 6
47752: 48 5 9 410 5
56324: 9 59 92 2 9 2
2035735966: 1 1 24 2 8 1 9 24 9 9 91 4
116570: 798 9 72 8 253 2
29709873287: 88 742 4 35 8 68 13
234174187: 3 45 1 57 5 84 9 4 5 9 59
16736701598: 343 6 2 7 7 37 72 5 439
211644: 9 2 9 60 13 61 10 434
179013467433: 378 60 67 45 2 61 4 30
166860: 16 5 3 201 90
21133014574: 3 773 752 6 35 4 4 5 4 2
477838346: 954 9 7 6 1 1 5 62 1 6 5
488581292: 2 67 4 158 704 5 2 93
613128: 8 424 1 472 3
1562923362: 38 2 473 80 3 3 577 22
883109888: 3 8 5 6 15 8 5 5 5 1 7 1
35305510: 9 255 1 1 75 22 205
61140267395: 37 46 2 51 69 2 356 4 8
921755: 7 7 867 62
1551312028568: 5 8 567 3 456 57 5 6 5
7965693: 862 11 475 8 94
91279: 90 6 6 61 9
14623844097: 38 483 8 76 5 88 11
15193332929: 54 638 63 7 9 28
445599: 608 35 9 7 11
3929218999: 76 678 172 18 6 64 76
40719830: 39 881 838 817 16
10234: 91 4 1 7 4 8 21 13
169: 124 8 1 1 36
30930: 30 63 6 9 288
49410: 2 19 6 139 270
16565: 6 10 55 1 7
938: 93 21 19 54 5
1150493155: 6 4 316 810 493 157
2162495: 12 2 30 832 3
1161: 5 28 74 943 4
49705: 4 1 5 1 8 6 73 2 52 24 5 5
1408162: 47 587 574 5 9
45711994001: 884 216 665 1 45 4 8 9
461746986: 1 6 432 38 668
6256097: 6 46 40 680 9 8
932669: 93 266 9
20168337417: 23 6 643 277 9 5 6 4 1 8
71351707: 37 290 2 4 857 823 76
3763: 1 37 99
16080: 396 7 433 5 58
434234: 2 163 647 8 5 10 9 44
6482: 7 7 13 6 4 6 991 979 1 1
39893544: 41 1 7 139 495 49
350035: 13 4 827 750 8
37814: 6 80 57 879 37
1380736918: 1 876 736 9 18
2125872000: 86 34 74 525 456
590566: 89 7 5 564 856 64
2383244325: 69 117 25 8 8 164 29 9
1111: 3 6 1 24 86
2478600296: 3 7 1 255 972 293
113122749: 6 58 2 781 226 64 85
3588584: 4 181 62 4 444 586
229896240115: 8 8 8 6 34 5 8 8 18 2 616
28575260691: 59 408 13 338 5 1 37
9265890414: 926 58 90 374 42
51484691: 520 322 2 50 63 611
1040297: 699 4 4 2 93
12524863130276: 5 30 6 928 6 2 969 73 5
6508607: 7 32 415 20 7 5
707165786: 227 35 673 10 89
50601453135: 715 17 87 794 8 68 52
28548350: 8 7 83 3 9 2 2 8 5 2 1
160292921527: 926 577 7 738 4 3 7
253414931: 888 267 87 22 274 8
898: 20 6 4 615 4 67
26195558: 77 486 23 7
36957027: 4 106 336 9
42539: 96 91 238 3 6
536380505: 7 5 595 18 29 47 9 3 8 2
344090: 5 92 2 6 59
3957: 76 553 56 634 3
2422596: 4 2 3 22 59 4
857: 34 23 75
101657569857: 331 24 33 149 93
50738: 6 2 9 4 48 8
126: 4 21 5
11479: 1 3 32 3 302
811587288: 72 992 83 919 9
2529432: 1 85 86 342 1
68498020: 58 31 330 93 766 2
151583563: 25 284 354 2 49 2
20379175829: 975 9 9 4 3 3 18 58 3 2 5
4603: 6 751 8 86 1
6217347273: 469 45 12 336 6 4 9 2 3
39774594: 3 718 259 459 6
2447810379025: 699 5 9 9 7 197 70 27
915416: 3 6 50 8 72 7 46 30 2
180195304: 25 6 7 9 3 4 6 6 6 7 303
133151317497: 522 34 423 739 6 3 8 9
78240: 9 3 321 9 237
1451933: 573 50 1 8 39 59 1
651038099: 24 84 84 4 8 32 43 961
4067855507: 686 8 4 87 74 74 76 5
85026: 27 4 46 6 1 18 4 7 4 797
725685: 900 7 8 8 2
13595185922: 1 6 3 3 96 262 2 4 8 590
254255: 6 12 353 9 3
504356064: 8 40 59 24 60 622
96248635: 962 141 336 9 37
5755949526: 2 6 1 59 9 94 5 9 495 2 3
3186: 42 7 8 6 8
75549035500: 1 4 2 5 6 4 11 111 3 54
2162: 7 20 3
16475136579: 85 673 8 4 8 97 9 14 9 9
195316907: 7 279 16 879 29
14893565: 71 233 5 38 9 20
18198236: 8 295 6 1 82 37
123309891: 759 8 193 833 69
47500: 5 28 92 5 76
7512336961: 829 3 2 27 43 4 9 964
87280442712: 8 7 280 442 712
7241800834: 36 7 8 30 8 9 8 4 69 4 8 7
734076: 364 600 7 9 84
116633: 92 337 679 58 34
1063639692: 3 426 41 3 832 74
696950735: 101 30 2 180 23 592 2
480860228: 8 60 8 60 226
23: 5 2 13
2786: 7 2 3 279 92 7
2616505: 701 533 7 87 8 5 974
97117282: 6 2 6 4 3 86 5 8 6 4 3 76
337862714: 97 1 6 53 8 1 574
467738006: 729 95 81 6 8 69 516
396418141: 301 439 210 171 3
19534253: 2 188 14 6 6 8 9 161 5 1
1226338348: 943 130 43 8 34 5
5161934: 80 76 849 14
29562003: 9 5 31 9 5 7 9 2 4 76 3
981279452: 5 545 76 3 4 9 4 1 9 9 2 9
211213420: 22 8 6 2 13 423
292: 27 5 6 5 2
3830: 1 48 3 35 8
39906107: 28 576 638 66 1
469693: 1 9 9 6 277 6 5 1 92 4 96
20531724713: 3 17 2 32 4 389 247 13
57609: 60 8 3 2 4
8600: 2 570 3 4 5
46593: 1 9 2 49 69 19 19 24
2068: 9 1 220 9 1 78
9673650: 8 33 32 74 7 83 45
857: 76 6 91
50804424: 269 69 76 47 551 3 3 4
47754844836: 8 71 44 7 7 4 152 9 3 4
5675402460: 2 4 3 4 520 4 5 2 7 1 21 2
37840572: 8 8 2 5 53 40 5 61 3 6
1921721: 9 1 9 7 75 117 7 61 2 21
724519325591: 105 69 193 2 5 54 4 8
184839: 2 1 46 1 24 2 7 5 8 5 3
482549415: 1 19 8 7 6 58 8 202 2 2 3
11195245: 4 36 6 929 5 4 351 247
111184: 9 2 117 7 7
847677692808: 752 95 67 76 92 810
29390907: 7 2 51 9 6 178 12 97 30
8444586: 957 2 43 47 874
2178027047: 21 780 270 11 37
2817139367: 7 3 846 28 46 7 34 1 7
3793: 46 3 363 41 7
1938: 6 39 3 14 6
1184361: 908 788 77 668
580356547: 37 66 8 83 296 8
9009968: 92 85 578 80 88
98071: 19 614 5
67259: 1 5 726 2
58837679: 3 3 95 6 782 4
13865366: 462 5 800 6 566
762572523: 6 3 532 8 3 8 8 7 1 4 646
28358704848: 378 5 58 15 4 850
455021: 214 239 985 1 78 959
897: 4 393 499
61970: 7 59 150 20
29458: 9 1 6 2 6 260 754 5 89 1
188015178612: 9 19 1 3 4 3 4 2 559 6 1 5
69160: 7 4 9 3 3 59 7 6 9 8 1 56
32750: 9 3 96 465 213 4 8
4752228: 9 22 800 30 226
117964: 56 5 391 795 66 1 77
2814624: 21 1 8 9 9 7 1 56 2 4 9 8
922210: 3 43 1 2 24 265 4 3
568694644: 56 8 694 215 427
28545605: 8 90 250 626 1 4
2226913383: 9 5 6 8 30 4 60 8 5 8 82
501457483: 9 200 737 6 41 63 98 2
47924895: 891 3 2 4 9 4 8 2 7 8 5 95
7837400: 3 7 4 3 6 82 2 973 8 745
9928694: 82 910 86 15 78
370200093: 987 788 5 50 1 19 417
46434: 8 9 4 601 758
45316225286: 953 11 175 47 289
5369310: 9 5 99 31 958 7 3 76 11
2325363: 31 2 7 69 65
42357933: 86 4 2 91 46 6 7 9 4 21 8
545787: 4 9 4 4 7 565
704933270: 37 381 6 9 1 55 7 3 4 5
8720: 5 436 4
11823: 2 71 43 97 72
22741950: 6 49 76 59 770
5235416: 871 6 48 893 6
3386: 59 1 9 49 8
358589982: 512 270 7 9 85
3034749: 3 98 1 3 6 7 108 2
744180: 9 9 662 1 181
58000414934: 82 918 58 410 4 931
48137153: 2 7 758 7 9 9 7 2 3 8 6 83
57552881: 32 1 6 32 7 4 4 83 1 8 41
7006549: 80 2 8 865 4 1 2
3102: 13 318 676 3 81
116644638: 3 205 5 3 3 47 564 9 1 4
1051844: 529 2 22 5 9 8 3
5353920: 8 8 715 3 39
2644597015: 419 295 7 559 5 37
52254730: 5 225 466 6 9
11488680: 9 7 924 679 2 9
18623: 3 2 76 146 8 6 3 7 3 70
42816163: 6 5 6 9 72 54 3 1 71 964
41946: 10 7 1 6 7 2 5 7 43 7 1 2
8183787858: 5 4 558 1 5 5 4 2 4 3 13 8
1557786579: 20 16 5 89 868
467817605: 495 2 678 3 94
2297229362: 3 745 62 225 4 363
4973982: 7 3 62 6 230
365272: 8 34 41 44 71
7269003: 3 81 9 7 1 3 793 6 7 7 63
952425: 4 4 7 75 83 3
91911: 83 1 89 7 76 407
9547: 57 66 23 984 2 9
11760261: 39 20 3 26 1
1533: 55 710 9 8 3 741 2 5
1100508606915: 78 21 246 572 54 912
684156: 41 2 89 73 71
21390: 1 23 1 7 1 3 5 2 610 5 6 1
1950692122: 830 235 19 2 121
25581: 2 5 551 1 1 25
11936: 23 718 5 8 2
1324: 97 15 3 7 99 3
802474583: 3 2 738 173 535 76 4 6
187725246720: 901 84 4 46 656 620
5640043914682: 5 470 24 439 1 467 9
116280601: 6 631 2 88 18 590 7 4
19881824665: 1 978 8 6 1 6 68 499 2 4
1036887: 2 100 7 9 885
11925030602: 52 3 77 265 408 25 3 5
57072334: 6 9 309 9 4 6 2 4 2 3 57 1
5512320057: 596 42 5 5 2 6 9 8 4 1 4 7
1320852514: 539 30 751 8 52 512 3
691: 77 2 7 8 1
6741399760: 674 1 399 688 69
50900868315: 797 1 3 2 129 9 3 165
737071088: 664 15 42 74 5
16292: 536 14 639 2 5
82162954: 2 19 3 112 144 463 5
135191283: 160 939 3 840 1
545472419: 8 6 6 6 23 47 23 1 4 7 9 3
323: 69 13 228 8 7
374447: 891 1 3 1 4 6 30 9 4 8 4 2
99302403523: 8 55 7 50 8 96 6 2 5 5 2 3
803: 7 23 5 1
20392701: 96 18 5 7 212
1846762620: 9 4 65 28 12 40 5 24 5
525204224: 58 35 6 9 222
5622: 96 32 22 412 2
1001297: 83 2 835 2 99
8866240740651: 4 9 9 9 5 1 470 42 5 8 7 1
16161: 202 80 3
18392917: 94 8 148 18 613 13 1 7
1956174: 95 1 7 5 22 831
118542608: 28 14 63 8 2 350 226 8
183216872: 1 88 563 7 8 457 77 9 2
117593050794363: 7 7 4 6 7 3 536 1 74 36 3
19028270: 3 338 3 67 28
852616932: 9 84 3 756 616 929
77555: 8 99 26 7 94 5
392041093: 5 5 2 81 6 7 5 8 35 6 9 3
72779300001: 9 67 8 94 9 25 80 1
146243097: 1 8 8 3 4 4 72 7 4 39 693
190320583: 519 1 6 61 583
14630112: 12 4 531 574
122929: 1 9 243 8 471
1423085307: 79 18 966 97 22 3 10 6
8082996929: 1 616 582 5 69 8 9 928
1719: 9 182 9
610770: 3 3 797 1 6 73 44 3 9 4 6
4104567568: 69 81 48 32 3 4 153
447058: 4 46 1 10 57
20788483: 67 310 5 177 2 56 29
504485325932: 6 8 4 1 7 6 7 369 39 47
65260363: 611 350 1 3 29 10 676
1128553734873: 5 302 9 4 54 532 9 4 71
90944000: 35 568 4 788 35 2 406
90052: 2 8 7 1 5 17 1 8 3 23 6 2
52570: 8 2 1 7 3 7 4 6 8 99 3 35
6115797: 47 14 15 791 6
250: 4 61 5 1
144388: 6 8 127 3 3
23719254375: 2 63 886 81 77 36
211983858: 11 3 1 19 6 7 3 7 6 8 4 5
13403: 46 89 99 1 37
";
