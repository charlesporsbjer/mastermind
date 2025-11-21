use crate::Bot;
use crate::GameConfig;
use crate::GameMode;
use crate::Gamestate;
use crate::StartupAction;
use crate::loadgame::handle_load;
use crate::types::Line;

pub fn handle_startup(action: StartupAction) -> (Gamestate, Option<Bot>) {
    match action {
        StartupAction::LoadGame => handle_load(),

        StartupAction::NewGame(cfg) => {
            let gs = init_gamestate(&cfg);
            let bot = init_bot(&gs);

            (gs, bot)
        }
    }
}

pub fn init_gamestate(cfg: &GameConfig) -> Gamestate {
    let no_of_pegs = cfg.pegs_in_a_line as usize;

    Gamestate::new(
        cfg.game_mode,
        cfg.number_of_guesses,
        no_of_pegs,
        Line::empty(no_of_pegs),
        cfg.is_empty_pegs_allowed,
        cfg.is_bot_guesser,
    )
}

pub fn init_bot(gamestate: &Gamestate) -> Option<Bot> {
    let needs_bot = (gamestate.is_bot_only_guesser
        && gamestate.game_mode == GameMode::SinglePlayer)
        || gamestate.game_mode == GameMode::SpectateBot
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
