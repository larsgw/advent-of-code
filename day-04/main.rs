fn check_password (password: i32) -> bool {
    let mut increasing = true;
    let mut consecutive_digits = false;

    let mut previous_digit = 0;
    for i in (0..6).rev() {
        let digit = (password % 10_i32.pow(i + 1)) / 10_i32.pow(i);

        if digit < previous_digit {
            increasing = false
        }

        if digit == previous_digit {
            consecutive_digits = true
        }

        previous_digit = digit
    }

    increasing && consecutive_digits
}

fn check_password_2 (password: i32) -> bool {
    let mut increasing = true;
    let mut two_consecutive_digits = false;
    let mut consecutive_digits = 1;

    let mut previous_digit = 0;
    for i in (0..6).rev() {
        let digit = (password % 10_i32.pow(i + 1)) / 10_i32.pow(i);

        if digit < previous_digit {
            increasing = false
        }

        if digit == previous_digit {
            consecutive_digits += 1
        } else if consecutive_digits == 2 {
            two_consecutive_digits = true
        } else {
            consecutive_digits = 1
        }

        previous_digit = digit
    }

    increasing && (two_consecutive_digits || consecutive_digits == 2)
}

fn find_passwords (lower: i32, upper: i32, check_password: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut passwords = Vec::new();
    for password in lower..upper {
        if check_password(password) {
            passwords.push(password)
        }
    }

    passwords
}

fn main () {
    let lower = 138307;
    let upper = 654504;

	println!(
		"star 4-1: {}
star 4-2: {}",
		find_passwords(lower, upper, &check_password).len(),
        find_passwords(lower, upper, &check_password_2).len()
	);
}
