pub fn gcd(a:u64,b:u64) -> u64 {
let mut a = a;
let mut b = b;
let mut r = a%b;
let mut temp = a;
loop {
		match r {
			0 => {return b;},
			_ => {temp=a;a=b;b=temp%b;r=a%b;},
		}
	}
}

#[test]
fn main() {
	let mut a = 486;
	let mut b = 330;
	let mut r = a%b;
	let mut temp = a;
	loop {
		match r {
			0 => {/*println!("GCD is {}",b );*/break;},
			_ => {temp=a;a=b;b=temp%b;r=a%b;},
		}
	}
	assert_eq!(b,6);
}
