use clap::{error::Result, Args, Parser, Subcommand, ValueEnum};

use crate::{errors::Error, player, system, weather};

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

#[derive(Subcommand)]
pub enum SystemCommands {
    Cpu,
    Cpus,
    Memory(SysMemArgs),
    Disk,
}

#[derive(Args)]
pub struct SysMemArgs {
    /// total memory
    #[arg(short, long)]
    pub total: bool,

    //// used memory
    #[arg(short, long)]
    pub usage: bool,
}

#[derive(Args)]
pub struct WeatherArgs {
    /// 城市
    #[arg(short, long)]
    pub city: String,

    /// 接口密钥
    #[arg(short, long)]
    pub key: Option<String>,

    /// 接口密钥文件
    #[arg(long)]
    pub key_file: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    pub day: usize,

    // #[arg(from_global)]
    #[arg(long)]
    pub waybar: bool,
}

#[derive(Subcommand)]
pub enum PlayerCommands {
    /// Player Identity
    Player,
    /// next song
    Next,
    /// previous song
    Previous,
    /// toggle play or pause
    Toggle,
    /// play
    Play,
    /// stop
    Stop,
    /// Playback status (Playing|Paused|Stopped)
    Status,
    /// Playing   , other 
    StatusIcon,
    /// title of song
    Title,
    /// artist of song
    Artist,
    /// album of song
    Album,
    /// cover of song
    Cover,
    /// Track Number of
    TrackNumber,

    /// Position time at playing
    Position,
    /// Position second at playing
    Positions,
    /// Length of song
    Length,
    /// Length second of song
    Lengths,
    // Percent,
    Shuffle(PlayerShuffleArgs),

    /// 循环模式
    Loop(PlayerLoopArgs),

    /// lyrics
    Lyrics,

    /// waybar format
    Waybar,
}

#[derive(Args)]
pub struct PlayerShuffleArgs {
    #[arg(long)]
    pub on: bool,
    #[arg(long)]
    pub off: bool,
    #[arg(long)]
    pub toggle: bool,
}

#[derive(Args)]
pub struct PlayerLoopArgs {
    #[arg(long, value_enum)]
    pub mode: Option<PlayerLoopMode>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PlayerLoopMode {
    /// 默认
    None,
    /// 循环
    Playlist,
    /// 单曲
    Track,
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
