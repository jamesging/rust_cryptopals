use std::ffi::CStr;

pub mod hex_utils;
pub mod crypto_utils;

fn main() {
	set1_challenge1();
	set1_challenge2();
}

fn set1_challenge1() {
	let input_hex = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	let expected_result = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	let base64_result = hex_utils::hex_to_base64(input_hex);
	let cstr = CStr::from_bytes_with_nul(&base64_result).expect("invalid string from hex_to_base64");
	assert!(cstr.to_str().expect("invalid string") == expected_result, "Set1 Challenge1 result does not match expectation");
	println!("Set1 Challenge1 Result: {}", cstr.to_str().expect("invalid string"));
}

fn set1_challenge2() {
	let input_hex1 = String::from("1c0111001f010100061a024b53535009181c");
	let input_hex2 = String::from("686974207468652062756c6c277320657965");
	let expected_result = String::from("746865206b696420646f6e277420706c6179");
	let fixed_xor_result: String = crypto_utils::fixed_xor(&input_hex1, &input_hex2);
	assert!(fixed_xor_result == expected_result, "Set1 Challenge2 result does not match expectation");
	println!("Set1 Challenge2 Result: {}", fixed_xor_result);
}