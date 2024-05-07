use std::path::PathBuf;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::io;
use std::env;


fn create_logdir() -> PathBuf  {
    if let Ok(mut exe_path) = env::current_dir() {
        exe_path.push("tmp");
        exe_path.push("logs");

        if let Err(err) = create_dir_all(&exe_path) {
            eprintln!("Failed to create log dir: {}", err);
        }
        else {
            return exe_path
        }
    }
    else {
        eprintln!("Failed to acces directory path!");
    }
    PathBuf::new()
}

pub fn logger(player: &str, coord: &(usize, usize)) {
    let logpath = create_logdir();
    let filepath = logpath.join("game.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath)
        .expect("Failed to open file");
    let log = format!("[{player}] {:?}", coord);
    file.write_all(log.as_bytes()).expect("Failed to write line!");
    writeln!(file).expect("Failed to write new line!");
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

