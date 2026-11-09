# Source: https://micstream.io/guides/use-phone-as-mic-linux

Use Your Phone as a Microphone on Linux — Micstream

[Skip to main content](#main-content) 

[![Micstream logo](/logo/logo_light.svg)![Micstream logo](/logo/logo_dark.svg)Micstream](/) [FEATURES](../#how-it-works) [GUIDES](../guides) [DOWNLOAD](../download?src=header) [FAQ](../#faq)

[Home](../) [Guides](../guides)

Turn your phone into a Linux microphone
=======================================

Updated September 23, 2026 · 6 min read

In short

* Install the Micstream client on your Linux PC
* Install the app on your phone and connect
* Select the new microphone called "Micstream"

Any app that lets you pick a microphone can use it, like Discord, Zoom or OBS.

[Micstream](../) is a small background service. It adds a virtual microphone to
PipeWire, the audio system of all current Linux distributions.

You need three things:

* A distribution with [PipeWire](https://www.pipewire.org) (Ubuntu 22.10+/Fedora 36+/Debian 12+/Arch)
* An Android phone or iPhone
* Both devices on the same Wi-Fi network

Setup
-----

1. 1

   ### Download Micstream

   Download the package for your distribution:

   * [micstream-amd64.deb](/downloads/micstream-amd64.deb) — Debian, Ubuntu, Mint
   * [micstream-x86\_64.rpm](/downloads/micstream-x86_64.rpm) — Fedora
   * [micstream-linux-x64.tar.gz](/downloads/micstream-linux-x64.tar.gz) — Arch and any other distro
2. 2

   ### Install the Linux client

   Debian/Ubuntu/Mint:

   ```
   sudo apt install ~/Downloads/micstream-amd64.deb
   ```

   Fedora:

   ```
   sudo dnf install ~/Downloads/micstream-x86_64.rpm
   ```

     

   On Arch and other distributions, use the tarball. It contains `install.sh`, which
   installs Micstream in your home folder without root. On Arch, PipeWire's audio plugins are a
   separate package, so install it first:

   ```
   sudo pacman -S --needed pipewire-audio
   ```
3. 3

   ### Start the service

   Turn on the service. It then also starts every time you log in:

   ```
   systemctl --user enable --now micstream
   ```

   If you installed from the tarball, `install.sh` has already done this.
4. 4

   ### Install the phone app

   Get it on [Google Play](https://play.google.com/store/apps/details?id=com.totech.phoneasmic&referrer=utm_source%3Dwebsite%26utm_medium%3Dweb%26utm_campaign%3Dwebsite%26utm_content%3Dguide-linux) or the [App Store](https://apps.apple.com/app/id6791630989?pt=129174575&ct=guide-linux&mt=8). You don't
   need an account.
5. 5

   ### Connect the phone

   Open the app. It finds your PC by itself if both are on the same network. On iPhone, allow
   Local Network access when asked. Without it, the app can't see your PC.
6. 6

   ### Pick Micstream as the input

   Select **Micstream** in your app's microphone settings. To use it everywhere, make
   it the default input (GNOME: Settings → Sound → Input; KDE: System Settings → Sound). Every app
   sees it as a normal microphone.

Tip

Unlike on Windows, there is no driver. The client runs as a normal service under your user, not
as root. Uninstalling removes it completely.

Discord and OBS ignore the system default and use their own setting, so select Micstream there.
There are separate guides for [Discord](../guides/phone-mic-for-discord) and [OBS](../guides/phone-mic-for-obs).

Noise cancellation
------------------

The client comes with [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet), an AI noise-cancellation model. It's on by default. To turn it off:

```
micstream set noise-suppression=off
```

The change works right away and stays after a reboot. The louder the noise, the more it removes,
up to 15 dB by default. If you still hear noise, raise the limit:

```
micstream set attenuation-limit=20
```

Processing runs on the computer, not on the phone, so it does not affect the phone's battery.

Configuration
-------------

The settings are in `~/.config/micstream/micstream.conf`. Each line is one `KEY=value` pair. Everything after a `#` is a comment.

After editing the file, restart the service:

```
systemctl --user restart micstream
```

Only `noise-suppression` and `attenuation-limit` can change live with `micstream set`. Other settings need a restart.

| Key | Default | What it does |
| --- | --- | --- |
| `noise-suppression` | `on` | DeepFilterNet noise cancellation, `on` or `off`. |
| `attenuation-limit` | `15` | How many dB of noise the suppression may remove at most (10–25). |
| `noise-model` | `standard` | `low-latency` switches to a faster model, which isn't bundled: put `DeepFilterNet3_ll_onnx.tar.gz` into `~/.local/share/micstream`. |
| `update-check` | `on` | Checks once a day for a new release and shows a notification. Updates are never installed automatically. |
| `telemetry` | `on` | Sends reliability statistics (packet counters, never audio) under a random ID. `off` turns it off. |

Everyday use
------------

The client has no window and no tray icon. It runs in the background. To check its state:

```
micstream status
```

```
micstream 1.0.0
phone:       connected (192.168.0.42)
virtual mic: live — an app is capturing
noise:       active (standard model, limit 15 dB)
buffer:      61 ms buffered, 0 underruns, 0 trims
session:     84210 packets, 3 late, 2 concealed, 0 decode errors
```

It shows whether a phone is connected, whether an app is using the microphone, and what noise
cancellation is doing. You also get a desktop notification when the phone connects or
disconnects. The only other commands are `--version` and `--help`.

The service
-----------

The client is a normal systemd user service, so the usual commands work:

```
# restart (needed after hand-editing the config file)
systemctl --user restart micstream

# stop, or stop and keep it off after reboots
systemctl --user stop micstream
systemctl --user disable --now micstream

# follow the logs
journalctl --user -u micstream -f
```

### Updates

The client doesn't update itself. When a new version comes out, you get a desktop notification.
Download the new package below and install it like the first time: `sudo apt install ~/Downloads/micstream-amd64.deb` (or the rpm) updates it. Then run `systemctl --user restart micstream` to start the new version. The tarball's `install.sh` restarts the service for you.

Troubleshooting
---------------

### Micstream isn't in the input device list

Check that the service is running: `systemctl --user status micstream`.

* If it says PipeWire is unreachable, your system probably still uses PulseAudio. Micstream
  needs PipeWire (the default since Ubuntu 22.10, Fedora 36 and Debian 12).
* If the log shows `pw_stream_connect failed`, PipeWire's audio plugins are
  missing. On Arch, install the `pipewire-audio` package.
* If Micstream is there but an app still uses another mic, select Micstream in that app and
  restart the app. Many apps only look for new devices when they start.

### The app can't find your computer

* Make sure the phone is on the same Wi-Fi as the computer, not on mobile data.
* Don't use a guest network. Guest networks often stop devices from seeing each other.
* If you use a firewall, allow UDP ports 37718–37720. With ufw: `sudo ufw allow 37718:37720/udp`. With firewalld, add the ports to your active
  zone.

### Audio is choppy or delayed

This is almost always the network, not the microphone. Move closer to the router, or connect
the computer with an Ethernet cable. The client keeps the delay low by itself: it hides
short dropouts and catches up automatically after the network was slow.

Common questions
----------------

### Is it free?

Yes, with ads: each short ad unlocks an hour of free access. The hour counts down whether or not
you're streaming. A Pro subscription removes the ads and the time limit.

### Which distributions are supported?

Any distribution with PipeWire and systemd, on x86-64: Ubuntu, Debian, Fedora, Mint, Arch and
their derivatives.

### What is the audio quality?

PipeWire receives the audio as 32-bit float at 48 kHz, in stereo. If network packets get lost,
the codec's error correction hides the gaps.

### Does it have noise suppression?

Yes. DeepFilterNet is included in every package and on by default. See [Noise cancellation](#noise-cancellation) above.

### Can I use the phone while it's streaming?

Yes. The app runs in the background.

Downloads
---------

* [micstream-amd64.deb](/downloads/micstream-amd64.deb) Debian, Ubuntu, Mint
* [micstream-x86\_64.rpm](/downloads/micstream-x86_64.rpm) Fedora
* [micstream-linux-x64.tar.gz](/downloads/micstream-linux-x64.tar.gz) Arch & any distro, installs to your home directory
* [SHA-256 checksums](/downloads/micstream-linux-sha256sums.txt)

And the app:

[Download Windows Client](/downloads/MicstreamSetup.exe) [Get Android App](https://play.google.com/store/apps/details?id=com.totech.phoneasmic&referrer=utm_source%3Dwebsite%26utm_medium%3Dweb%26utm_campaign%3Dwebsite%26utm_content%3Dguide-linux)

© 2026 Micstream. All rights reserved.

[Guides](../guides) • [Press Kit](/press) • [Privacy Policy](/privacy) • [Terms of Service](/terms)