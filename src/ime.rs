#[cfg(target_os = "linux")]
mod fcitx;

#[cfg(target_os = "linux")]
mod ibus;

#[cfg(target_os = "macos")]
mod macime;
