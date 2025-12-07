# Dashi

![](https://gist.githubusercontent.com/nate-craft/648bbda6337b503a5d703f86757e4647/raw/144cf1f5f80e9c5ac6b5efde45869d01feb2ccd9/brainmade.png)

Dashi is a simple shell for lightweight window managers on Linux with minimal resources

## Features

- Control backlight brightness

- Control audio output and input

- Control microphone and speaker mute status 

- List, add, and remove global bookmarks

- Control bluetooth connectivity systemd

- Control and monitor battery power and AC connections with an optional notification daemon

- Easily add/remove system notification with the `--silent` flag

- Control automatic nightshift (currently via gammastep)

___

## Installation

[Recommended] Dashi can most easilly be installed via the installer script:
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://raw.githubusercontent.com/nate-craft/dashi/refs/heads/main/installer.sh | sh
```

Dashi can also be installed via the AUR:
```
paru -Syu dashi
```
> Note: if AUR is used for installation. Manual polkit/udev instructions must still be followed

Dashi can be manually built with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```sh
git clone https://github.com/nate-craft/dashi

# Permissions for bluetooth and backlight control
sudo groupadd -f wheel
sudo usermod -aG wheel "$USER"
cat pkg/90-backlight.rules | sudo tee /etc/udev/rules.d/90-backlight.rules > /dev/null 2>&1
cat pkg/30-bluetooth.rules | sudo tee /etc/polkit-1/rules.d/30-bluetooth.rules > /dev/null 1>&1

# Building the dashi binary
cargo install --git https://github.com/nate-craft/dashi

# Reload udev and polkit. May require a restart for full functionality
sudo udevadm control --reload
sudo udevadm trigger
sudo systemctl restart polkit

```

## Dependencies

Dashi has no build time dependencies. For some features, runtime dependencies are necessary:

- Audio Control: [pulseaudio](https://www.freedesktop.org/wiki/Software/PulseAudio/)
- Notifications (Optional): any notification daemon
- Nightshift: [gammastep](https://gitlab.com/chinstrap/gammastep)
  - This may be removed in the future in favor of a native solution

___

## Integration with Sway

- The following is an example of a Sway configuration that hooks into dashi
- The `--locked` flag is passed to allow functionality with the screen locked
- The `--silent` flag can be passed to disable system notifications
- Keybinds can be changed to suit a given workflow
- exec can be used for commands to run on startup

```sh
bindsym Ctrl+Shift+b exec "dashi bookmark stdout | rofi -dmenu | wl-copy "
bindsym $mod+Shift+b exec "wl-paste | xargs -I _ dashi bookmark add _ "
bindsym --locked XF86AudioRaiseVolume exec "dashi volume add 5"
bindsym --locked XF86AudioLowerVolume exec "dashi volume sub 5"
bindsym --locked XF86AudioMute exec "dashi volume mute"
bindsym --locked XF86AudioMicMute exec "dashi volume mute-mic"
bindsym --locked XF86MonBrightnessUp exec "dashi brightness add 5"
bindsym --locked XF86MonBrightnessDown exec "dashi brightness sub 5"
bindsym --locked XF86Bluetooth exec "dashi bluetooth toggle"

exec "dashi power daemon"
exec "dashi nightshift start"
```
