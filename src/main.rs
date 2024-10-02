use std::env;
use std::error;
use std::error::Error;
use std::fs;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::num::Wrapping;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct InputError;

impl Error for InputError{}

impl fmt::Display for InputError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "usage: ./md5 (= reads stdin)\nor\nusage: ./md5 -s <string>\nor\nusage: ./md5 <filename>")
	}
}

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


fn f_func(x: u32, y: u32, z: u32) -> u32 {
	x & y | ((!x) & 0xffffffff) & z
}

fn g_func(x: u32, y: u32, z: u32) -> u32 {
	x & z | y & ((!z) & 0xffffffff) 
}

fn h_func(x: u32, y: u32, z: u32) -> u32 {
	x ^ y ^ z
}

fn i_func(x: u32, y: u32, z: u32) -> u32 {
	y ^ (x | ((!z) & 0xffffffff))
}

fn rotate_left (x: u32, n: u32) -> u32 {
	(x << n) | (x >> (32 - n))
}

// fn print_bytes(s: &str) {
// 	let bytes = s.as_bytes();

// 	for byte in bytes.iter() {
// 		println!("{byte:x}");
// 	}
// }

// fn print_bytes(s: &Vec<u8>) {

// 	for byte in s.iter() {
// 		print!("{byte:x} ");
// 	}
// 	println!()
// }

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

fn cut_to_u32_words(input: &Vec<u8>, start: usize) -> Vec<u32>  {
	let mut words: Vec<u32> = Vec::new();
	let mut	tmp: Vec<u8> = Vec::new(); 
	let len: usize = start + 64;

	let mut byte_count = 0;
	for i in start..len {
		tmp.push(input[i].clone());
		if byte_count == 3 {
            let byte_arr: [u8; 4] = tmp.clone().try_into().unwrap();
			let word: u32 = u32::from_le_bytes(byte_arr);
			words.push(word);
			tmp.clear();
			byte_count = 0;
		}
		else {
			byte_count += 1;
		}
	}
	words
}

fn hash_loop(input: Vec<u8>, hash: &mut [u32; 4]) {
	let mut i = 0;

	while i < input.len() {
		let words: Vec<u32>  = cut_to_u32_words(&input, i);

		let mut a: u32 = hash[0];
		let mut b: u32 = hash[1];
		let mut c: u32 = hash[2];
		let mut d: u32 = hash[3];

		for j in 0..64 {
			let mut e: u32 = 0;
			let mut k: u32 = 0;

			match j / 16 {
				0 => { 
					e = f_func(b, c, d);
					k = j;
				},
				1 => {
					e = g_func(b, c, d);
					k = (j * 5 + 1) % 16;
				},
				2 => {
					e = h_func(b, c, d);
					k = (j * 3 + 5) % 16;
				},
				3 => {
					e = i_func(b, c, d);
					k = (j * 7) % 16;
				},
				_ => (),
			}

			let tmp: u32 = b.wrapping_add(
				rotate_left(
					a.wrapping_add(e)
					.wrapping_add(K[j as usize])
					.wrapping_add(words[k as usize]),
						 S[j as usize]));
			
			a = d;
			d = c;
			c = b;
			b = tmp;
		}
		hash[0] = hash[0].wrapping_add(a);
		hash[1] = hash[1].wrapping_add(b);
		hash[2] = hash[2].wrapping_add(c);
		hash[3] = hash[3].wrapping_add(d);
		
		i += 64;
	}
}

fn read_from_stdin() -> Result<String>
{
	let mut out = String::new();

	let result = io::stdin().read_to_string(&mut out);
	
	match result {
		Ok(_) => Ok(out),
		Err(error) => Err(Box::new(error))
	}
}

fn get_input(args: &Vec<String>) -> Result<String> {
	match args.len() {
		1 => read_from_stdin(),
		2 => {
			if args[1] == "-s" {				
				return Err(Box::new(InputError));
			}
			let infile = fs::File::open(args[1].clone())?;
			
			let mut buf_reader = BufReader::new(infile);
			let mut input = String::new();
			let result = buf_reader.read_to_string(&mut input);
			match result {
				Ok(_) => Ok(input),
				Err(error) => Err(Box::new(error)),
			}
		},
		3 => {
			if args[1] != "-s" {
				return Err(Box::new(InputError));
			}
			Ok(String::from(args[2].clone()))
		},
		_ => { 
			Err(Box::new(InputError))			
		},
	}
}
fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	
	let input = get_input(&args).unwrap_or_else(|error| {
		eprintln!("{error}");
		panic!("{error:?}");
	});
	
	let mut hash: [u32; 4] = [A, B, C, D];
	
	let padded: Vec<u8> = make_padded_vec(&input);
	
	hash_loop(padded, &mut hash);
	
    println!( "{:08x}{:08x}{:08x}{:08x}",
        hash[0].swap_bytes(), hash[1].swap_bytes(),
        hash[2].swap_bytes(), hash[3].swap_bytes()
    );

	Ok(())
}
