use player::Player;
use dice::Dice;

pub fn jail_roll(jailed_player: &mut Player) {
    println!("{} is in jail!", jailed_player.name);

    jailed_player.jail_count += 1;

    let dice = Dice::roll();

    if dice.is_doubles() {
        println!("{} got out of jail, lucky bastard.", jailed_player.name);
        jailed_player.jail_count = 0;
        jailed_player.in_jail = false;
    }
    if jailed_player.jail_count == 3 {
        if dice.is_doubles() {
            println!("Lucky!, {} got out of jail at the last moment!", jailed_player.name);
        }
        else {
            println!("{} must pay $50 to get out.", jailed_player.name);
        }
        jailed_player.jail_count = 0;
        jailed_player.in_jail = false;
    }
}
