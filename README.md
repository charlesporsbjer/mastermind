# Mastermind (Rust)

A working version of **Mastermind**, built to learn and explore **Rust**.  
Fully playable, with multiple modes, bot logic, saving/loading, and console graphics.

## Features

### Game Modes
- Player vs Player  
- Player vs Bot  
- Solo  
- Spectate Bot

### Bot
- Uses an *imperfect* version of **Knuth’s Mastermind algorithm**  
- Simulates all possible outcomes  
- Solves the code in **≤6 guesses reliably**  
- Stays consistent even with large peg counts  
- First guess has a random element

### Saving & Loading
- Save / Continue / Quit after every round  
- Choose New Game or Load Game on startup

### Setup Options
- Configure via **config file**  
- Configure **in-game**

### Core Rules
- 6 peg colors  
- Breaker: guess the hidden code  
- Maker: set the hidden code

### Rule Tweaks
- Adjustable code length  
- Adjustable max guesses  
- Option to allow missing pegs

### Graphics
- Console board drawn dynamically

### Input / Output
- Robust invalid-input handling  
- Clear and helpful user guidance

### Bonus
- Estimates how long the bot will take to break the code in Spectate Bot mode

---

## Coming Features
- `--help` command  
- Cleaner and prettier console output  
- Hide Maker’s code from the Breaker  
- Store bot simulations using integers instead of HashSets (better memory usage)  
- More peg colors  
- Switch to a graphics library  
- General code cleanup  
- Binary releases

## Known Bugs
- Some data is skewed when loading a saved game

