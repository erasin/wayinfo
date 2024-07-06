mod args;
mod errors;
mod ime;
mod player;
mod system;
mod tmux;
mod utils;
mod waybar;
mod weather;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    args::parse()
}
