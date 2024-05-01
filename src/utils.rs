use std::io;

pub fn print_win() {
    println!("                                      # #  ( )");
    println!("                                   ___#_#___|__");
    println!("                               _  |____________|  _");
    println!("                        _=====| | |            | | |==== _");
    println!("                  =====| |.---------------------------. | |====");
    println!("    <--------------------'   .  .  .  .  .  .  .  .   '--------------/");
    println!(r"      \                                                             /");
    println!(r"       \_______________________________________________WWS_________/");
    println!("   wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww");
    println!(" wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww");
    println!("    wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww ");
    println!("");
    println!(r"+ ----------------------------------------------------------------------+");
    println!(r"|  /$$     /$$                                                          | ");
    println!(r"| |  $$   /$$/                                                          |");
    println!(r"|  \  $$ /$$//$$$$$$  /$$   /$$       /$$  /$$  /$$  /$$$$$$  /$$$$$$$  |");
    println!(r"|   \  $$$$//$$__  $$| $$  | $$      | $$ | $$ | $$ /$$__  $$| $$__  $$ |");
    println!(r"|    \  $$/| $$  \ $$| $$  | $$      | $$ | $$ | $$| $$  \ $$| $$  \ $$ |");
    println!(r"|     | $$ | $$  | $$| $$  | $$      | $$ | $$ | $$| $$  | $$| $$  | $$ |");
    println!(r"|     | $$ |  $$$$$$/|  $$$$$$/      |  $$$$$/$$$$/|  $$$$$$/| $$  | $$ |");
    println!(r"|     |__/  \______/  \______/        \_____/\___/  \______/ |__/  |__/ |");
    println!(r"+ ----------------------------------------------------------------------+");
}

pub fn print_lost() {
    println!(r"+------------------------------------------------------------------------+");
    println!(r"|  /$$     /$$                        /$$                       /$$      |"); 
    println!(r"| |  $$   /$$/                       | $$                      | $$      |"); 
    println!(r"|  \  $$ /$$//$$$$$$  /$$   /$$      | $$  /$$$$$$   /$$$$$$$ /$$$$$$    |"); 
    println!(r"|   \  $$$$//$$__  $$| $$  | $$      | $$ /$$__  $$ /$$_____/|_  $$_/    |"); 
    println!(r"|    \  $$/| $$  \ $$| $$  | $$      | $$| $$  \ $$|  $$$$$$   | $$      |"); 
    println!(r"|     | $$ | $$  | $$| $$  | $$      | $$| $$  | $$ \____  $$  | $$ /$$  |");
    println!(r"|     | $$ |  $$$$$$/|  $$$$$$/      | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/  |");
    println!(r"|     |__/  \______/  \______/       |__/ \______/ |_______/    \___/    |"); 
    println!(r"+------------------------------------------------------------------------+");
}

pub fn start_again() -> bool {
    loop {
        println!("Do you want to start again? [y]es [n]o");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                match input {
                    "y" => return true,
                    "n" => return false,
                    _ => println!("Invalid input. Please enter 'y' or 'n'."),
                }
            }
            Err(_) => println!("Error reading input. Please try again."),
        }
    }
}
