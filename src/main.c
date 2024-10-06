/*
Title: Number to Word Converter
Author: Nathan Cavalli
Date Started: 23/10/2020
Description: Code to convert any positive integer < 1 thousand to its respective word-form
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char* convert_to_words(num){

	char strNum[7];
	sprintf(strNum, "%i", num);

	// Amalgamated Array (ones and teens)
	char *digits_below_twenty[] = {"", "one", "two",
								   "three", "four","five",
								   "six", "seven", "eight", "nine",
								   "ten", "eleven", "twelve",
								   "thirteen", "fourteen",
								   "fifteen", "sixteen",
								   "seventeen", "eighteen", "nineteen"};

	char *tens_multiple[] = {"", "ten", "twenty", "thirty", "forty", "fifty",
							 "sixty", "seventy", "eighty", "ninety"};

	char *tens_power[] = {"", "thousand", "million"};


	if (num < 20)
		{return digits_below_twenty[num];}
	else if (num < 100){
		char *firstWord = tens_multiple[strNum[0] - '0'];
		char *secondWord = digits_below_twenty[strNum[1] - '0'];
		// This is horrible design, should be good for now though
		char *number;
		number = (char *) malloc(sizeof(firstWord) + sizeof(secondWord) + 1);
		//Determining Formatting
		if (secondWord == "")
			{sprintf(number, "%s", firstWord);}
		else
			{sprintf(number, "%s-%s", firstWord, secondWord);}
		return number;
	}
	else if (num < 1000){
		// This all needs to be worked out for teens
		char *firstWord = digits_below_twenty[strNum[0] - '0'];
		char *secondWord = tens_multiple[strNum[1] - '0'];
		char *thirdWord = digits_below_twenty[strNum[2] - '0'];
		char *number;
		number = (char *) malloc(sizeof(firstWord) + sizeof(secondWord) + sizeof(thirdWord) + 1);
		// Determine Formatting
		if (secondWord == "" && thirdWord == "")
			{sprintf(number, "%s hundred", firstWord);}
		else if (secondWord == "")
			{sprintf(number, "%s hundred and %s", firstWord, thirdWord);}
		else if (thirdWord == "")
			{sprintf(number, "%s hundred and %s", firstWord, secondWord);}
		else
			{sprintf(number, "%s hundred and %s-%s", firstWord, secondWord, thirdWord);}
		return number;
	}
	else
		{return "";}
}

int main(void){
	for (int i = 1; i < 1000; i++){
		printf("%s \n", convert_to_words(i));
	}
	return 0;
}
