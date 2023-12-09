use crate::utils;
use std::collections::HashMap;

pub fn find_first_digit(line: &String) -> u32{
    
    for c in line.chars(){
        if c as u32 - 0x30 <= 10{
            return c as u32 - 0x30
        }

    }
    0
}


pub fn find_first_word_or_digit(line: &String, word_map: &HashMap<&str, u32>) -> u32{

    let size_line = line.len();
    for (i, c) in line.chars().enumerate(){
        if c as u32 - 0x30 <= 10{
            return c as u32 - 0x30
        }
        if ((i+3) < size_line) && word_map.get(&line[i..(i+3)]).copied().unwrap_or(10) != 10{
            return word_map.get(&line[i..(i+3)]).copied().unwrap()
        }
        else if ((i+4) < size_line) && (word_map.get(&line[i..(i+4)]).copied().unwrap_or(10) != 10){
            return word_map.get(&line[i..(i+4)]).copied().unwrap()
        }
        else if ((i+5) < size_line) && (word_map.get(&line[i..(i+5)]).copied().unwrap_or(10) != 10){
            return word_map.get(&line[i..(i+5)]).copied().unwrap()
        }
    }
    0
}


//First part of the day
pub fn solve_1() -> u32{

    //input
    let lists = utils::read_input_of_the_day_lines(1);
    let result: u32 = lists.iter()
                        .map(|line| (find_first_digit(&line), find_first_digit(&line.chars().rev().collect())))
                        .map(|(a, b)| 10*a + b)
                        .sum();
    return result
}

//Second part of the day
pub fn solve_2() -> u32{
    //Hashmap of number as string to value
    let word_map =  HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let reverse_word_map = HashMap::from([
        ("orez", 0),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9)
    ]);
    
    //input
    let lists = utils::read_input_of_the_day_lines(1);
    let result: Vec<u32> = lists.iter()
         .map(|line| (find_first_word_or_digit(&line, &word_map), find_first_word_or_digit(&line.chars().rev().collect(), &reverse_word_map)))
         .map(|(a, b)| 10*a + b)
         .collect();
    return result.iter().sum()

}

mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        let input: Vec<String> = vec!["1abc2",
                                      "pqr3stu8vwx",
                                      "a1b2c3d4e5f",
                                      "treb7uchet"].iter()
                                                   .map(|s| s.to_string())
                                                   .collect();

        let result: u32 = input.into_iter()
        .map(|line| (find_first_digit(&line), find_first_digit(&line.chars().rev().collect())))
        .map(|(a, b)| 10*a + b)
        .sum();

        assert_eq!(result, 142);
    }

    #[test]
    //Test second part of the puzzle
    fn test_solve_2 (){
        
        
    }

}