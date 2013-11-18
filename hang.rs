use std::{io, path};
use std::rand;
use std::rand::Rng;

fn end(level: uint, state: ~[~str], word:~str){
	println("");
	println("|H| ||  /|A\\  |N|\\ ||   /|G\\\\");	
	println("|H|H|| /A|_\\\\ |N| \\||  |G|  __ ");
	println("|H| || |A| || |N|  ||   \\|G/|  man ");	
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
	println("");
	println("");

	print("   ");	
	for j in range(0, state.len()){
		print(state[j]);
		print(" ");
	}	
	println("");
	println("");
	println(fmt!("   The word was: %s",word.to_str()));
	println("   PLAY AGAIN? ");
}


fn draw(level: uint, state: ~[~str], past_guess: ~[~str]){
	println("");
	println("|H| ||  /|A\\  |N|\\ ||   /|G\\\\");	
	println("|H|H|| /A|_\\\\ |N| \\||  |G|  __ ");
	println("|H| || |A| || |N|  ||   \\|G/|  man ");	
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
	println("");
	println("");

	print("   ");	
	for j in range(0, state.len()){
		print(state[j]);
		print(" ");
	}	
	println("");
	println("");
	
	print("   Past Guesses: ");
	for j in range(0, past_guess.len()){
		print(past_guess[j]);
                print(" ");
	}
	println("");
	println("");
	print("   Enter Next Guess: ");
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


fn isDuplicate(guess: char, guesses: ~[~str]) -> bool{
	
	for i in range(0,guesses.len()) {
		if (guess.to_str() == guesses[i]){
			return true;
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
	let mut stateWord: ~[~str] = ~[];
	while (i < word_len) {
        	stateWord = stateWord + ~[~"_"];
		i += 1;
        }
	
        //println(fmt!("%s Guessed Word: ", stateWord.to_str()));
	//println(word);

	let mut guesses: ~[~str] = ~[];
	      
        
	let mut j = 0;
	
        let max_guess : uint = 6;
	let mut has_won: bool = false;
	while (j < max_guess  && !has_won){
		let mut is_valid: bool = false;
		has_won = true;
		for i in range(0,stateWord.len()) {
			if(stateWord[i]== ~"_"){
				has_won = false;
			}
		}
		if(has_won){
			end(11, stateWord.clone(), word.clone());
			break;
		}
		draw(j+1, stateWord.clone(), guesses.clone());
		let mut line : ~str = ~"";		
		while(!is_valid){
		  line = io::stdin().read_line();
		  if(line.len() > 0){
			if(line.char_at(0).is_alphabetic()){
				is_valid = true;
			}
             }
		
		}
		let guess_char: char = line.char_at(0);
		//println(fmt!("%? guessed: ", guess_char));
		let has_char: bool = word.contains_char(guess_char);
		
		if(isDuplicate(guess_char, guesses.clone())){
			//println(fmt!("%? Pick another letter! You have already guess ", guess_char.to_str()));
		}else if (has_char){
			guesses.push(guess_char.to_str());
       			let mut i = 0;	
			while (i < word_len) {
				if(word.char_at(i) != guess_char){
        	 			//print("_ ");
                		}else{
		 			stateWord[i] = guess_char.to_str();
				}
				i += 1;	
       			 }
			//println("");
		}
		else{   
			guesses.push(guess_char.to_str());
			//println("nope.");			
			j += 1;
		}
		
	}
	if(j >= max_guess) {end(17, stateWord.clone(), word.clone());}
}
