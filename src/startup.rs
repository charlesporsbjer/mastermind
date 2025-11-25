/*
    Startup module.

    Responsible for creating the initial game state when the program begins.

    It handles:
    - Loading an existing save file when the user chooses "Load Game".
    - Creating a fresh Gamestate when starting a new game.
    - Initializing an optional Bot based on the selected GameMode.

    Public API:
    - handle_startup: main entry point for all startup logic.

    Internal helpers (private):
    - init_gamestate: builds a new Gamestate from a GameConfig.
    - init_bot: creates a Bot when the selected GameMode requires one.
*/

use crate::{
    bot::Bot,
    gameconfig::GameConfig,
    gamestate::Gamestate,
    loadgame::handle_load,
    types::{GameMode, Line},
    usersetup::StartupAction,
};

pub fn handle_startup(action: StartupAction) -> (Gamestate, Option<Bot>) {
    match action {
        StartupAction::LoadGame => {
            let gs = handle_load();
            let bot = init_bot(&gs);
            (gs, bot)
        }
        StartupAction::NewGame(cfg) => {
            let gs = init_gamestate(&cfg);
            let bot = init_bot(&gs);

            (gs, bot)
        }
    }
}

fn init_gamestate(cfg: &GameConfig) -> Gamestate {
    let no_of_pegs = cfg.pegs_in_a_line as usize;

    Gamestate::new(
        cfg.game_mode,
        cfg.number_of_guesses,
        no_of_pegs,
        Line::empty(no_of_pegs),
        cfg.is_empty_pegs_allowed,
    )
}

fn init_bot(gamestate: &Gamestate) -> Option<Bot> {
    let needs_bot = gamestate.game_mode == GameMode::SpectateBot
        || gamestate.game_mode == GameMode::PlayerVsBot;
    if needs_bot {
        Some(Bot::new(
            gamestate.is_empty_allowed,
            gamestate.pegs_in_a_line,
        ))
    } else {
        None
    }
}
