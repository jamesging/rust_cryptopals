const BASE64_CHAR_SET: [u8; 64] = ['A' as u8,'B' as u8,'C' as u8,'D' as u8,'E' as u8,'F' as u8,
	'G' as u8,'H' as u8,'I' as u8,'J' as u8,'K' as u8,'L' as u8,'M' as u8,'N' as u8,'O' as u8,'P' as u8,
	'Q' as u8,'R' as u8,'S' as u8,'T' as u8,'U' as u8,'V' as u8,'W' as u8,'X' as u8,'Y' as u8,'Z' as u8,
	'a' as u8,'b' as u8,'c' as u8,'d' as u8,'e' as u8,'f' as u8,'g' as u8,'h' as u8,'i' as u8,'j' as u8,
	'k' as u8,'l' as u8,'m' as u8,'n' as u8,'o' as u8,'p' as u8,'q' as u8,'r' as u8,'s' as u8,'t' as u8,
	'u' as u8,'v' as u8,'w' as u8,'x' as u8,'y' as u8,'z' as u8,'0' as u8,'1' as u8,'2' as u8,'3' as u8,
	'4' as u8,'5' as u8,'6' as u8,'7' as u8,'8' as u8,'9' as u8,'+' as u8,'/' as u8];

fn byte_to_base64_char(byte: u8) -> u8 {
	return BASE64_CHAR_SET[byte as usize];
}

fn push_buffer_to_result(base64_buffer_ref: &mut u32, base64_result_ref: &mut Vec<u8>, buffer_count_ref: &mut i32) {
	for char_num in 0..4 {
		let base64_byte: u8 = 0b00111111 & (*base64_buffer_ref >> (6 * (3 - char_num))) as u8;
		base64_result_ref.push(byte_to_base64_char(base64_byte));
	}
	*buffer_count_ref = 0;
	*base64_buffer_ref = 0;
}

pub fn string_to_hex_value_u8_vec(input: &String) -> Vec<u8> {
	let mut accum: String = String::new();
	let mut result_vec: Vec<u8> = Vec::new();
	for a_byte in input.chars() {
		if accum.len() < 2 {
			accum.push(a_byte);
		}
		if accum.len() == 2 {
			let dec_byte_val: u8 = u8::from_str_radix(&accum, 16).expect("Invalid hex string");
			result_vec.push(dec_byte_val);
			accum.clear();
		}
	}
	return result_vec;
}

pub fn hex_value_u8_vec_to_string(input: &Vec<u8>) -> String {
	let mut result_string: String = String::new();
	for a_byte in input {
		result_string.push_str(format!("{:x}", a_byte).as_str());
	}
	return result_string;
}

pub fn hex_to_base64(input: String) -> Vec<u8> {
	let mut base64_result: Vec<u8> = Vec::new();
	let mut base64_buffer: u32 = 0;
	let mut buffer_count: i32 = 0;
	let input_hex_vals = string_to_hex_value_u8_vec(&input);
	for a_byte in input_hex_vals {
		if buffer_count < 3 {
			let shifted_byte: u32 = (a_byte as u32) << (8 * (2 - buffer_count));
			base64_buffer |= shifted_byte;
			buffer_count += 1;
		}
		if buffer_count == 3 {
			push_buffer_to_result(&mut base64_buffer, &mut base64_result, &mut buffer_count);
		}
	}
	if buffer_count != 0 {
		for _i in buffer_count..3 {
			base64_buffer <<= 8;
			buffer_count += 1;
		}
		push_buffer_to_result(&mut base64_buffer, &mut base64_result, &mut buffer_count);
	}
	base64_result.push(0b00000000);
	return base64_result;
}
