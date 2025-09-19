use env_logger::Env;

use crate::ui::ui_state::UiState;

pub mod ui;

#[tokio::main]
async fn main() -> iced::Result {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    iced::run("Iota Desktop", UiState::update, UiState::view)
}
