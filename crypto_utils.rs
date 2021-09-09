use hex_utils::string_to_hex_value_u8_vec;
use hex_utils::hex_value_u8_vec_to_string;

pub fn fixed_xor(input_left: &String, input_right: &String) -> String {
	let mut result_xor: Vec<u8> = Vec::new();
	let left_hex_vals: Vec<u8> = string_to_hex_value_u8_vec(input_left);
	let right_hex_vals: Vec<u8> = string_to_hex_value_u8_vec(input_right);
	if left_hex_vals.len() != right_hex_vals.len() {
		panic!("Invalid input strings to fixed_xor, lengths are not the same");
	}
	for i in 0..left_hex_vals.len() {
		result_xor.push(left_hex_vals[i] ^ right_hex_vals[i]);
	}
	return hex_value_u8_vec_to_string(&result_xor);
}