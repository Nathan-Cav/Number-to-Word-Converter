/*
Title: Number to Word Converter
Date Started: 2/12/2020
Description: Script to print 1 - 1 million as words and functions to convert positive integers < ~4 billion to their respective word-form
*/

fn convert_segment(num: u16) -> String {
	let str_num = num.to_string();
	
	let digits_below_twenty = ["", "one", "two", 
								   "three", "four","five", 
								   "six", "seven", "eight", "nine", 
								   "ten", "eleven", "twelve", 
								   "thirteen", "fourteen",
								   "fifteen", "sixteen", 
								   "seventeen", "eighteen", "nineteen"];
	
	let tens_multiple = ["", "ten", "twenty", "thirty", "forty", "fifty",
							 "sixty", "seventy", "eighty", "ninety"];
	
	if num < 20 
		{return digits_below_twenty[num as usize].to_string();}
	else if num < 100 {
		let first_word = String::from(tens_multiple[str_num[0..1].parse::<u8>().unwrap() as usize]);
		let second_word = String::from(digits_below_twenty[str_num[1..2].parse::<u8>().unwrap() as usize]);
		// Determine Formatting
		if second_word == "" 
			{return format!("{}", first_word);}
		else 
			{return format!("{}-{}", first_word, second_word);}
	}
	else if num < 1000 {
		let first_num = str_num[0..1].parse::<u8>().unwrap();
		let second_num = str_num[1..2].parse::<u8>().unwrap();
		let third_num = str_num[2..3].parse::<u8>().unwrap();
		
		let first_word = String::from(digits_below_twenty[first_num as usize]);
		let third_word;
		// Edge-case for Teen Numbers
		let second_word = if second_num == 1 {
			let concat_num = second_num * 10 + third_num;
			third_word = String::from("");
			String::from(digits_below_twenty[concat_num as usize])
		}
		else {
			third_word = String::from(digits_below_twenty[third_num as usize]);
			String::from(tens_multiple[second_num as usize])
		};
		// Determine Formatting
		if second_word == "" && third_word == "" 
			{return format!("{} hundred", first_word);}
		else if second_word == "" 
			{return format!("{} hundred and {}", first_word, third_word);}
		else if third_word == "" 
			{return format!("{} hundred and {}", first_word, second_word);}
		else 
			{return format!("{} hundred and {}-{}", first_word, second_word, third_word);}
	}
	else {
		return String::from("");
	}
}

fn convert_to_words(num: u32) -> String {
	let tens_power = ["", "thousand", "million", "billion"];
	
	// Storage for Final Number
	let mut numvec = Vec::new();
	let mut number = String::from("");
	
	// Splitting Number into Groups of 3 Integers
	let mut tmp_num = num;
	let mut segments = Vec::new();
	
	while tmp_num > 0 {
		segments.push(tmp_num % 1000);
		tmp_num /= 1000;
	}
	
	// Processing Segments and Adding Relevant Scalers in Tmp Vector 
	for (i, segment) in segments.iter().enumerate() {
		if tens_power[i] == ""
			{numvec.push(format!("{}", convert_segment(*segment as u16)));}
		else if convert_segment(*segment as u16) == ""
			{continue;}
		else 
			{numvec.push(format!("{} {}", convert_segment(*segment as u16), tens_power[i]));}
	}
	numvec.reverse();
	
	// Formatting Number into String for Return
	for (i, num) in numvec.iter().enumerate() {
		number.push_str(num);
		if segments[0] < 100 && numvec.len() > 1 && i+1 != numvec.len() && numvec[numvec.len()-1] != "" 
			{number.push_str(" and ");}
		else if i+1 != numvec.len() && numvec[numvec.len()-1] != "" 
			{number.push_str(", ");}
		else 
			{continue;}
	}
	return number;
}

fn main() {
	for i in 1..1000001 {
		println!("{}", convert_to_words(i));
	}
}
