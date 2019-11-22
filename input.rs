fn main() -> i32 {
	return test(1, true);
}

fn test(mut a: i32, b: bool) -> i32 {
	a = a + 1;
	let c: bool = b && true;

	if (c) {
		let a: i32 = 100;
	}

	if (a == 100) {
		a = 0;
	} else {
		a = a + 1;
	}

	a = prec(a);

	if (a <= 2) {
		a = -1;
	} else if (a == 0) {
		a = -2;
	} else {
		a = a + 1;
	}

	return a;
}

fn prec(a: i32) -> i32 {
	return 2 + a * 3;
}