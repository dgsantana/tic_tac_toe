# Tic Tac Toe with Bevy Replicon

This project is a Tic Tac Toe game implemented using the Bevy Replicon library. It features a simple menu system and server discovery functionality.

Original game logic taken from [Tic Tac Toe](https://github.com/projectharmonia/bevy_replicon/blob/master/examples/tic_tac_toe.rs) example on [bevy_replicon](https://github.com/projectharmonia/bevy_replicon).

## Installation

To run this project, you need to have Rust and Cargo installed. Follow these steps to get started:

1. Clone the repository: `git clone https://github.com/dgsantana/tic-tac-toe.git`
2. Navigate to the project directory: `cd tic-tac-toe`
3. Build and run the project: `cargo run`

## How to Play

Once the game is running, you can use the following controls:

- Use the mouse to navigate the menu options.
- Pick one of the options and play on the same computer or on the LAN

## Features

### Game Board

The game board is a 3x3 grid where players can make their moves. It is displayed on the screen and updated in real-time.

### Menu System

The menu system allows players to start a new game, quit the game.

### Server Discovery

The server discovery functionality enables players to discover and connect to available game servers for multiplayer matches.
This avoids having to manually insert the server ip. Easy to reuse on other projects, a single source [file](https://github.com/dgsantana/tic_tac_toe/blob/main/src/network/discovery.rs).

## Contributing

Contributions are welcome! If you have any ideas, bug reports, or feature requests, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
