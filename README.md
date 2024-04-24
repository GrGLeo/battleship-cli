# Battleship Game

Welcome to Battleship! This is a simple command-line implementation of the classic game Battleship, where you'll strategically deploy your fleet of ships and attempt to sink your opponent's ships before they sink yours.

## How to Play

1. **Setup**: Place your ships on the board by entering coordinates for each ship. Ships can be placed vertically or horizontally, but cannot overlap.

2. **Gameplay**: Take turns with your opponent firing shots at their ships. Try to guess where their ships are hidden and sink them all!

3. **Winning**: The game ends when one player's ships have all been sunk. If it's your opponent's ships that are all at the bottom of the sea, congratulations—you win!

## Features

- Simple command-line interface for easy gameplay.
- The game displays both the players's ships board and hit board, providing a comprehensive view of the game state.
- Fun and engaging gameplay for all ages.

## Getting Started

To get started with Battleship, simply clone this repository and run the game from your terminal. No additional dependencies required!

```bash
git clone https://github.com/your-username/battleship-cli.git
cd battleship-cli
cargo run
```

## Roadmap

Here's what's planned for future versions of Battleship:
### Player side
- [x] Better error handling on player moves to provide clearer feedback.  
### Enhanced AI: Improve the bot AI for a more challenging single-player experience.
- [x] Ship placement is random
- [ ] Better shooting position
### Multiplayer
- [ ] Multiplayer mode: Implement network multiplayer functionality so players can compete against each other online.
