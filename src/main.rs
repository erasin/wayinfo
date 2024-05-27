mod args;
mod errors;
mod player;
mod system;
mod tmux;
mod utils;
mod waybar;
mod weather;

fn main() {
    match args::parse() {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
}
