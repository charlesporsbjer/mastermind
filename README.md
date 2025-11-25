# Mastermind (Rust)

A working version of **Mastermind**, built to learn and explore **Rust**.  
Fully playable, with multiple modes, bot logic, saving/loading, and console graphics.

## Features

### Game Modes
- Player vs Player – Two humans take turns as Code Maker and Code Breaker
- Player vs Bot – Play against an AI bot
- Solo / Practice – Solve codes on your own
- Spectate Bot – Watch the bot play against itself

### Bot
- Uses an *imperfect* version of **Knuth’s Mastermind algorithm**  
- Simulates all possible outcomes to make the "best" guess.
- Solves the code in **≤6 guesses reliably**  
- Stays consistent even with large peg counts  
- First guess has a random element

### Saving & Loading
- Autosave after every round
- Save / Continue / Quit after every round  
- Choose New Game or Load Game on startup
- Save files stored in savegames/ directory

### Setup Options
- Configure via **config file**  
- Configure manually in-game if no config is found or user chooses manual setup

### Core Rules
- Colors: 6 standard colors (White, Black, Red, Green, Blue, Yellow), plus optional Empty
- Breaker: guess the hidden code  
- Maker: set the hidden code

### Rule Tweaks
- Adjustable code length  
- Adjustable max guesses  
- Option to allow missing pegs

### Graphics
- Console board drawn dynamically
- Displays guesses, feedback, and legend for colors

### Input / Output
- Validates input robustly
- Clear and helpful user guidance
- Hides code input in Two-Player mode when necessary

### Bonus
- Estimates how long the bot will take to break the code in Spectate Bot mode

---

## Coming Features
- `--help` command  
- Store bot simulations using integers instead of HashSets (better memory usage)  
- More peg colors  
- Switch to a graphics library  
- Binary releases

