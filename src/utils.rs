use std::io;

pub fn print_win() {
    println!(r"                                   .''.");
    println!(r"       .''.      .        *''*    :_\/_:     .");
    println!(r"      :_\/_:   _\(/_  .:.*_\/_*   : /\ :  .'.:.'.");
    println!(r"  .''.: /\ :    /)\   ':'* /\ *  : '..'.  -=:o:=-");
    println!(r" :_\/_:'.:::.  | ' *''*    * '.\'/.'_\(/_'.':'.'");
    println!(r" : /\ : :::::  =  *_\/_*     -= o =- /)\    '  *");
    println!(r"  '..'  ':::' === * /\ *     .'/.\'.  ' ._____");
    println!(r"      *        |   *..*         :       |.   |' .---||");
    println!(r"        *      |                 .---|  ||   |.-|    |");
    println!(r"        *      |  .--|      .--  |   |  |    ||      |");
    println!(r"     .-----.   |  |. |  ||  |  | |   |  |    ||      |");
    println!(r" ___|       | /|\ |  |-.||.    |-|   |-.|    |`      |----");
    println!(r"jgs~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!(r"  &                    ~-~-~-~-~-~-~-~-~-~   /|");
    println!(r" ejm97    )      ~-~-~-~-~-~-~-~  /|~       /_|\");
    println!(r"        _-H-__  -~-~-~-~-~-~     /_|\    -~======-~");
    println!(r"~-\XXXXXXXXXX/~     ~-~-~-~     /__|_\ ~-~-~-~");
    println!(r"~-~-~-~-~-~    ~-~~-~-~-~-~    ========  ~-~-~-~");
    println!(r"_____________________________________________________________________");
    println!(r" /$$     /$$                                                         ");
    println!(r"|  $$   /$$/                                                         ");
    println!(r" \  $$ /$$//$$$$$$  /$$   /$$       /$$  /$$  /$$  /$$$$$$  /$$$$$$$ ");
    println!(r"  \  $$$$//$$__  $$| $$  | $$      | $$ | $$ | $$ /$$__  $$| $$__  $$");
    println!(r"   \  $$/| $$  \ $$| $$  | $$      | $$ | $$ | $$| $$  \ $$| $$  \ $$");
    println!(r"    | $$ | $$  | $$| $$  | $$      | $$ | $$ | $$| $$  | $$| $$  | $$");
    println!(r"    | $$ |  $$$$$$/|  $$$$$$/      |  $$$$$/$$$$/|  $$$$$$/| $$  | $$");
    println!(r"    |__/  \______/  \______/        \_____/\___/  \______/ |__/  |__/");
    println!(r"_____________________________________________________________________");
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
