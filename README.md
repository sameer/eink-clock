# eink-clock

A clock for e-ink displays like that in the [Kindle DX Graphite](https://en.wikipedia.org/wiki/Amazon_Kindle#Kindle_DX_Graphite).

## Setup

### Physical requirements

* Raspberry Pi Zero W or other ARM device of your choice with networking capabilities
* [jailbroken Kindle](https://wiki.mobileread.com/wiki/Kindle_Hacks_Information#Jail_break_JB) with usbnetwork


### Software

#### Prerequisites

* [Rust Language](https://www.rust-lang.org/learn/get-started) and [Cargo package manager](https://doc.rust-lang.org/cargo/index.html)

#### Building

##### Install directly on an ARM device (easier but takes longer)

First get the necessary dependencies

###### ArchLinux
```bash
# harfbuzz library, used for text rendering:
pacman -S harfbuzz
# ssh to kindle root without any credentials
pacman -S sshpass
```

Now build.
```bash
# May take a long time
cargo build --release
```

##### Build on your computer (cross-compile, harder but takes less time)

Not working right now

###### ArchLinux

~~Get the cross-compile toolchain:~~
```bash
yay -S arm-bcm2708-linux-gnueabi
```

Build
```bash
HARFBUZZ_SYS_NO_PKG_CONFIG=1 cargo build --release
```


#### Deploying

If you cross-compiled, transfer the project folder to the device.

##### Set up Kindle networking

To network with a Kindle that has usbnetwork enabled and shows up as usb0 in `ip addr`:

Create a static profile for any usb network gadgets at the end of `/etc/dhcpd.conf`:
```
interface usb*
static ip_address=192.168.2.1/24
static routers=192.168.2.1
```

If you are connecting multiple Kindles, you'll need to change the usbnetwork config to assign unique MAC addresses and unique IP addresses.

If you haven't already, enable dhcpcd on startup so the Kindle network interface can be configured on startup and hotplug.
```bash
systemctl enable dhcpcd
# Make sure to disable interface-specific targets like wlan0, if needed
systemctl disable dhcpcd@wlan0
reboot
```

It will hang on the start command if it's set up incorrectly.
Now, the device can network with the Kindle.

On the Kindle, make sure you've enabled auto-start for usbnetwork just in case the Kindle loses power:

```bash
ssh root@192.168.2.2
ls /mnt/us/usbnet/
# If there is a file DISABLED_auto, rename it to auto
# Beware that this means networking will always be enabled at startup
# You cannot connect the Kindle as a USB storage device again until you rename auto to DISABLED_auto
mv /mnt/us/usbnet/DISABLED_auto /mnt/us/usbnet/auto
```

##### Set up systemd files

These are needed to run eink-clock once at the start of every minute.

```bash
ln -s /root/eink-clock/eink-clock.service /etc/systemd/system/
ln -s /root/eink-clock/eink-clock.timer /etc/systemd/system/
systemctl enable eink-clock.timer
systemctl start eink-clock.service
```

And that's it, the clock should now be running! Feel free to [contact me](https://purisa.me/about/) if you have problems.
