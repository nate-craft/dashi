#!/usr/bin/env sh

panic() {
    echo "$1"
    exit 1
}

git clone https://github.com/nate-craft/dashi dashi || panic "Could not clone dashi repository!"

cd dashi || panic "Could not enter dashi repository!"

sudo groupadd -f wheel
sudo usermod -aG wheel "$USER"

cat pkg/90-backlight.rules | sudo tee /etc/udev/rules.d/90-backlight.rules > /dev/null 2>&1 \
    || panic "Could not install backlight udev rule!"
cat pkg/91-leds.rules | sudo tee /etc/udev/rules.d/91-leds.rules > /dev/null 2>&1 \
    || panic "Could not install leds udev rule!"
cat pkg/30-bluetooth.rules | sudo tee /etc/udev/rules.d/30-bluetooth.rules > /dev/null 2>&1 \
    || panic "Could not install bluetooth udev rule!"

cargo install --path . || panic "Could not install dashi!"

cd .. && rm -rf dashi || "Could not cleanup repository. Please manually delete dashi/ folder"

sudo udevadm control --reload
sudo udevadm trigger
sudo systemctl restart polkit

printf "Dashi installed to %s. If brightness/bluetooth is not functional immediately, restart your system\n" "$(command -v dashi)"
