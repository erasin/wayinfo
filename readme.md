# wayinfo

一组用于获取系统信息，天气，播放器控制的命令工具。

主要用于在 wayland 系统下提供数据给 `waybar`, `eww` 。

> 功能开发中

## 功能 

wayinfo 

- [ ] system
  - [ ] cpu
  - [ ] gpu
  - [ ] memory
  - [ ] network
  - [ ] disk
  - [ ] unmount
- [x] weather 使用高德天气API
- [x] player
  - [x] player        Player Identity
  - [x] next          next song
  - [x] previous      previous song
  - [x] toggle        toggle play or pause
  - [x] play          play
  - [x] stop          stop
  - [x] status        Playback status (Playing|Paused|Stopped)
  - [x] status-icon   Playing   , other 
  - [x] title         title of song
  - [x] artist        artist of song
  - [x] album         album of song
  - [x] cover         cover of song
  - [x] track-number  Track Number of
  - [x] position      Position time at playing
  - [x] positions     Position second at playing
  - [x] length        Length of song
  - [x] lengths       Length second of song
  - [x] shuffle
  - [x] loop          循环模式
  - [x] lyrics        lyrics
  - [x] waybar        waybar format

## system

## weather

天气使用高德天气API，需要 key.

```sh
wayinfo weather --waybar --city 上海 --key-file $HOME/.config/apikeys/gaode.txt
```

## player

播放器控制使用 `playerctld` 支持 `MPRIS` 协议播放器。 

> 如果使用 `mpd` 可以安装 `mpd-mpris` 服务以获得支持。




