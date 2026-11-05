pub const NUM_ACTIONS: usize = 3;

// The order matters: each move is beaten by the next one (Paper beats Rock, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub const ALL: [Move; NUM_ACTIONS] = [Move::Rock, Move::Paper, Move::Scissors];

    // Payoff of playing `self` against `other`: 1 for a win, 0 for a draw, -1 for a loss
    pub fn utility(self, other: Move) -> f32 {
        let (a, b) = (self as usize, other as usize);
        if a == b {
            0.0
        } else if a == (b + 1) % NUM_ACTIONS {
            1.0
        } else {
            -1.0
        }
    }
}
