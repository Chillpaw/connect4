# Connect Four

This is my first substantive Rust project seeking to achieve the following goals:

- Build a connect four clone in the terminal
- Utilise a Bitboard wrapper to track game state
- Clean code management and architecture
- Implement a layer of self-review before pushing changes to the main branch

## Bitboard

This project utilises a Bitboard u64 wrapper to represent the state of the game board. The game engine will consist of 2 Bitboard types (1 for each player).

The Bitboard class implements functions to act as an API to the rest of the game engine opposed to handling a u64 in the main game loop. The ```board.rs``` file also overloads the following logical operators: ```& | ^ !``` for operations between two Bitboards.

## Position

The Position struct keeps track of the board's state.

The core responsibilities of the position struct is to:

- Track the current player
- Assess if moves are valid
- Update Player Bitboard when a move is played


