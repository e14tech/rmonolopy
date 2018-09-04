use rand;
use rand::Rng;

pub struct Dice {
    values: (u8, u8)
}

impl Dice {
    pub fn roll() -> Self {
        let first = roll_dice();
        let second = roll_dice();

        println!("First dice is: {}", first);
        println!("Second dice is: {}", second);

        Dice {
            values: (first, second),
        }
    }
    pub fn is_doubles(&self) -> bool {
        self.values.0 == self.values.1
    }
}
fn roll_dice() -> u8 {
    rand::thread_rng().gen_range(1, 7)
}
