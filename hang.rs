use std::{io, path};
use std::rand;
use std::rand::Rng;



fn draw(level: uint){
	let mut myfile: ~str = level.to_str();
	myfile = myfile + ".txt";
	  
	let read: Result<@Reader, ~str>;
	read = io::file_reader(~path::Path(myfile));

	if read.is_ok() {
	let file = read.unwrap();
	let text: ~[~str] = file.read_lines();
        for i in range(0, text.len()) {	
		println(text[i]);
        }
	}
	
}

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

fn isDuplicate(guess: char) -> bool{
	for uint::range(0,guesses.length()) |b|{
		let ch = copy(guesses[b]);
		match ch {
			guess => {return true;}
			_ => {}
		}
	}
	return false;
}

//fn searchChar(guess: char) -> bool{
	
//}




fn main() {
        let mut rng = rand::task_rng();
	let dictionary: ~[~str] = load(~"words.txt");
	let max_size: uint = dictionary.len();
        let n: uint = rng.gen_integer_range(0u, max_size);
	let word: ~str = dictionary[n];	
	let word_len: uint = word.char_len();
	
	//Stores the guessed word
	let mut i = 0;	
	//let mut guessedWord: ~[~str] = ~[~""];
	let mut guessedWord: ~[char] = ~[];
	while (i < word_len) {
        	guessedWord = guessedWord + ~['_'];
		i += 1;
        }
        println(guessedWord.to_str());
	println(word);

	let mut guesses: ~[char] = ~[];
	      
        
	let mut j = 0;
        let max_guess : uint = 6;
	while (j < max_guess){

		let line = io::stdin().read_line();
		let guess_char: char = line.char_at(0);
		println(fmt!("%? guessed: ", guess_char));
		guesses = guesses + ~['a', 'b'];
		println(isDuplicate(guess_char).to_str());
		let has_char: bool = word.contains_char(guess_char);
	        draw(j+1);
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
			println("");
		}
		else{
			println("nope.");			
			j += 1;
		}
	}
	println("Out of guesses - you lose.");

}
