use std::io;

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

pub fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input,
        Err(_) => {
            println!("Error with the input, please try again.");
            read_input()
        }
    }
}
