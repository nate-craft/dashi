#!/usr/bin/env sh

DEV=$(ls --color=always /sys/class/backlight/ | head -n 1)
BACKLIGHT_RULE=$(sed "s/DEV/${DEV}/g" pkg/90-backlight.rules)

printf "\nThe dashi installer requires elevated permissions to allow bluetooth and backlight control\n"

sudo groupadd -f wheel
echo "$BACKLIGHT_RULE" | sudo tee /etc/udev/rules.d/90-backlight.rules > /dev/null 2>&1
cat pkg/30-bluetooth.rules | sudo tee /etc/polkit-1/rules.d/30-bluetooth.rules > /dev/null 1>&1

cargo install --git https://github.com/nate-craft/dashi

