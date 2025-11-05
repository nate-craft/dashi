#!/usr/bin/env sh

sudo groupadd -f wheel
sudo usermod -aG wheel "$USER"
cat pkg/90-backlight.rules | sudo tee /etc/udev/rules.d/90-backlight.rules > /dev/null 2>&1
cat pkg/30-bluetooth.rules | sudo tee /etc/polkit-1/rules.d/30-bluetooth.rules > /dev/null 1>&1

cargo install --git https://github.com/nate-craft/dashi

