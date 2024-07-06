use std::time::Duration;

use clap::{Args, Subcommand, ValueEnum};
use dbus::{
    arg,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};

use crate::Result;
use crate::{errors::Error, waybar::WaybarData};

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

fn shuffle_tag(shuffle: bool) -> &'static str {
    match shuffle {
        true => "On",
        false => "Off",
    }
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

impl PlayerLoopMode {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerLoopMode::None => "None",
            PlayerLoopMode::Playlist => "Playlist",
            PlayerLoopMode::Track => "Track",
        }
    }
}

impl From<PlayerLoopMode> for &'static str {
    fn from(val: PlayerLoopMode) -> Self {
        val.as_str()
    }
}

/// 使用 playerctl
pub fn parse(cmd: &PlayerCommands) -> Result<()> {
    let client = PlayerClient::new()?;

    match cmd {
        PlayerCommands::Next => client.next(),
        PlayerCommands::Previous => client.previous(),
        PlayerCommands::Toggle => client.toggle(),
        PlayerCommands::Play => client.play(),
        PlayerCommands::Stop => client.stop(),
        PlayerCommands::Title => client.title(),
        PlayerCommands::Artist => client.artist(),
        PlayerCommands::Album => client.album(),
        PlayerCommands::Status => client.status(),
        PlayerCommands::StatusIcon => client.status_icon(),
        PlayerCommands::Player => client.player(),
        PlayerCommands::Cover => client.cover(),
        PlayerCommands::Position => client.position(),
        PlayerCommands::Positions => client.positions(),
        PlayerCommands::Length => client.length(),
        PlayerCommands::Lengths => client.lengths(),
        PlayerCommands::TrackNumber => client.track_number(),
        PlayerCommands::Shuffle(args) => client.shuffle(args),
        PlayerCommands::Loop(args) => client.loop_mode(args),
        PlayerCommands::Lyrics => client.lyrics(),
        PlayerCommands::Waybar => client.waybar(),
    }
}

enum PlayerAction {
    Play,
    Stop,
    Previous,
    Next,
    Toggle,
    // Seak
    // SetPosition
}

impl PlayerAction {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerAction::Play => "Play",
            PlayerAction::Stop => "Stop",
            PlayerAction::Previous => "Previous",
            PlayerAction::Next => "Next",
            PlayerAction::Toggle => "PlayPause",
        }
    }
}

impl From<PlayerAction> for &'static str {
    fn from(val: PlayerAction) -> Self {
        val.as_str()
    }
}

enum PlayerProperty {
    Metadata,
    PlaybackStatus,
    Position,
    LoopStatus,
    Shuffle,
    // Rate
    // Volume
}

impl PlayerProperty {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerProperty::Metadata => "Metadata",
            PlayerProperty::PlaybackStatus => "PlaybackStatus",
            PlayerProperty::Position => "Position",
            PlayerProperty::LoopStatus => "LoopStatus",
            PlayerProperty::Shuffle => "Shuffle",
        }
    }
}

impl From<PlayerProperty> for &'static str {
    fn from(val: PlayerProperty) -> Self {
        val.as_str()
    }
}

enum PlayerMetadata {
    Title,
    Artist,
    Album,
    ArtUrl,
    TrackNumber,
    Length,
}

impl PlayerMetadata {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerMetadata::Title => "xesam:title",
            PlayerMetadata::Artist => "xesam:artist",
            PlayerMetadata::Album => "xesam:album",
            PlayerMetadata::ArtUrl => "xesam:artUrl",
            PlayerMetadata::TrackNumber => "xesam:trackNumber",
            PlayerMetadata::Length => "mpris:length",
        }
    }
}

impl From<PlayerMetadata> for &'static str {
    fn from(val: PlayerMetadata) -> Self {
        val.as_str()
    }
}

pub struct PlayerClient {
    conn: Connection,
    id: String,
    path: String,
    // proxy: Proxy<'a, &'b Connection>,
}

const DBUS_PLAYER: &str = "org.mpris.MediaPlayer2.Player";
const DBUS_MEDIA_PLAYER: &str = "org.mpris.MediaPlayer2";

impl PlayerClient {
    pub fn new() -> Result<PlayerClient> {
        // if use mpd, with mpd-mpris.
        let id = "org.mpris.MediaPlayer2.playerctld".to_owned();
        let path = "/org/mpris/MediaPlayer2".to_owned();
        let conn = Connection::new_session()?;
        Ok(PlayerClient { conn, id, path })
    }

    fn get_metadata(&self, key: PlayerMetadata) -> Result<String> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let metadata: arg::PropMap = proxy.get(DBUS_PLAYER, PlayerProperty::Metadata.into())?;

        // 'xesam:artist': <['milet']>,
        // 'mpris:artUrl': <'file:///../.cache/covers/xxx.png'>,
        // 'xesam:title': <'Who I Am'>,
        // 'mpris:length': <'203000000'>,
        // 'xesam:album': <'Who I Am'>

        let err_none = Error::Player {
            msg: "没有找到相应的标签".to_owned(),
        };

        match key {
            PlayerMetadata::Artist => {
                let strs: Option<&Vec<String>> = arg::prop_cast(&metadata, key.as_str());

                if let Some(strs) = strs {
                    Ok(strs.join(","))
                } else {
                    Err(err_none.into())
                }
            }

            // string
            _ => {
                let str: Option<&String> = arg::prop_cast(&metadata, key.as_str());

                if let Some(str) = str {
                    Ok(str.clone())
                } else {
                    Err(err_none.into())
                }
            }
        }
    }

    fn get_property_string(&self, key: PlayerProperty) -> Result<String> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: String = proxy.get(DBUS_PLAYER, key.into())?;

        Ok(status)
    }

    fn set_property_string(&self, key: &str, value: &str) -> Result<()> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        proxy.set(DBUS_PLAYER, key, value)?;

        Ok(())
    }

    fn action(&self, key: PlayerAction) -> Result<()> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let (): () = proxy.method_call(DBUS_PLAYER, key.as_str(), ())?;

        Ok(())
    }

    /// 播放器 id
    pub fn player(&self) -> Result<()> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        // let id: String = proxy.get("org.mpris.MediaPlayer2", "Identity")?;
        let id: String = proxy.get(DBUS_MEDIA_PLAYER, "DesktopEntry")?;

        println!("{id}");

        Ok(())
    }

    pub fn next(&self) -> Result<()> {
        self.action(PlayerAction::Next)
    }

    pub fn previous(&self) -> Result<()> {
        self.action(PlayerAction::Previous)
    }

    pub fn toggle(&self) -> Result<()> {
        self.action(PlayerAction::Toggle)
    }

    pub fn play(&self) -> Result<()> {
        self.action(PlayerAction::Play)
    }

    pub fn stop(&self) -> Result<()> {
        self.action(PlayerAction::Stop)
    }

    pub fn title(&self) -> Result<()> {
        let title = self.get_metadata(PlayerMetadata::Title)?;
        println!("{title}");
        Ok(())
    }

    pub fn artist(&self) -> Result<()> {
        let artist = self.get_metadata(PlayerMetadata::Artist)?;
        println!("{artist}");
        Ok(())
    }

    pub fn album(&self) -> Result<()> {
        let album = self.get_metadata(PlayerMetadata::Album)?;
        println!("{album}");
        Ok(())
    }

    pub fn status(&self) -> Result<()> {
        let status = self.get_property_string(PlayerProperty::PlaybackStatus)?;
        println!("{status}");
        Ok(())
    }

    pub fn get_status_icon(&self) -> Result<String> {
        let status = self.get_property_string(PlayerProperty::PlaybackStatus)?;
        let icon = match status.as_str() {
            "Playing" => "",
            // "Paused"| "Stopped",
            _ => "",
        };
        Ok(icon.to_owned())
    }

    pub fn status_icon(&self) -> Result<()> {
        let icon = self.get_status_icon()?;
        println!("{icon}");

        Ok(())
    }

    pub fn cover(&self) -> Result<()> {
        let cover_url = self.get_metadata(PlayerMetadata::ArtUrl)?;
        println!("{cover_url}");
        // TODO write temp file

        Ok(())
    }

    pub fn track_number(&self) -> Result<()> {
        let track_number = self.get_metadata(PlayerMetadata::TrackNumber)?;
        println!("{track_number}");

        Ok(())
    }

    pub fn length(&self) -> Result<()> {
        let length = self.get_metadata(PlayerMetadata::Length)?;
        let duration = length.parse::<u64>().unwrap_or_default();
        print_duration(duration);
        Ok(())
    }

    pub fn lengths(&self) -> Result<()> {
        let length = self.get_metadata(PlayerMetadata::Length)?;
        let duration = length.parse::<u64>().unwrap_or_default();
        println!("{}", duration / 1000000);
        Ok(())
    }

    pub fn get_position(&self) -> Result<u64> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: i64 = proxy.get(DBUS_PLAYER, PlayerProperty::Position.into())?;

        Ok(status as u64)
    }

    pub fn position(&self) -> Result<()> {
        let duration = self.get_position()?;
        print_duration(duration);
        Ok(())
    }

    pub fn positions(&self) -> Result<()> {
        let duration = self.get_position()?;
        println!("{}", duration / 1000000);
        Ok(())
    }

    pub fn get_shuffle(&self) -> Result<bool> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: bool = proxy.get(DBUS_PLAYER, PlayerProperty::Shuffle.into())?;

        Ok(status)
    }

    pub fn set_shuffle(&self, value: bool) -> Result<()> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        proxy.set(DBUS_PLAYER, "Shuffle", value)?;

        Ok(())
    }

    pub fn shuffle(&self, args: &PlayerShuffleArgs) -> Result<()> {
        if args.on {
            self.set_shuffle(true)?;
        } else if args.off {
            self.set_shuffle(false)?;
        } else if args.toggle {
            let shuffle = self.get_shuffle()?;
            self.set_shuffle(!shuffle)?;
        }

        let shuffle = self.get_shuffle()?;
        let shuffle = shuffle_tag(shuffle);

        println!("{shuffle}");

        Ok(())
    }

    pub fn loop_mode(&self, args: &PlayerLoopArgs) -> Result<()> {
        if let Some(mode) = args.mode {
            self.set_property_string(PlayerProperty::LoopStatus.into(), mode.into())?;
        }

        let status = self.get_property_string(PlayerProperty::LoopStatus.into())?;

        println!("{status}");

        Ok(())
    }

    // pub fn get_lyrics(&self) -> Result<String, Error> {}
    pub fn lyrics(&self) -> Result<()> {
        Ok(())
    }

    pub fn waybar(&self) -> Result<()> {
        let title = self.get_metadata(PlayerMetadata::Title)?;
        let artist = self.get_metadata(PlayerMetadata::Artist)?;
        let icon = self.get_status_icon()?;

        let data = WaybarData {
            class: "wayinfo-player".to_owned(),
            text: format!("{icon} {artist}-{title}"),
            alt: None,
            tooltip: None,
            percentage: None,
        };

        let re = serde_json::to_string(&data).unwrap();
        println!("{re}");

        Ok(())
    }
}

fn print_duration(duration: u64) {
    let seconds = duration / 1000000 % 60;
    let minutes = (duration / 1000000 / 60) % 60;
    let hours = duration / 1000000 / 60 / 60;

    if hours > 0 {
        println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    } else {
        println!("{:02}:{:02}", minutes, seconds);
    }
}
