use clap::{error::Result, Parser, Subcommand};

use crate::{
    errors::Error,
    player::{self, PlayerCommands},
    system::{self, SystemCommands},
    weather::{self, WeatherArgs},
};

/// 为 wayland 提供信息工具
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    /// [global] Stdout format for waybar
    // #[arg(long, global = true)]
    // waybar: bool,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 系统信息
    #[command(version, about, long_about = None, arg_required_else_help(true))]
    System {
        #[command(subcommand)]
        command: Option<SystemCommands>,
    },

    /// 天气情况
    #[command(version, about, long_about = None, arg_required_else_help(true))]
    Weather(WeatherArgs),

    /// Media Info,
    /// Power by playerctld with dbus
    #[command(version, about, long_about, arg_required_else_help(true))]
    Player {
        #[command(subcommand)]
        command: Option<PlayerCommands>,
    },
}

pub fn parse() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(cmds) => match cmds {
            Commands::System { command } => match command {
                Some(player_cmd) => system::parse(player_cmd),
                None => Ok(()),
            },
            Commands::Weather(args) => weather::parse(args),
            Commands::Player { command } => match command {
                Some(player_cmd) => player::parse(player_cmd),
                None => Ok(()),
            },
        },
        None => Ok(()),
    }
}
