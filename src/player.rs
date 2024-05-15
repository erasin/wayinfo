use std::time::Duration;

use clap::error::Result;
use dbus::{
    arg,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};

use crate::{
    args::{PlayerCommands, PlayerLoopArgs, PlayerLoopMode, PlayerShuffleArgs},
    errors::Error,
    waybar::WaybarData,
};

/// 使用 playerctl

pub fn parse(cmd: &PlayerCommands) -> Result<(), Error> {
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

pub struct PlayerClient {
    conn: Connection,
    id: String,
    path: String,
    // proxy: Proxy<'a, &'b Connection>,
}

impl PlayerClient {
    pub fn new() -> Result<PlayerClient, Error> {
        let id = "org.mpris.MediaPlayer2.playerctld".to_owned();
        let path = "/org/mpris/MediaPlayer2".to_owned();
        let conn = Connection::new_session()?;
        return Ok(PlayerClient { conn, id, path });
    }

    fn get_metadata(&self, key: &str) -> Result<String, Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let metadata: arg::PropMap = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

        // (<{'xesam:artist': <['milet']>, 'mpris:artUrl': <'file:///home/erasin/.cache/amberol/covers/0946e0166b73dabc1fa60b0a462d7180e52f5c97b6ef5a1cebc765db97ed68a0.png'>, 'xesam:title': <'Who I Am'>, 'mpris:length': <'203000000'>, 'xesam:album': <'Who I Am'>}>,)

        let err_none = Error::Player {
            msg: "没有找到相应的标签".to_owned(),
        };

        match key {
            "xesam:artist" => {
                let strs: Option<&Vec<String>> = arg::prop_cast(&metadata, key);

                if let Some(strs) = strs {
                    Ok(strs.join(","))
                } else {
                    Err(err_none)
                }
            }

            // string
            _ => {
                let str: Option<&String> = arg::prop_cast(&metadata, key);

                if let Some(str) = str {
                    Ok(str.clone())
                } else {
                    Err(err_none)
                }
            }
        }
    }

    fn get_property_string(&self, key: &str) -> Result<String, Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: String = proxy.get("org.mpris.MediaPlayer2.Player", key)?;

        Ok(status)
    }

    fn set_property_string(&self, key: &str, value: &str) -> Result<(), Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        proxy.set("org.mpris.MediaPlayer2.Player", key, value)?;

        Ok(())
    }

    fn action(&self, key: &str) -> Result<(), Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let (): () = proxy.method_call("org.mpris.MediaPlayer2.Player", key, ())?;

        Ok(())
    }

    /// 播放器 id
    pub fn player(&self) -> Result<(), Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        // let id: String = proxy.get("org.mpris.MediaPlayer2", "Identity")?;
        let id: String = proxy.get("org.mpris.MediaPlayer2", "DesktopEntry")?;

        println!("{id}");

        Ok(())
    }

    pub fn next(&self) -> Result<(), Error> {
        self.action("Next")
    }

    pub fn previous(&self) -> Result<(), Error> {
        self.action("Previous")
    }

    pub fn toggle(&self) -> Result<(), Error> {
        self.action("PlayPause")
    }

    pub fn play(&self) -> Result<(), Error> {
        self.action("Play")
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.action("Stop")
    }

    pub fn title(&self) -> Result<(), Error> {
        let title = self.get_metadata("xesam:title")?;
        println!("{title}");
        Ok(())
    }

    pub fn artist(&self) -> Result<(), Error> {
        let artist = self.get_metadata("xesam:artist")?;
        println!("{artist}");
        Ok(())
    }

    pub fn album(&self) -> Result<(), Error> {
        let album = self.get_metadata("xesam:album")?;
        println!("{album}");
        Ok(())
    }

    pub fn status(&self) -> Result<(), Error> {
        let status = self.get_property_string("PlaybackStatus")?;
        println!("{status}");
        Ok(())
    }

    pub fn get_status_icon(&self) -> Result<String, Error> {
        let status = self.get_property_string("PlaybackStatus")?;
        let icon = match status.as_str() {
            "Playing" => "",
            // "Paused"| "Stopped",
            _ => "",
        };
        Ok(icon.to_owned())
    }

    pub fn status_icon(&self) -> Result<(), Error> {
        let icon = self.get_status_icon()?;
        println!("{icon}");

        Ok(())
    }

    pub fn cover(&self) -> Result<(), Error> {
        let cover_url = self.get_metadata("xesam:artUrl")?;
        println!("{cover_url}");
        // TODO write temp file

        Ok(())
    }

    pub fn track_number(&self) -> Result<(), Error> {
        let track_number = self.get_metadata("xesam:trackNumber")?;
        println!("{track_number}");

        Ok(())
    }

    pub fn length(&self) -> Result<(), Error> {
        let length = self.get_metadata("mpris:length")?;
        let duration = length.parse::<u64>().unwrap_or_default();
        print_duration(duration);
        Ok(())
    }

    pub fn lengths(&self) -> Result<(), Error> {
        let length = self.get_metadata("mpris:length")?;
        let duration = length.parse::<u64>().unwrap_or_default();
        println!("{}", duration / 1000000);
        Ok(())
    }

    pub fn get_position(&self) -> Result<u64, Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: i64 = proxy.get("org.mpris.MediaPlayer2.Player", "Position")?;

        Ok(status as u64)
    }

    pub fn position(&self) -> Result<(), Error> {
        let duration = self.get_position()?;
        print_duration(duration);
        Ok(())
    }

    pub fn positions(&self) -> Result<(), Error> {
        let duration = self.get_position()?;
        println!("{}", duration / 1000000);
        Ok(())
    }

    pub fn get_shuffle(&self) -> Result<bool, Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        let status: bool = proxy.get("org.mpris.MediaPlayer2.Player", "Shuffle")?;

        Ok(status)
    }

    pub fn set_shuffle(&self, value: bool) -> Result<(), Error> {
        let proxy = self.conn.with_proxy(
            self.id.clone(),
            self.path.clone(),
            Duration::from_millis(5000),
        );

        proxy.set("org.mpris.MediaPlayer2.Player", "Shuffle", value)?;

        Ok(())
    }

    pub fn shuffle(&self, args: &PlayerShuffleArgs) -> Result<(), Error> {
        if args.on {
            self.set_shuffle(true)?;
        } else if args.off {
            self.set_shuffle(false)?;
        } else if args.toggle {
            let shuffle = self.get_shuffle()?;
            self.set_shuffle(!shuffle)?;
        }

        let shuffle = self.get_shuffle()?;
        let shuffle = match shuffle {
            true => "On",
            false => "Off",
        };
        println!("{shuffle}");

        Ok(())
    }

    pub fn loop_mode(&self, args: &PlayerLoopArgs) -> Result<(), Error> {
        let property_name = "LoopStatus";

        if let Some(mode) = args.mode {
            let mode_set = match mode {
                PlayerLoopMode::None => "None",
                PlayerLoopMode::Playlist => "Playlist",
                PlayerLoopMode::Track => "Track",
            };

            self.set_property_string(property_name, mode_set)?;
        }

        let status = self.get_property_string(property_name)?;

        println!("{status}");

        Ok(())
    }

    // pub fn get_lyrics(&self) -> Result<String, Error> {}
    pub fn lyrics(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn waybar(&self) -> Result<(), Error> {
        let title = self.get_metadata("xesam:title")?;
        let artist = self.get_metadata("xesam:artist")?;
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
