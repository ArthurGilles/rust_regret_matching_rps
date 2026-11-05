mod game;
mod player;

use game::{Move, NUM_ACTIONS};
use player::Player;

const ITERATIONS: usize = 10_000_000;

fn print_strategy(name: &str, strategy: [f32; NUM_ACTIONS]) {
    print!("{name}:");
    for (m, p) in Move::ALL.iter().zip(strategy) {
        print!("  {m:?}: {:.2}%", p * 100.0);
    }
    println!();
}

fn main() {
    let mut p1 = Player::new();
    let mut p2 = Player::new();

    // Both players learn by playing against each other
    for _ in 0..ITERATIONS {
        let m1 = p1.sample_action();
        let m2 = p2.sample_action();
        p1.update_regret(m1, m2);
        p2.update_regret(m2, m1);
    }

    print_strategy("Player 1", p1.average_strategy());
    print_strategy("Player 2", p2.average_strategy());
}
