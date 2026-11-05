# Regret Matching for Rock-Paper-Scissors

A small Rust implementation of **regret matching** applied to rock-paper-scissors. I made it as a first project to learn Rust, and to understand regret matching, the building block of CFR (the algorithm used by poker AIs).

## How it works

Two players start with no knowledge and play 10 million games against each other. After each game, a player computes its *regret* for every move: how much better that move would have done than the move it actually played. These regrets are accumulated, and the next move is picked with probability proportional to the positive regrets.

The strategy played at each step keeps oscillating, but the **average strategy** over all games converges to the Nash equilibrium of rock-paper-scissors: playing each move 1/3 of the time.

## Running

```sh
cargo run --release
```

The debug build also works but is much slower. Example output:

```
Player 1:  Rock: 33.31%  Paper: 33.35%  Scissors: 33.33%
Player 2:  Rock: 33.37%  Paper: 33.30%  Scissors: 33.33%
```

To run the tests:

```sh
cargo test
```

## Project structure

- `src/game.rs`: the moves and the payoff function
- `src/player.rs`: the regret matching player
- `src/main.rs`: the training loop where both players play against each other
