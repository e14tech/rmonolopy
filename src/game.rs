use player::Player;
use jail::jail_roll;
use dice::Dice;

pub fn game_loop(current_player: &mut Player) {
    loop {
        newlns(100);

        if current_player.in_jail {
            jail_roll(current_player);
            break;
        }
        println!("{}, it is your turn.", current_player.name);
        let dice = Dice::roll();

        if !dice.is_doubles() {
            current_player.doubles += 0;
            break;
        }
        else {
            if current_player.doubles < 3 {
                println!("You rolled doubles, play again.");

                if current_player.doubles == 2 {
                    println!("Get doubles one more time and you'll go to jail!");
                }
            }
            else {
                current_player.in_jail = true;
                current_player.doubles = 0;
                println!("Go to jail!");
                break;
            }
        }
    }
}

fn newlns(lns: u8) {
    for _ in 0..lns {
        println!("\n");
    }
}
