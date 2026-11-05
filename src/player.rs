use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

use crate::game::{Move, NUM_ACTIONS};

pub struct Player {
    regret_sum: [f32; NUM_ACTIONS],
    strategy_sum: [f32; NUM_ACTIONS],
}

impl Player {
    pub fn new() -> Self {
        Self {
            regret_sum: [0.0; NUM_ACTIONS],
            strategy_sum: [0.0; NUM_ACTIONS],
        }
    }

    pub fn update_regret(&mut self, self_move: Move, opp_move: Move) {
        let played = self_move.utility(opp_move);
        for (regret, action) in self.regret_sum.iter_mut().zip(Move::ALL) {
            *regret += action.utility(opp_move) - played;
        }
    }

    fn compute_strategy(&mut self) -> [f32; NUM_ACTIONS] {
        let mut strategy = self.regret_sum.map(|r| r.max(0.0));
        let norm_sum: f32 = strategy.iter().sum();

        if norm_sum > 0.0 {
            for s in strategy.iter_mut() {
                *s /= norm_sum;
            }
        } else {
            strategy = [1.0 / NUM_ACTIONS as f32; NUM_ACTIONS];
        }

        for (sum, s) in self.strategy_sum.iter_mut().zip(strategy) {
            *sum += s;
        }
        strategy
    }

    pub fn average_strategy(&self) -> [f32; NUM_ACTIONS] {
        let mut strategy = self.strategy_sum;
        let norm_sum: f32 = strategy.iter().sum();

        if norm_sum > 0.0 {
            for s in strategy.iter_mut() {
                *s /= norm_sum;
            }
            strategy
        } else {
            [1.0 / NUM_ACTIONS as f32; NUM_ACTIONS]
        }
    }

    pub fn sample_action(&mut self) -> Move {
        let strategy = self.compute_strategy();
        let mut rng = rand::rng();
        let dist = WeightedIndex::new(strategy).expect("strategy should be a valid distribution");
        let index = dist.sample(&mut rng);

        Move::ALL[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_strategy_without_regret() {
        let mut player = Player::new();
        assert_eq!(player.compute_strategy(), [1.0 / 3.0; NUM_ACTIONS]);
    }

    #[test]
    fn regret_after_rock_loses_to_paper() {
        let mut player = Player::new();
        player.update_regret(Move::Rock, Move::Paper);

        assert_eq!(player.regret_sum, [0.0, 1.0, 2.0]);
        assert_eq!(player.compute_strategy(), [0.0, 1.0 / 3.0, 2.0 / 3.0]);
    }
}
