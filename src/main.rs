use std::env;

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

// 	uint8_t *p;
 
	// for (size_t i = 0; i != len; i++)
	// {
    // 	p = (uint8_t *)&hash[i];
    // 	printf("%2.2x%2.2x%2.2x%2.2x", p[0], p[1], p[2], p[3]);
	// }
	// printf("\n");

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

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		eprintln!("usage: {} <string>", args[0]);
		return ;
	}

	let input = String::from(args[1].clone());

	let hash: [u32; 4] = [A, B, C, D];

	let len = ((((input.len() + 8) / 64) + 1) * 64) - 8 + 4;

	println!("{len}");
	// let padded = add_padding(input, len);
	// hash_loop(padded, len, hash);
	print_hash(hash);
}
