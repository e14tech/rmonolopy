extern crate rand;

mod input;
mod dice;
mod player;
mod jail;
mod game;

use player::Player;

fn  main() {
    let number_of_players = input::get_number_of_players();

    let mut players: Vec<Player> = Vec::new();

    for i in 0..number_of_players {
        let player_name = input::get_player_name(i);
        players.push(Player::new(player_name));
    }
    let mut turn: usize = 0;
    loop {
        if turn >= number_of_players {
            turn = 0;
        }
        game::game_loop(&mut players[turn]);
        turn += 1;

        if !input::quit_input() {
            break;
        }
    }
}

/*struct Player {
    name: String,
    doubles_roll: u8,
    jail_count: u8,
    in_jail: bool,
}

fn create_player(name: String) -> Player {
    Player {
        name,
        doubles_roll: 0,
        jail_count: 0,
        in_jail: false,
    }
}

fn jail_roll(jailed_player: &mut Player) -> bool {
    println!("{} is in jail", jailed_player.name);
    jailed_player.jail_count += 1;

    let dice = Dice::roll();

    if dice.is_doubles() {
        println!("{} got out of jail!", jailed_player.name);
        jailed_player.jail_count = 0;
        return false;
    }
    if jailed_player.jail_count == 3 {
        println!("You must pay $50 to get out now.");
        jailed_player.jail_count = 0;
        return false;
    }
    true
}

fn printnls(newlines: u8) {
    for _ in 0..newlines {
        print!("\n");
    }
}

fn press_enter() -> bool {
    println!("Press ENTER or RETURN to continue.");
    println!("Press a key before ENTER or RETURN to quit.");

    let mut quit_input = String::new();
    io::stdin().read_line(&mut quit_input)
    .expect("Failed to read line.");

    if quit_input.starts_with('\n') {
        return true;
    }
    else {
        return false;
    }
}

//fn continue_loop() {
//    game_loop();
//}

//fn game_loop() {
//
//}


fn main() {
    let number_of_players = input::get_number_of_players();

    let mut player_name: Vec<String> = Vec::new();
    for i in 0..number_of_players {
        let mut name = input::get_player_name(i);

        player_name.push(name.trim().to_string());
    }
    let mut player: Vec<Player> = Vec::new();
    for i in 0..number_of_players {
        player.push(create_player(player_name[i].to_string()));
    }
    let mut turn: usize = 0;
    loop {
        loop {
            if turn >= number_of_players {
                turn = 0;
            }
            if player[turn].in_jail {
                printnls(100);

                if !jail_roll(&mut player[turn]) {
                    player[turn].in_jail = false;
                }
                turn += 1;
                break;
            }
            printnls(100);

            println!("{}, it's your turn.", player[turn].name);

            let dice = Dice::roll();


            if !dice.is_doubles() {
                player[turn].doubles_roll = 0;
                turn += 1;
                break;
            }
            else {
                player[turn].doubles_roll += 1;

                if player[turn].doubles_roll < 3 {
                    println!("You rolled doubles! Play again!");

                    if player[turn].doubles_roll == 2 {
                        println!("If you roll doubles again, you'll go to jail!");
                    }
                }
                if player[turn].doubles_roll >= 3 {
                    player[turn].in_jail = true;
                    player[turn].doubles_roll = 0;
                    println!("Go to jail!");
                    turn += 1;
                    break;
                }
            }
        }
        if press_enter() {
            continue;
        }
        else {
            break;
        }
    }
}*/
