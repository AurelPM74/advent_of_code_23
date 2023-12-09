use itertools::Itertools;
use std::collections::HashMap;

use crate::utils;

//Parse a game line into an 2d array [["n red", "n green", "n blue"], ...]
pub fn parse_game_line(game: &String) -> Vec<Vec<String>>{
    //remove "Game i: " and splitt the game
    let subgames = utils::substring_delimited(&utils::substring_delimited(&game,':' )[1],';' );
    //Separate along the ;
    let subgames_parsed = subgames.iter()
                                           .map(|subgame|utils::substring_delimited(&subgame,',' ))
                                           .collect_vec();
    return  subgames_parsed;
}

pub fn is_valide_game(game_parsed: Vec<Vec<String>>) -> bool{
    let max_cubes_map =  HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)]);
    for sub_game in game_parsed.iter(){
        for type_cube in sub_game{
            let number_and_color =  utils::substring_delimited(&type_cube,' ' );
            if max_cubes_map.get(&number_and_color[2] as &str).unwrap() < &number_and_color[1].parse::<u32>().unwrap(){
                return false
            }
        }
            
    }
    return true
}
pub fn power_of_game(game_parsed: Vec<Vec<String>>) -> u32{
    let mut max_cubes_map =  HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0)]);
    for sub_game in game_parsed.iter(){
        for type_cube in sub_game{
            let number_and_color =  utils::substring_delimited(&type_cube,' ' );
            let n_cubes = number_and_color[1].parse::<u32>().unwrap();
            let color = number_and_color[2].clone();
            if max_cubes_map.get(&color as &str).unwrap() < &n_cubes{
                *max_cubes_map.get_mut(&color as &str).unwrap() =  n_cubes;
            }
        }
            
    }
    let power = max_cubes_map.get("blue").unwrap()*max_cubes_map.get("red").unwrap()*max_cubes_map.get("green").unwrap();
    return power
}

//First part of the day
pub fn solve_1() -> u32{
    //input
    let lists = utils::read_input_of_the_day_lines(2);
    let result:usize = lists.iter()
                            .enumerate()
                            .map(|(index, game)|(index+1)*(is_valide_game(parse_game_line(game)) as usize))
                            .sum();
    return result as u32 
}

//First part of the day
pub fn solve_2() -> u32{
    let lists = utils::read_input_of_the_day_lines(2);
    let result:usize = lists.iter()
                            .enumerate()
                            .map(|(index, game)|power_of_game(parse_game_line(game)) as usize)
                            .sum();
    return result as u32 
    
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