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

Then start the script.