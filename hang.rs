use std::{io, path};
use std::rand;
use std::rand::Rng;

static LOW:int = 0;
static HIGH:int = 3;


fn load(filename: ~str) -> ~[~str] {
	let read: Result<@Reader, ~str>;
	read = io::file_reader(~path::Path(filename));

	if read.is_ok() {
	let file = read.unwrap();
	return file.read_lines();
	}
 
	println(fmt!("Error in reading the file: %?", read.unwrap_err()));
	return ~[];
}


fn main() {
        let mut rng = rand::task_rng();
	let dictionary: ~[~str] = load(~"words.txt");
	let max_size: uint = dictionary.len();
        let n: uint = rng.gen_integer_range(0u, max_size);
	let word: ~str = dictionary[n];	
	let word_len: uint = word.char_len();
	
	let mut i = 0;	
	while (i < word_len) {
        	print("_ ");
		i += 1;
        }
        println("");
	      
        
	let mut j = 0;
        let max_guess : uint = 8;
	while (j < max_guess){

		let line = io::stdin().read_line();
		let guess_char: char = line.char_at(0);
		println(fmt!("%? guessed: ", guess_char));
		let has_char: bool = word.contains_char(guess_char);
	
		if (has_char){
       			let mut i = 0;	
			while (i < word_len) {
				if(word.char_at(i) != guess_char){
        	 			print("_ ");
                		}else{
		 			print(fmt!("%c", guess_char));
		 			print(" ");
				}
				i += 1;
       			 }
		
		}
		else{
			j += 1;
		}
	}

}
