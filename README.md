# Dashi

![](https://gist.githubusercontent.com/nate-craft/648bbda6337b503a5d703f86757e4647/raw/144cf1f5f80e9c5ac6b5efde45869d01feb2ccd9/brainmade.png)

Dashi is a simple shell for lightweight window managers on Linux with no background tasks

## Features

- Set, add, subtract, and get backlight brightness

- Set, add, subtract, and get current volume

- Get and toggle microphone and speaker mute status 

- List, add, and remove global bookmarks

- Easily add/remove system notification with the `--silent` flag

___

## Installation

Dashi can be installed from source via [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```sh
cargo install --git https://github.com/nate-craft/dashi
```

Dashi requires [pulseaudio](https://www.freedesktop.org/wiki/Software/PulseAudio/) and optionally any notification library
___

## Integration with Sway

- The following is an example of a Sway configuration that hooks into dashi
- The `--locked` flag is passed to allow functionality with the screen locked
- The `--silent` flag can be passed to disable system notifications
- Keybinds can be changed to suit a given workflow

```sh
bindsym Ctrl+Shift+b exec "dashi bookmark stdout | rofi -dmenu -theme theme | wl-copy "
bindsym $mod+Shift+b exec "wl-paste | xargs -I _ dashi bookmark add _ "
bindsym --locked XF86AudioRaiseVolume exec "dashi volume add 5"
bindsym --locked XF86AudioLowerVolume exec "dashi volume sub 5"
bindsym --locked XF86AudioMute exec "dashi volume mute"
bindsym --locked XF86AudioMicMute exec "dashi volume mute-mic"
bindsym --locked XF86MonBrightnessUp exec "dashi brightness add 5"
bindsym --locked XF86MonBrightnessDown exec "dashi brightness sub 5"
````
