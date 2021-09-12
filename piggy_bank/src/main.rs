//piggy bank custom type, stores coins
struct PiggyBank {
    coins: [Change; 5], //Q: how to read in a generic array?\
    value: f32,
}

fn build_piggy_bank(coins: [Change; 5]) -> PiggyBank {
    PiggyBank { coins, value: 0.0 }
}

impl PiggyBank {
    //Calculate the total value of coins in piggy bank
    fn total_value(&mut self) -> f32 {
        for (_i, change) in self.coins.iter().enumerate() {
            match change {
                Change::Penny => {
                    self.value += 0.01;
                }
                Change::Nickel => {
                    self.value += 0.05;
                }
                Change::Dime => {
                    self.value += 0.10;
                }
                Change::Quarter => {
                    self.value += 0.25;
                }
            };
        }
        self.value
    }
}

//types of money in the piggy bank
enum Change {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let my_coins = [
        Change::Quarter,
        Change::Quarter,
        Change::Penny,
        Change::Nickel,
        Change::Dime,
    ];

    let mut my_pig_bank = build_piggy_bank(my_coins);
    println!("total value: ${}", my_pig_bank.total_value());
}
