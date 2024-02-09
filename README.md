# SHIP WRECK

Ship_Wreck is a two-player game implemented in Rust where players
strategically position their ships and take turns guessing and destroying each
other's ships. The objective of the game is to sink the opponent's ship before
they sink yours.

## Gameplay

1. **Setup**
   - Players take turns entering the location of their ship by specifying the
     index of the first cell of their boat in the grid array.
   - The board is represented as a grid which is a one dimensional array, and
     each player's ship is marked on the grid with different indices.

2. **Gameplay**
   - Players alternate turns to guess the location of their opponent's ship.
   - When a player guesses a cell, the game checks if it hits or misses the
     opponent ship's cell. Each boat is comprised of 3 cells in the array.
   - If 3 hits are made, the opponent's ship is destroyed, and the player wins
     the game.
   - The game ends when one player's ship is destroyed.

## Getting Started

To run the game, make sure you have Rust installed on your system. You can
install Rust by following the instructions on the
[official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, follow these steps to play the game:

1. Clone the repository.

2. Navigate to the project directory:
   ```
   cd shipWreck
   ```
3. Build and run the game:
    ```
    cargo run
    ```
4. Follow on-screen instructions. Play with a friend or a foe. 

## Contributing
Contributions are welcome! If you have any ideas for improvements or new features, feel free to open an issue or submit a pull request.

