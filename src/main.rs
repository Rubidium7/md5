use std::env;
use std::num::Wrapping;

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

const S: [u32; 64] = [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
	5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
	4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
	6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21]; 

const K: [u32; 64] = [0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
	0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
	0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
	0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
	0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
	0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
	0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
	0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391];



// fn print_bytes(s: &str) {
// 	let bytes = s.as_bytes();

// 	for byte in bytes.iter() {
// 		println!("{byte:x}");
// 	}
// }

fn print_bytes(s: &Vec<u8>) {

	for byte in s.iter() {
		print!("{byte:x} ");
	}
	println!()
}

 

fn print_hash(hash: [u32; 4]) {
	
    let message_digest = format!(
        "{:08x}{:08x}{:08x}{:08x}",
        hash[0].swap_bytes(),
        hash[1].swap_bytes(),
        hash[2].swap_bytes(),
        hash[3].swap_bytes()
    );

	println!("{message_digest}");
}

fn make_padded_vec(input: &str) -> Vec<u8> {

	let og_len_in_bits = Wrapping(input.len() as u64 * 8);
	let mut input_vec: Vec<u8> = Vec::new();
	input_vec.extend(input.as_bytes());
	input_vec.push(128_u8);

	while (input_vec.len() * 8) % 512 != 448 {
		input_vec.push(0_u8);
	}
	let og_len_in_bytes : [u8; 8] = og_len_in_bits.0.to_le_bytes();
	let mut og_len_in_bytes = og_len_in_bytes.to_vec();
	input_vec.append(&mut og_len_in_bytes);

	input_vec
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		eprintln!("usage: {} <string>", args[0]);
		return ;
	}

	let input = String::from(args[1].clone());
	let hash: [u32; 4] = [A, B, C, D];
	
	let len: u64= ((((input.len() as u64) + 8) / 64) + 1) * 64;

	let padded: Vec<u8> = make_padded_vec(&input);
	
	//hash_loop(padded, len, hash);
	
	print_hash(hash);
}
