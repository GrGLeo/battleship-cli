use std::io;
use regex::Regex;

pub fn input_to_int(input: &str) -> Vec<usize> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut position: Vec<usize> = Vec::new();

    for part in parts{
        if let (Some::<char>(posy), Some::<char>(posx)) = (part.chars().next(), part.chars().nth(1)) {
            let posy_usize = posy as usize - 'A' as usize;
            let posx_usize = posx.to_digit(10).unwrap();
            position.push(posy_usize);
            position.push(posx_usize.try_into().unwrap());
        }
    }
    position
}

pub fn reorder_position(position: &mut Vec<usize>){
    if position[0] > position [2] {
        position.swap(0,2);
    }
    if position[1] > position [3] {
        position.swap(1,3);
    }
    
}

pub fn read_input(input_type: &str) -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
            if validate_input(&input_type, &input){
                return input
            }
            else {
                println!("Error with the input, please try again.");
                read_input(&input_type)
            }
        }

        Err(_) => {
            println!("Error with the input, please try again.");
            read_input(&input_type)
        }
    }
}

fn validate_input(input_type: &str, input: &String) -> bool {
    if input_type == "ship" {
        let re = Regex::new(r"^[A-J][0-9]\s[A-J][0-9]$").unwrap();
        return re.is_match(input)
    }
    else if input_type== "fire" {
        let re = Regex::new(r"^[A-J][0-9]$").unwrap();
        re.is_match(input)
    }
    else { 
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_int_valid_input() {
        let input = "A1 B2 C3".to_string();
        let result = input_to_int(&input);
        assert_eq!(result, vec![0, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_reorder_position_needs_reordering() {
        let mut positions = vec![3, 2, 1, 0]; // Represents positions (3,2) and (1,0)
        reorder_position(&mut positions);
        assert_eq!(positions, vec![1, 0, 3, 2]); // Expected reordered positions
    }
}

