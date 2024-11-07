# The Game

A simple turn-based combat game in Rust, where the player can move around the map and interact with an enemy.

## Requirements

- Rust (installed via [rustup](https://rustup.rs/))
- Dependencies:
  - [crossterm](https://crates.io/crates/crossterm) for terminal manipulation and keyboard input.

## How to Run

1. Clone this repository:

```bash
   git clone https://github.com/tysaiwofc/the-rust-game.git
```

2. Navigate to the project directory:

```bash
cd the-rust-game
```

3. Build and run the project:

```bash
cargon run
```

## Controls

W: Move up
A: Move left
S: Move down
D: Move right
K: Attack the enemy and deal damage
Q: Quit the game

## Description

The goal of the game is to move the player around the map, attack the enemy, and avoid obstacles. When the player occupies the same position as the enemy, they take damage. The game ends when the player's health reaches zero.

## Project Structure
map.rs: Defines the map structure and its functionalities.
enemy.rs: Defines the enemy structure and its functionalities.
inventory.rs: Defines the player's inventory structure.
main.rs: The game's entry point, where the game logic and controls are defined.

## License
This project is open-source and distributed under the MIT license.