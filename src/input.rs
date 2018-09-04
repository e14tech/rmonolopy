use std::io;

pub fn get_number_of_players() -> usize {
    println!("How many players?");
    loop {
        match input().trim().parse() {
            Ok(0) => {
                println!("You can't play and not play at the same time. This isn't the Copenhagen interpretation of quantum mechanics.");
            },
            Ok(1) => {
                println!("Sounds pretty boring but whatever.");
                return 1;
            },
            Ok(nop) => return nop,
            Err(_) => {
                println!("Please enter an integer.");
            },
        };
    }
}

pub fn get_player_name(player_number: usize) -> String {
    println!("Please enter name for player {}.", player_number + 1);
    input()
}

pub fn quit_input() -> bool {
    println!("Press ENTER or RETURN to continue.");
    println!("Press any key before ENTER or RETURN to quit.");
    let quit_input = input();

    quit_input.starts_with('\n')
}

fn input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line.");

    user_input
}
