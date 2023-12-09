use std::collections::HashMap;

use crate::utils;
use itertools::Itertools;


//First part of the day
pub fn solve_1() -> u32{
    //input
    let mut lists = utils::read_input_of_the_day_lines(3);

    //Add lines around the borders to deal with indices problems
    let mut line_dots:String = vec!['.'; lists[0].len()].into_iter().collect();
    //last line =
    lists.push(line_dots.clone());
    //first line =
    lists.insert(0,line_dots.clone());
    //left/right
    lists.iter_mut().map(|line| {line.insert(0, '.'); line.push('.')})
                    .collect_vec();

    //Trnasform into array of chars
    let array_chars = lists.iter().map(|line| line.chars().collect_vec()).collect_vec();

    let mut temp_number = String::new();
    let mut is_current_number_valid = false;
    let mut in_number = false;
    let mut sum = 0;

    //Iterate over each character of the array
    for row in 1..array_chars.len(){
        for col in 1..(array_chars[0].len()){
            //check if we are in a number
            println!("{} {} {}", row, col, array_chars[row][col]);
            if array_chars[row][col].is_numeric(){
                temp_number.push(array_chars[row][col]);
                if !in_number{
                    in_number = true
                }
                //if the current number is not valid
                //We check the neightbours to look for symbols:
                if !is_current_number_valid {
                    for neighbour in [array_chars[row + 1][col],
                                         array_chars[row][col+1],
                                         array_chars[row + 1][col+1],
                                         array_chars[row-1][col],
                                         array_chars[row][col-1],
                                         array_chars[row-1][col-1],
                                         array_chars[row+1][col-1],
                                         array_chars[row-1][col+1]]{
                    if !neighbour.is_numeric() && neighbour != '.'{
                        println!("{}", neighbour);
                        is_current_number_valid = true
                        } 
                    }
                }

            }//If it is not a character and we are in a number and number is valid
            //we push the number into the sum, and we are now not in a number
            else if in_number{
                //No in a number anymore
                in_number = false;
                //Add to the sum
                if is_current_number_valid{
                    println!("we add {}", temp_number);
                    sum = sum + temp_number.parse::<u32>().unwrap();
                }
                //In any case we reset the current number:
                temp_number.clear();
                is_current_number_valid = false;
            }
        }
    } 
    return  sum;
}

//First part of the day
pub fn solve_2() -> u32{
        //input
        let mut lists = utils::read_input_of_the_day_lines(3);

        //Add lines around the borders to deal with indices problems
        let mut line_dots:String = vec!['.'; lists[0].len()].into_iter().collect();
        //last line =
        lists.push(line_dots.clone());
        //first line =
        lists.insert(0,line_dots.clone());
        //left/right
        lists.iter_mut().map(|line| {line.insert(0, '.'); line.push('.')})
                        .collect_vec();
    
        //Transform into array of chars
        let array_chars = lists.iter().map(|line| line.chars().collect_vec()).collect_vec();
    
        let mut temp_number = String::new();
        let mut is_current_number_valid = false;
        let mut in_number = false;
        let mut sum = 0;

        //Hashmap of all the potential gears, consisiting of key:(i, j) value: [val_n 1, val_2, ...]
        let potential_gears: HashMap<(usize, usize), Vec<u32>>  = HashMap::new();
    
        //Iterate over each character of the array
        for row in 1..array_chars.len(){
            for col in 1..(array_chars[0].len()){
                //check if we are in a number
                println!("{} {} {}", row, col, array_chars[row][col]);
                if array_chars[row][col].is_numeric(){
                    temp_number.push(array_chars[row][col]);
                    if !in_number{
                        in_number = true
                    }
                    //We check the neightbours to look for *:
                    for neighbour in [array_chars[row + 1][col],
                                             array_chars[row][col+1],
                                             array_chars[row + 1][col+1],
                                             array_chars[row-1][col],
                                             array_chars[row][col-1],
                                             array_chars[row-1][col-1],
                                             array_chars[row+1][col-1],
                                             array_chars[row-1][col+1]]{
                        if neighbour == '*'{
                            //Add entry if potential gear dont exists
                            potential_gears.entry(&(row, col)).or_insert()
                            if potential_gears.contains_key(&(row, col)){
                                potential_gears.entry(&(row, col))
                            }
                        } 
                    }
                }//If it is not a character and we are in a number and number is valid
                //we push the number into the sum, and we are now not in a number
                else if in_number{
                    //No in a number anymore
                    in_number = false;
                    //Add to the sum
                    if is_current_number_valid{
                        println!("we add {}", temp_number);
                        sum = sum + temp_number.parse::<u32>().unwrap();
                    }
                    //In any case we reset the current number:
                    temp_number.clear();
                    is_current_number_valid = false;
                }
            }
        } 
        return  sum;
}

mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){

    }

    #[test]
    //Test second part of the puzzle
    fn test_solve_2 (){
        
        
    }

}