# Geekworm NasPI Control

Simple program to control the Fan speed of the NasPI based on Temp.

**TODO**

- Add config file
- Add Safe shutdown from HW Button
- Add service file

## Setup
The script uses the PWM Channel of the PI. Make sure to enable it accordingly.

``` bash
# /boot/config.txt

...
# Enable PWM
dtoverlay=pwm
...
```

# Build Debian Package

You can build a Debian package with `cargo-deb`.

**Installing `cargo-deb` globally**

``` bash
    cargo install cargo-deb
```

**Build and install with `cargo-deb`**

``` bash
    cargo-deb --install
```
The Debian package includes a systemd service unit. After installation check if the service `naspi-control` is enabled and started.

``` bash
    systemctl status naspi-control
```
