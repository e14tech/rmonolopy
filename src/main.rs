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
